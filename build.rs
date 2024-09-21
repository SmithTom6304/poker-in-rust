use anyhow::*;
use fs_extra::copy_items;
use fs_extra::dir::CopyOptions;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::{env, fs};

fn main() -> Result<()> {
    let handranks_data = PathBuf::from("two-plus-two-hand-evaluator/HandRanks.dat");

    if !handranks_data.exists() {
        let make_command = Command::new("make")
            .current_dir(
                handranks_data
                    .parent()
                    .expect("Could not find handranks data parent directory"),
            )
            .output();
        if let Err(make_error) = make_command {
            if let std::io::ErrorKind::NotFound = make_error.kind() { 
                return Err(anyhow!("make not found. Check your path, or install make."));
            } else {
                return Err(anyhow!("Unexpected error occured making HandRanks.Dat: {}", make_error));
            }
        }
    }

    let out_dir = get_output_path();
    if !&out_dir.exists() {
        fs::create_dir(&out_dir).expect("Failed creating directory");
    }
    let mut copy_options = CopyOptions::new();
    copy_options.overwrite = true;
    let paths_to_copy = vec![handranks_data];
    copy_items(&paths_to_copy, out_dir, &copy_options)?;

    Ok(())
}

fn get_output_path() -> PathBuf {
    // See https://stackoverflow.com/a/67516503
    //<root or manifest path>/target/<profile>/
    let manifest_dir_string = env::var("CARGO_MANIFEST_DIR").unwrap();
    let build_type = env::var("PROFILE").unwrap();
    let path = Path::new(&manifest_dir_string)
        .join("target")
        .join(build_type);
    path
}
