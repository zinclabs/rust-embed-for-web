use rust_embed_for_web::RustEmbed;

#[derive(RustEmbed)]
#[folder = "examples/public/"]
struct Assets;

/// Prevent attempts to access files outside of the embedded folder.
/// This is mainly a concern when running in debug mode, since that loads from
/// the file system at runtime.
#[test]
fn path_traversal_attack_fails() {
    assert!(Assets::get("../../Cargo.toml").is_none());
}
