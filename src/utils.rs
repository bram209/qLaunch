use std::path::Path;
use std::fs;
use std::env;

pub fn qlauncher_settings_folder() -> String {
    let home_dir = env::var_os("HOME").unwrap();
    let path = home_dir.into_string().unwrap() + "/.qlauncher/";
    let settings_folder = Path::new(&path);
    if !settings_folder.exists() {
        fs::create_dir(settings_folder);
    }

    path.to_owned()
}