use std::{env, path::Path, process::Command};

fn is_release() -> bool {
    let profile = env::var("PROFILE").expect("Could not determine profile.");
    match profile.as_str() {
        "release" => true,
        _ => false,
    }
}

fn main() {
    if is_release() {
        let parent = Path::new(
            &env::var("CARGO_MANIFEST_DIR").expect("Could not determine manifest directory."),
        )
        .join("soash-client");
        Command::new("npm")
            .args(vec!["run", "build"])
            .current_dir(&parent)
            .output()
            .expect("Could not build frontend.");
    }
}
