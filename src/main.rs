use clap::Clap;
use serde::{Deserialize, Serialize};
use std::{fs, path::PathBuf};
#[derive(Deserialize, Serialize)]
struct Whatever([String; 2]);

#[derive(Clap)]
struct Opts {
    ///(Optional) Path of folder, defaults to current directory
    path: Option<PathBuf>,
    #[clap(short, long)]
    word_only: bool,
}

fn main() -> Result<(), ureq::Error> {
    let x = Opts::parse();
    let api_url = "https://random-word-api.herokuapp.com/word?number=2".to_owned();
    let result: Whatever = ureq::get(&api_url).call()?.into_json()?;
    println!("{}", result.0.join("-"));
    if x.word_only {
        return Ok(());
    } else {
        match x.path {
            Some(y) => fs::create_dir_all(y.join(result.0.join("-"))),
            None => fs::create_dir_all(format!("./{}", result.0.join("-"))),
        }?;
    }
    Ok(())
}
