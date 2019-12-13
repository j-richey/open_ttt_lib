#[test]
fn readme_deps_version_number_should_match_cargo_toml() {
    version_sync::assert_markdown_deps_updated!("README.md");
}

#[test]
fn html_root_url_version_number_should_match_cargo_toml() {
    version_sync::assert_html_root_url_updated!("src/lib.rs");
}
