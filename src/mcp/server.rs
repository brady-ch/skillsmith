//! MCP stdio server exposing thin catalog tools.

use std::path::{Path, PathBuf};
use std::sync::Arc;

use rmcp::{
    ServerHandler, ServiceExt,
    handler::server::{router::tool::ToolRouter, wrapper::Parameters},
    model::{ServerCapabilities, ServerInfo},
    schemars, tool, tool_handler, tool_router,
};
use serde::Deserialize;

use crate::catalog::repo_paths::{
    catalog_skill_roots, is_under_catalog_skill, join_repo_relative, read_utf8_capped,
};
use crate::catalog::{Catalog, CatalogCache, explain_skill_selection, recommend_for_intent};
use crate::error::AppError;

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub(crate) struct RecommendParams {
    #[schemars(description = "User task synopsis.")]
    pub intent: String,
    #[serde(default = "default_limit")]
    pub limit: usize,
    pub skill: Option<String>,
    pub source: Option<String>,
    #[serde(default)]
    pub include_deprecated: bool,
}

fn default_limit() -> usize {
    5
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub(crate) struct ExplainParams {
    pub intent: Option<String>,
    pub skill: Option<String>,
    pub source: Option<String>,
    #[serde(default)]
    pub include_deprecated: bool,
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
        description = "Rank catalog skills plus one suggested references/ file each (RecommendResponse schema_version 1 JSON)."
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
        description = "Explain one resolved skill/reference for an intent or explicit skill (ExplainMatch JSON)."
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
        description = "Read a UTF-8 text file under a catalog skill directory (truncated beyond max_bytes)."
    )]
    async fn fetch_tool(&self, Parameters(params): Parameters<FetchParams>) -> String {
        Self::exec_fetch(
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
                params.include_deprecated,
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
                params.include_deprecated,
            )?;
            Ok(serde_json::to_string(&explain)?)
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
            "skillsmith routing: call skillsmith_recommend with the user task, then skillsmith_fetch_file for SKILL.md and one references/ slice. Keep responses terse.",
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
