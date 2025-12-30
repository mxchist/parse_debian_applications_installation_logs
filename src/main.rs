use std::fs;
use regex::Regex;
//use std::str::FromStr;
use std::vec::Vec;
use std::fmt;
//use pretty_assertions::{assert_eq, Comparison};

#[derive(Debug)]
enum Action {
    Installed,
    Remove,
    StartupPackagesRemove,
    Other(CouldntParse),
}

#[derive(Debug)]
struct CouldntParse {
    line: String,
}

impl Action {

    fn from_str(s: &str) -> Self {
        match s {
            "installed" => Action::Installed,
            "remove" => Action::Remove,
            _ => Action::Other(CouldntParse{line: String::from(s)}),
        }   
    }
    
}

impl fmt::Display for CouldntParse {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.line)
    }
}

#[derive(Debug)]
struct InstallationStatus {
    action: Action,
    package_name: String,
}

fn main() {
    let (mut to_install, mut to_remove) = (Vec::<String>::new(), Vec::<String>::new());

    let contents = fs::read_to_string(String::from("/home/max/Documents/system_config/var/log/dpkg.log")).unwrap();
    let mut lines = contents.lines();
    println!("The initial lines count is: {}", lines.clone().count());
    while let Some(last_line) = lines.next_back() {
        let event = get_event(last_line, &lines.clone().count());
        match event.action {
            Action::Installed => {
                match to_remove.iter().position(|key| key.eq(&event.package_name)) {
                    Some(p) => {
                        //let to_remove_before = to_remove.clone();
                        to_remove.remove(p);
                        //assert_eq!(to_remove_before, to_remove, "the last_line: {}");
                        //println!("the package to remove: {}, the to_remove:\n{}", &event.package_name, Comparison::new(&to_remove_before, &to_remove));
                    },
                    None => to_install.push(event.package_name),
                }
            },
            Action::Remove => to_remove.push(event.package_name),
            Action::Other(couldnt_parse) => eprintln!("not parsed line: {couldnt_parse}"),
            Action::StartupPackagesRemove => (),
        };
    }
    dbg!(":?", &to_remove);
    dbg!(":?", &to_install);
}

fn get_event<'a>(last_line: &'a str, lines_count: &usize) -> InstallationStatus {
    let re = Regex::new(r"^2\d{3}\-\d\d\-\d\d \d\d:\d\d:\d\d (status )?(?<action>installed|remove) (?<package_name>[^:]+):").unwrap();
    let caps = re.captures(last_line);
    let mut installation_status = InstallationStatus{
        action: Action::Other(CouldntParse{line: String::from("init")}),
        package_name: String::new(),
    };
    match caps {
        Some(caps) => match caps.name("action") {
            Some(action) => {
                installation_status.action = Action::from_str(action.as_str());
                installation_status.package_name = caps.name("package_name").unwrap().as_str().to_string();
            },
            None => eprintln!("Caption is empty. last_line is: {last_line}"),
        },
        None => installation_status.action = check_startup_packages_remove(last_line).or_else(
            || {eprintln!("the lines count is: {lines_count}, the last_line is: {last_line}");
            None}
        ).unwrap(),
    }
    installation_status
}

fn check_startup_packages_remove<'a>(last_line: &'a str) -> Option<Action> {
    let re = Regex::new(r"^2\d{3}\-\d\d\-\d\d \d\d:\d\d:\d\d (?<startup_packages>startup packages remove)").unwrap();
    match re.captures(last_line) {
        Some(caps) => {
            if let Some(_) = caps.name("startup_packages")  {
                Some(Action::StartupPackagesRemove)
            }
            else {
                None
            }
        },
        None => None,
    }
}
