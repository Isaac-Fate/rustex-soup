use std::{
    fs::read_to_string,
    path::{Path, PathBuf}
};
use anyhow::Result;
use lazy_static::lazy_static;

lazy_static! {
    static ref TESTS_DIR: PathBuf = std::env::current_dir()
        .unwrap()
        .join("tests");
    static ref TEX_DIR: PathBuf = TESTS_DIR.join("tex");
    static ref SAMPLE_TEX_FILE_PATH: PathBuf = TEX_DIR
        .join("sample")
        .with_extension("tex");
}

fn read_tex() -> Result<String> {
    Ok(read_to_string(SAMPLE_TEX_FILE_PATH.as_path())?)
}

#[test]
fn print_tex_content() -> Result<()> {
    let tex = read_tex()?;
    println!("{}", tex);
    Ok(())
}

