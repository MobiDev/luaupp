


use std::{path::{PathBuf}, io::Write};

use full_moon::ast::Ast;
use luaupp::{Project, transformations::string::transform_string};
use structopt::StructOpt;
use stylua_lib::OutputVerification;



#[derive(StructOpt)]
struct Opt {
    #[structopt(subcommand)] // Note that we mark a field as a subcommand
    cmd: Commands,
}


#[derive(StructOpt)]
enum Commands {
    Build {

    }
}

fn main() {
    let opt = Opt::from_args();
    match opt.cmd {
        Commands::Build {} => {
            match std::env::current_dir() {
                Ok(dir) => {
					match dir.read_dir() {
						Ok(read_dir_ok) => {
							for entry in read_dir_ok {
								match entry {
									Ok(file) => {
										if file.file_name() == "luppconfig.toml" {
											build_config(file.path());
										}
									},
									Err(_) => todo!(),
								}
							}
						},
						Err(_) => todo!(),
    				}
				},
                Err(_) => todo!(),
            };
        }
    }
    // let mut data = String::new();
    // let mut f = File::open("test.luaupp").expect("Unable to open file");
    // f.read_to_string(&mut data).expect("Unable to read string");
    // let mut newf = File::create("test.luau").expect("Unable to create file");
    // newf.write_all(transform_string(data).as_bytes())
    //     .expect("Unable to write data");
}

fn build_config(path: PathBuf) {
	let config: Project = toml::from_str(std::str::from_utf8(&std::fs::read(path).unwrap()).unwrap()).unwrap();
	std::fs::remove_dir_all(config.compiler.out_dir.clone()).expect("Error occured removing out directory");
	std::fs::create_dir_all(config.compiler.out_dir.clone()).expect("Error occured creating out directory");
	compile_directory(
		std::fs::canonicalize(config.compiler.root_dir).unwrap(), 
		std::fs::canonicalize(config.compiler.out_dir).unwrap()
	);
}

fn compile_directory(origin_dir: PathBuf, out_dir: PathBuf) {
	match origin_dir.read_dir() {
		Ok(read_dir_ok) => {
			for entry in read_dir_ok {
				match entry {
					Ok(file) => {
						if file.path().extension().unwrap() == "lupp" {
							let mut origin_path = origin_dir.clone();
							let mut out_path = out_dir.clone();

							origin_path.push(file.file_name());
							out_path.push(file.file_name());
							out_path.push(out_path.with_extension("luau"));
							compile_file(
								origin_path,
								out_path
							);
						}
						// if  {
						// 	build_config(file.path());
						// }
					},
					Err(_) => todo!(),
				}
			}
		},
		Err(_) => todo!(),
	}
}

fn compile_file(origin_path: PathBuf, out_path: PathBuf) {
	let file_text_vec = std::fs::read(origin_path).unwrap();
	let file_text = std::str::from_utf8(&file_text_vec).unwrap();
	
	let mut out_path_dir = out_path.clone();
	out_path_dir.pop();
	std::fs::create_dir_all(out_path_dir).expect("error creating dir");


	let mut file = std::fs::File::create(out_path).expect("create failed");
	file.write_all(transform_string(file_text).as_bytes()).expect("write failed");
}
