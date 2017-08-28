use std::path::Path;
use std::fs;
use std::env;
use execution;

pub fn qlauncher_settings_folder() -> String {
    let home_dir = env::var_os("HOME").unwrap();
    let path = home_dir.into_string().unwrap() + "/.qlauncher/";
    let settings_folder = Path::new(&path);
    if !settings_folder.exists() {
        fs::create_dir(settings_folder);
    }

    path.to_owned()
}

pub fn get_gsetting(path: &str, key: &str) -> Option<String> {
    let result = execution::execute_and_output(format!("gsettings get {} {}", path, key).to_owned());
    match result {
        Ok(name) => if name.len() > 2 { //theme is surrounded by single quotes
            unsafe {
                Some(name.slice_unchecked(1, name.len() - 1).to_owned())  //'Faba-Mono' -> Faba-Mono
            }
        } else { None },
        Err(e) => panic!(e)
    }
}
