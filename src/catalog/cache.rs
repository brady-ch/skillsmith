use std::collections::HashMap;
use std::fs;
use std::path::Path;

use crate::error::AppError;

use super::types::{Catalog, CatalogCache, ReferenceIndex};

impl CatalogCache {
    pub fn new(catalog: Catalog) -> Self {
        Self {
            catalog,
            reference_indexes: HashMap::new(),
        }
    }

    pub fn catalog(&self) -> &Catalog {
        &self.catalog
    }

    pub fn load_reference_index(
        &mut self,
        repo_root: &Path,
        relative_skill_path: &str,
    ) -> Result<ReferenceIndex, AppError> {
        let path = repo_root
            .join(relative_skill_path)
            .join("references")
            .join("index.toml");
        if let Some(index) = self.reference_indexes.get(&path) {
            return Ok(index.clone());
        }
        let content = fs::read_to_string(&path)?;
        let index: ReferenceIndex = toml::from_str(&content)?;
        index.validate(&path)?;
        self.reference_indexes.insert(path, index.clone());
        Ok(index)
    }
}
