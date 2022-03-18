use std::path::PathBuf;

use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Compiler {
    pub root_dir: PathBuf,
    pub out_dir: PathBuf,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Project {
	pub compiler: Compiler
}