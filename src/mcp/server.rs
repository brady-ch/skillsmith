//! MCP stdio server exposing thin catalog tools.

use std::path::{Path, PathBuf};
use std::sync::Arc;

use rmcp::{
    ServerHandler, ServiceExt,
    handler::server::{router::tool::ToolRouter, wrapper::Parameters},
    model::{ServerCapabilities, ServerInfo},
    schemars, tool, tool_handler, tool_router,
};
use serde::{Deserialize, Serialize};

use crate::catalog::repo_paths::{
    catalog_skill_roots, is_under_catalog_skill, join_repo_relative, read_utf8_capped,
};
use crate::catalog::{Catalog, CatalogCache, explain_skill_selection, recommend_for_intent};
use crate::error::AppError;

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub(crate) struct RecommendParams {
    #[schemars(
        description = "Short task synopsis; prefer under ~200 characters to keep tool context small."
    )]
    pub intent: String,
    #[serde(default = "default_limit")]
    #[schemars(description = "Max skills in the response; prefer 3–5 for token budget.")]
    pub limit: usize,
    pub skill: Option<String>,
    pub source: Option<String>,
}

fn default_limit() -> usize {
    5
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub(crate) struct ExplainParams {
    pub intent: Option<String>,
    pub skill: Option<String>,
    pub source: Option<String>,
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub(crate) struct FetchParams {
    #[schemars(
        description = "Relative path beneath the skillsmith repo (e.g. skills/foo/SKILL.md)."
    )]
    pub path: String,
    #[serde(default = "default_max_bytes")]
    pub max_bytes: usize,
}

fn default_max_bytes() -> usize {
    65_536
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub(crate) struct RouteTraceParams {
    #[schemars(
        description = "Short task synopsis; prefer under ~200 characters to keep tool context small."
    )]
    pub intent: String,
    #[serde(default = "default_trace_limit")]
    #[schemars(description = "How many ranked skills to include (paths only; default 3).")]
    pub limit: usize,
    pub skill: Option<String>,
    pub source: Option<String>,
}

fn default_trace_limit() -> usize {
    3
}

#[derive(Debug, Serialize)]
struct RouteTraceResponse {
    schema_version: u32,
    intent: String,
    traces: Vec<RouteTraceEntry>,
}

#[derive(Debug, Serialize)]
struct RouteTraceEntry {
    skill_name: String,
    source: Option<String>,
    skill_path: String,
    skill_md: String,
    reference_router: String,
    suggested_reference_file: String,
    suggested_reference: String,
}

#[derive(Clone)]
pub struct SkillsmithMcpService {
    tool_router: ToolRouter<Self>,
    repo_root: Arc<PathBuf>,
    catalog_path: Arc<PathBuf>,
}

impl SkillsmithMcpService {
    pub fn new(repo_root: PathBuf, catalog_path: PathBuf) -> Self {
        Self {
            tool_router: Self::tool_router(),
            repo_root: Arc::new(repo_root),
            catalog_path: Arc::new(catalog_path),
        }
    }
}

#[tool_router]
impl SkillsmithMcpService {
    #[tool(
        name = "skillsmith_recommend",
        description = "Rank catalog skills with one suggested references/ file each (schema_version 1 JSON). Call first with a short intent; use limit 3–5; then fetch_file only the paths you need (SKILL.md, router, one reference)."
    )]
    async fn recommend_tool(&self, Parameters(params): Parameters<RecommendParams>) -> String {
        Self::exec_recommend(
            self.repo_root.as_path(),
            self.catalog_path.as_path(),
            params,
        )
    }

    #[tool(
        name = "skillsmith_explain",
        description = "Explain one resolved skill and reference (ExplainMatch JSON). Use when you need match reasons or a single deep path; otherwise prefer recommend + fetch_file."
    )]
    async fn explain_tool(&self, Parameters(params): Parameters<ExplainParams>) -> String {
        Self::exec_explain(
            self.repo_root.as_path(),
            self.catalog_path.as_path(),
            params,
        )
    }

    #[tool(
        name = "skillsmith_fetch_file",
        description = "Read one UTF-8 file under a catalog skill directory (default max_bytes=65536, truncates with suffix). Fetch SKILL.md, then reference-router.md, then exactly one references/ slice from recommend—do not bulk-load every reference."
    )]
    async fn fetch_tool(&self, Parameters(params): Parameters<FetchParams>) -> String {
        Self::exec_fetch(
            self.repo_root.as_path(),
            self.catalog_path.as_path(),
            params,
        )
    }

    #[tool(
        name = "skillsmith_route_trace",
        description = "Path-only preview of recommend results (schema_version 1 JSON): repo-relative paths for SKILL.md, reference-router.md, and the suggested reference—no file bodies or match reason arrays. Smallest tool output when you only need load order."
    )]
    async fn route_trace_tool(&self, Parameters(params): Parameters<RouteTraceParams>) -> String {
        Self::exec_route_trace(
            self.repo_root.as_path(),
            self.catalog_path.as_path(),
            params,
        )
    }
}

impl SkillsmithMcpService {
    fn exec_recommend(repo_root: &Path, catalog_path: &Path, params: RecommendParams) -> String {
        (|| -> Result<String, AppError> {
            let catalog = Catalog::load_from_file(catalog_path)?;
            let mut cache = CatalogCache::new(catalog);
            let resp = recommend_for_intent(
                &mut cache,
                repo_root,
                &params.intent,
                params.limit,
                params.skill.as_deref(),
                params.source.as_deref(),
            )?;
            Ok(serde_json::to_string(&resp)?)
        })()
        .unwrap_or_else(|err| serde_json::json!({ "error": err.to_string() }).to_string())
    }

    fn exec_explain(repo_root: &Path, catalog_path: &Path, params: ExplainParams) -> String {
        (|| -> Result<String, AppError> {
            if params.intent.is_none() && params.skill.is_none() {
                return Err(AppError::InputError(
                    "skillsmith_explain requires intent or skill".into(),
                ));
            }
            let catalog = Catalog::load_from_file(catalog_path)?;
            let mut cache = CatalogCache::new(catalog);
            let explain = explain_skill_selection(
                &mut cache,
                repo_root,
                params.skill.as_deref(),
                params.intent.as_deref(),
                params.source.as_deref(),
            )?;
            Ok(serde_json::to_string(&explain)?)
        })()
        .unwrap_or_else(|err| serde_json::json!({ "error": err.to_string() }).to_string())
    }

    fn exec_route_trace(repo_root: &Path, catalog_path: &Path, params: RouteTraceParams) -> String {
        (|| -> Result<String, AppError> {
            let catalog = Catalog::load_from_file(catalog_path)?;
            let mut cache = CatalogCache::new(catalog);
            let resp = recommend_for_intent(
                &mut cache,
                repo_root,
                &params.intent,
                params.limit,
                params.skill.as_deref(),
                params.source.as_deref(),
            )?;
            let intent = resp.intent;
            let traces: Vec<RouteTraceEntry> = resp
                .recommendations
                .into_iter()
                .map(|e| {
                    let skill_path = e.skill_path.trim_end_matches('/').to_string();
                    let suggested_reference_file = e.suggested_reference_file;
                    let suggested_reference =
                        format!("{skill_path}/references/{suggested_reference_file}");
                    RouteTraceEntry {
                        skill_name: e.skill_name,
                        source: e.source,
                        skill_path: skill_path.clone(),
                        skill_md: format!("{skill_path}/SKILL.md"),
                        reference_router: format!("{skill_path}/references/reference-router.md"),
                        suggested_reference_file,
                        suggested_reference,
                    }
                })
                .collect();
            let out = RouteTraceResponse {
                schema_version: 1,
                intent,
                traces,
            };
            Ok(serde_json::to_string(&out)?)
        })()
        .unwrap_or_else(|err| serde_json::json!({ "error": err.to_string() }).to_string())
    }

    fn exec_fetch(repo_root: &Path, catalog_path: &Path, params: FetchParams) -> String {
        (|| -> Result<String, AppError> {
            let catalog = Catalog::load_from_file(catalog_path)?;
            let roots = catalog_skill_roots(&catalog, repo_root);
            let abs = join_repo_relative(repo_root, &params.path)?;
            if !abs.is_file() {
                return Err(AppError::InputError(format!(
                    "not a file: {}",
                    abs.display()
                )));
            }
            if !is_under_catalog_skill(&abs, &roots) {
                return Err(AppError::InputError(
                    "path is not under a catalog skill root".into(),
                ));
            }
            let text = read_utf8_capped(&abs, params.max_bytes)?;
            Ok(serde_json::json!({ "path": params.path, "content": text }).to_string())
        })()
        .unwrap_or_else(|err| serde_json::json!({ "error": err.to_string() }).to_string())
    }
}

#[tool_handler(router = self.tool_router)]
impl ServerHandler for SkillsmithMcpService {
    fn get_info(&self) -> ServerInfo {
        ServerInfo::new(ServerCapabilities::builder().enable_tools().build()).with_instructions(
            "Token-first skillsmith: (1) skillsmith_route_trace or skillsmith_recommend with a short intent and small limit; (2) skillsmith_fetch_file for SKILL.md, reference-router.md, then one references/ file from the result—never load all references. Keep assistant replies terse.",
        )
    }
}

pub fn serve_stdio(repo_root: PathBuf, catalog_path: PathBuf) -> Result<(), AppError> {
    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .map_err(|e| AppError::CommandError(format!("tokio runtime: {e}")))?
        .block_on(async move {
            let service = SkillsmithMcpService::new(repo_root, catalog_path);
            let transport = rmcp::transport::stdio();
            let running = service
                .serve(transport)
                .await
                .map_err(|e| AppError::CommandError(format!("mcp serve: {e}")))?;
            running
                .waiting()
                .await
                .map_err(|e| AppError::CommandError(format!("mcp join: {e}")))?;
            Ok(())
        })
}
