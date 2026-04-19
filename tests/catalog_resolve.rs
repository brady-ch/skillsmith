use skillsmith::setup::resolve_catalog_paths;

/// `cargo test` runs with cwd at the package root, where catalog/catalog.toml exists.
#[test]
fn resolve_finds_catalog_in_package_root() {
    let (root, catalog) = resolve_catalog_paths().expect("catalog should resolve");
    assert!(catalog.is_file());
    assert_eq!(
        catalog.file_name().and_then(|n| n.to_str()),
        Some("catalog.toml")
    );
    assert!(root.join("catalog/catalog.toml").is_file());
}
