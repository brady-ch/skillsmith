mod hooks_install;
mod paths;
mod state;
mod wizard;

pub use hooks_install::{install_cursor_hooks, install_project_agent_rules};
pub use paths::resolve_catalog_paths;
pub use wizard::{run_setup, run_setup_update};
