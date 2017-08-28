use std::fs;
use std::fs::File;
use std::io;
use std::env;
use ini::Ini;
use icons;

#[derive(Debug, Clone)]
pub struct Application {
    pub name: String,
    pub comment: String,
    pub exec: String,
    pub icon_path: Option<String>
}

pub struct ApplicationReader {
    icon_lookup: icons::IconLookup
}


macro_rules! ok (
($e:expr) => (match $e { Some(e) => e, None => return None })
);


impl ApplicationReader {
    pub fn new() -> ApplicationReader {
        ApplicationReader {
            icon_lookup: icons::IconLookup::new()
        }
    }


    pub fn read_applications(&mut self) -> Vec<Application> {
        println!("IconTheme: {:?}", icons::get_icon_theme().unwrap());
        let mut applications: Vec<Application> = vec![];

        let home_dir = env::var_os("HOME").unwrap();
        let local_apps = home_dir.into_string().unwrap() + "/.local/share/applications/";
       
        for path in vec!["/usr/share/applications/", &local_apps] {
            match fs::read_dir(path) {
                Err(why) => println!("! {:?}", why.kind()),
                Ok(paths) => for path in paths {
                    if let Some(mut application) = self.read_application(path.unwrap().path().to_str().unwrap()) {
                        applications.push(application);
                    }
                },
            }
        }

        self.icon_lookup.save();
        applications
    }

    fn read_application(&mut self, path: &str) -> Option<Application> {
        if let Ok(entry) = Ini::load_from_file(path) {
            let section = ok!(entry.section(Some("Desktop Entry".to_owned())));
            if ok!(section.get("Type")) == "Application" {
                let mut exec = ok!(section.get("Exec")).clone();
                //todo just remove %f, %u, %F or %U field codes for now
                //todo expand_field_codes(...)
                //dirty temporary fix
                let len = exec.len();
                if exec.clone().chars().nth(len - 2).unwrap() == '%' {
                    exec.truncate(len - 3);
                }

                let icon_path = match section.get("Icon") {
                    Some(icon) => {
                        self.icon_lookup.lookup(icon)
                    },
                    None => None
                };

                return Some(Application {
                    name: ok!(section.get("Name")).to_owned(),
                    comment: section.get("Comment").unwrap_or(&"".to_owned()).to_owned(),
                    exec: exec.to_owned(),
                    icon_path: icon_path,
                });
            }
        }

        None
    }
}