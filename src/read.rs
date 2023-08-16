
use std::{path::PathBuf, fs};
use anyhow::Result;

use nom_bibtex::Bibtex;

pub fn load_file(path: &PathBuf) -> Result<Bibtex> {
    let content = fs::read_to_string(path)?;
    let lib = nom_bibtex::Bibtex::parse(&content)?;
    Ok(lib)
}



