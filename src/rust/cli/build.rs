use std::path::Path;

fn main() {
    let icon = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../../images/icon.ico");
    println!("cargo:rerun-if-changed={}", icon.display());

    // Only PE executables carry an icon resource; check the target, not the host, so cross-compiles work.
    // The icon lives outside the crate, so builds from the published package skip it.
    if std::env::var("CARGO_CFG_TARGET_OS").as_deref() == Ok("windows") && icon.exists() {
        winresource::WindowsResource::new()
            .set_icon(icon.to_str().expect("icon path is not valid UTF-8"))
            .compile()
            .expect("failed to embed Windows resources");
    }
}
