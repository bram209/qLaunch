use execution;
use ini::Ini;
use std::collections::{HashSet, HashMap};
use std::fs;
use std::env;
use std::path::Path;
use utils;

fn icon_cache_path() -> String {
//    let home_dir = env::var_os("HOME").unwrap();
//    utils::qlauncher_settings_folder().join("icon_cache").as_path()
    utils::qlauncher_settings_folder() + "icon_cache"
}

pub struct IconLookup {
    icon_theme: String,
    icon_cache: HashMap<String, String>
}

impl IconLookup {
    pub fn new() -> IconLookup {
        let icon_theme: String = get_icon_theme().unwrap();
        let icon_cache: HashMap<String, String> = match Ini::load_from_file(&icon_cache_path()) {
            Ok(ini) => {
                let ini: Ini = ini;
                if icon_theme.as_str() != ini.general_section().get("Theme").unwrap() {
                    //todo error handling
                    HashMap::new()
                } else {
                    ini.section(Some("Cache")).unwrap().clone()
                }
            },
            Err(err) => HashMap::new()
        };

        IconLookup {
            icon_theme: icon_theme,
            icon_cache: icon_cache
        }
    }

    pub fn save(&self) {
        let mut ini: Ini = Ini::new();
        ini.with_section(None::<String>).set("Theme", self.icon_theme.clone());
        for (icon_name, icon_path) in &self.icon_cache {
            ini.with_section(Some("Cache".to_owned())).set(icon_name.clone(), icon_path.clone());
        }

        let result = ini.write_to_file(&icon_cache_path());
    }

    pub fn lookup(&mut self, icon_name: &str) -> Option<String> {
        if icon_name.chars().next().unwrap() == '/' {
            //absolute path
            return Some(icon_name.to_owned());
        }

        {
            let icon_path = self.icon_cache.get(icon_name);
            if icon_path.is_some() {
                if icon_path.unwrap() == "None" {
                    return None;
                }
                return icon_path.cloned();
            }
        }

        let mut icon_path = self.lookup_recursive(self.icon_theme.as_ref(), icon_name, &mut HashSet::new());
        if icon_path.is_some() {
            self.icon_cache.insert(icon_name.to_owned(), icon_path.clone().unwrap());
        } else {
            icon_path = self.lookup_dir(icon_name, &"/usr/share/pixmaps/");
            if icon_path.is_some() {
                self.icon_cache.insert(icon_name.to_owned(), icon_path.clone().unwrap());
            } else {
                self.icon_cache.insert(icon_name.to_owned(), "None".to_owned());
            }
        }
        icon_path
    }

    fn lookup_dir(&self, icon_name: &str, path: &str) -> Option<String> {
        if let Ok(paths) = fs::read_dir(path) {
            for entry in paths {
                let path = entry.unwrap().path();
                if !path.is_dir() {
                    if path.file_stem().unwrap() == icon_name {
                        return Some(path.to_str().unwrap().to_owned());
                    }
                }
            }
        }

        None
    }

    fn lookup_recursive(&self, theme_name: &str, icon_name: &str, ref mut checked: &mut HashSet<String>) -> Option<String> {
        if checked.contains(theme_name) {
            return None;
        }

        checked.insert(theme_name.to_owned());
        let icon_theme: Option<IconTheme> = IconTheme::from_file(&("/usr/share/icons/".to_owned() + theme_name + "/")[..]);
        match icon_theme {
            Some(icon_theme) => {
                let mut icon_theme: IconTheme = icon_theme;
                icon_theme.directories
                    .sort_by(|a, b| b.icon_size.cmp(&a.icon_size)); //bigger is better?

                for theme_dir in icon_theme.directories {
                    let icon_path =  self.lookup_dir(icon_name, theme_dir.path.as_str());
                    if icon_path.is_some() {
                        return icon_path;
                    }
                }
                for inherit in icon_theme.inherits {
                    let result = self.lookup_recursive(inherit.as_ref(), icon_name, checked);
                    if result.is_some() {
                        return result;
                    }
                }
            },
            None => {}//println! ("Lookup failed for: {}", theme_name); }
        }

        //    unimplemented!()
        None
    }
}


pub fn get_icon_theme() -> Option<String> {
    utils::get_gsetting("org.gnome.desktop.interface", "icon-theme")
}

struct IconTheme {
    name: String,
    comment: String,
    directories: Vec<ThemeDirectory>,
    inherits: Vec<String>
}

impl IconTheme {
    fn from_file(theme_path: &str) -> Option<IconTheme> {
        let ini: Ini = match Ini::load_from_file(AsRef::<str>::as_ref(&(theme_path.to_owned() + "index.theme"))) {
            Ok(ini) => ini,
            Err(err) => return None
        };

        let section = ini.section(Some("Icon Theme")).unwrap();
        let name = section.get("Name").unwrap();
        let comment = section.get("Comment").unwrap();

        //directories
        let directory_names: &str = section.get("Directories").unwrap();
        let directory_names: Vec<&str> = directory_names.split(',').collect();

        let mut directories: Vec<ThemeDirectory> = vec![];
        for directory_name in directory_names {
            let dir_section = ini.section(Some(directory_name)).unwrap();
            directories.push(ThemeDirectory {
                path: theme_path.to_owned() + directory_name,
                context: dir_section.get("Context").unwrap().to_owned(),
                icon_size: dir_section.get("Size").unwrap().parse::<u16>().unwrap()
            });
        }

        //inherits
        let inherits: Vec<String> = match section.get("Inherits") {
            Some(inherits) => {
                inherits.split(',').map(String::from).collect()
            },
            None => vec![]
        };

        Some(IconTheme {
            name: name.to_owned(),
            comment: comment.to_owned(),
            directories: directories,
            inherits: inherits
        })
    }
}

struct ThemeDirectory {
    path: String,
    context: String,
    icon_size: u16
}