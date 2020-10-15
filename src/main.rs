use std::fs::{canonicalize, create_dir, File};
use std::io::Write;

use alpm::SigLevel;
use serde::Serialize;
use symlink::symlink_file;
use tempdir::TempDir;
use std::env;
use std::io;
use std::env::{args, Args};
use std::path::PathBuf;
use std::process::exit;

#[derive(Serialize)]
struct ArchPkg<'a> {
    arch: &'a str,
    name: &'a str,
    version: &'a str,
    description: &'a str,
    last_built: i64,
    filename: &'a str,
    size: i64,
}

fn parse_args(args: Args) -> Result<(String, String), ()> {
    let args: Vec<String> = args.collect();
    match args.len(){
        3=>{
            Ok((args[1].clone(), args[2].clone()))
        },
        _ => Err(())
    }
}

fn get_exec_name() -> String {
    std::env::current_exe()
        .ok()
        .and_then(|pb| pb.file_name().map(|s| s.to_os_string()))
        .and_then(|s| s.into_string().ok())
        .unwrap()
}

fn main() {
    let usage: String = format!("{}\n{} {}", "Arch DB Meta Exporter", get_exec_name(), "<db_path> <output_path>");

    let (db_path, output_path) = parse_args(args()).unwrap_or_else(|()|{
        println!("{}", usage);
        exit(1);
    });

    let tmp_dir = TempDir::new("dbmeta").expect("Unable to create temp dir");
    let tmp_dir_path = String::from((&tmp_dir).path().to_str().unwrap());
    let sync_db_path = tmp_dir.path().join("sync");
    let cloned_db_path = sync_db_path.join("repo.db");

    create_dir(&sync_db_path).expect("Unable to create temp sync directory");
    symlink_file(canonicalize(db_path).unwrap(), cloned_db_path).expect("Cannot create symlink");

    let alpm_handler = alpm::Alpm::new(&tmp_dir_path, &tmp_dir_path).expect("Unable to initialize alpm");
    let db = alpm_handler.register_syncdb("repo", SigLevel::DATABASE_OPTIONAL).expect("Unable to parse database");

    let packages = db.pkgs().expect("Unable to read package list");
    let pkg_meta: Vec<ArchPkg> = packages.into_iter().map(|package|
        ArchPkg { arch: package.arch().unwrap_or("any"), name: package.name(), version: package.version().as_str(), description: package.desc().unwrap_or(""), last_built: package.build_date(), filename: package.filename(), size: package.size() }
    ).collect();

    let mut json_file = File::create(output_path).expect("Cannot create output file");
    write!(json_file, "{}", serde_json::to_string(&pkg_meta).expect("Cannot serialize meta data")).expect("Cannot write to json file");
}
