use std::path::Path;

fn copy_dir(src: impl AsRef<Path>, dst: impl AsRef<Path>) -> std::io::Result<()> {
    std::fs::create_dir_all(&dst)?;
    for entry in std::fs::read_dir(src)? {
        let entry = entry?;
        if entry.file_type()?.is_dir() {
            copy_dir(entry.path(), dst.as_ref().join(entry.file_name()))?;
        } else {
            std::fs::copy(entry.path(), dst.as_ref().join(entry.file_name()))?;
        }
    }
    Ok(())
}

fn main() {
    let use_local_scenes = std::env::var("LISPERS_USE_LOCAL_SCENES").unwrap_or_default() == "1";
    let no_copy = std::env::var("LISPERS_DONT_COPY_SCENES").unwrap_or_default() == "1";

    let out_dir = match std::env::var("LISPERS_OUT_DIR") {
        Ok(val) => val,
        Err(_) => std::env::var("OUT_DIR").unwrap(),
    };

    let mut scenes_dir = Path::new(&std::env::var("CARGO_MANIFEST_DIR").unwrap())
        .canonicalize()
        .unwrap()
        .join("scenes");

    if !use_local_scenes {
        let tgt_scenes_dir = Path::new(&out_dir).join("scenes");
        if !no_copy {
            copy_dir(&scenes_dir, &tgt_scenes_dir).expect("Failed to copy scenes directory");
        }
        scenes_dir = tgt_scenes_dir;
    }

    println!("cargo:rustc-env=SCENES_DIR={}", scenes_dir.display());
}
