extern crate failure;
extern crate sass_rs;
extern crate url;
extern crate webapp;

use failure::Fallible;
use sass_rs::{compile_file, Options, OutputStyle};
use std::{
    env,
    fs::{copy, write},
    path::{Path, PathBuf},
    process::Command,
};
use url::Url;
use webapp::{config::Config, CONFIG_FILENAME};

const REPOSITORY: &str = "https://github.com/uikit/uikit";
const TAG: &str = "v3.0.0-rc.11";
const CSS_FILE: &str = "style.css";
const SCSS_FILE: &str = "style.scss";

pub fn main() -> Fallible<()> {
    // Prepeare UIkit and build the complete style
    prepare_style()?;

    // Prepare the API URL paths
    prepare_api()?;

    Ok(())
}

fn run<F>(name: &str, mut configure: F) -> Fallible<()>
where
    F: FnMut(&mut Command) -> &mut Command,
{
    let mut command = Command::new(name);
    let configured = configure(&mut command);
    if !configured.status()?.success() {
        panic!("failed to execute {:?}", configured);
    }
    Ok(())
}

fn prepare_style() -> Fallible<()> {
    // Prepare the directory
    let out_dir = env::var("OUT_DIR")?;
    let mut target = PathBuf::from(out_dir);
    target.push("uikit");

    // Clone the repo if needed
    if !Path::new(&target).exists() {
        run("git", |command| {
            command
                .arg("clone")
                .arg(format!("--branch={}", TAG))
                .arg("--recursive")
                .arg(REPOSITORY)
                .arg(&target)
        })?;
    }

    // Copy the scss file into the output directory
    target.pop();
    target.push(SCSS_FILE);
    copy(format!("src/{}", SCSS_FILE), &target)?;

    // Build the file
    let mut options = Options::default();
    options.output_style = OutputStyle::Compressed;
    match compile_file(&target, options) {
        Err(error) => panic!(error),
        Ok(content) => {
            // Copy the file into the static directory
            target.pop();
            target.push(CSS_FILE);
            write(&target, content)?;
            copy(&target, format!("static/css/{}", CSS_FILE))?;
        }
    }

    Ok(())
}

fn prepare_api() -> Fallible<()> {
    let config = Config::new(&format!("../{}", CONFIG_FILENAME))?;
    let url = Url::parse(&config.server.url)?;
    println!("cargo:rustc-env=API_URL={}", url);
    Ok(())
}
