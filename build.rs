#[cfg(target_os = "windows")]
fn main() {
    use embed_manifest::{embed_manifest, new_manifest};
    use std::io::Write;

    // if Ok("release".to_owned()) != std::env::var("PROFILE") {
    //     return;
    // }

    let mut res = winres::WindowsResource::new();

    res.set_icon_with_id("./assets/app-icon.ico", "app-icon.ico");
    match res.compile() {
        Err(e) => {
            write!(std::io::stderr(), "{}", e).unwrap();
            std::process::exit(1);
        }
        Ok(_) => {}
    }

    // test
    if std::env::var_os("CARGO_CFG_WINDOWS").is_some() {
        embed_manifest(new_manifest("My Company")).expect("unable to embed manifest file");
    }

    println!("cargo:rerun-if-changed=build.rs");
}

// nothing to do for other operating systems
#[cfg(not(target_os = "windows"))]
fn main() {}
