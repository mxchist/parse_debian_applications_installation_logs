use std::fs;
use regex::Regex;
use std::str::FromStr;
use std::vec::Vec;
use std::fmt;

#[derive(Debug)]
enum Action {
    Installed,
    Remove,
    Other,
}

#[derive(Debug)]
struct CouldntParse {
    line: String,
}

impl FromStr for Action {
    type Err = CouldntParse;

    fn from_str(s: &str) -> Result<Self, CouldntParse> {
        match s {
            "installed" => Ok(Action::Installed),
            "remove" => Ok(Action::Remove),
            _ => Err(CouldntParse{line: String::from(s)}),
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
    let mut last_line: &str;
    for _i in 1..1000 {
        last_line = lines.next_back().unwrap();
        let event = get_event(last_line);
        match event.action {
            Action::Installed => to_install.push(event.package_name),
            Action::Remove => to_remove.push(event.package_name),
            _ => (),
        };
        dbg!("{}", get_event(last_line));
    }
}

fn get_event<'a>(last_line: &'a str) -> InstallationStatus {
    let regex = Regex::new(r"^2\d{3}\-\d\d\-\d\d \d\d:\d\d:\d\d (status)? (?<action>installed|remove) (?<package_name>[^:]+):").unwrap();
    let caps = regex.captures(last_line).unwrap();
    let mut installation_status = InstallationStatus{
        action: Action::Other,
        package_name: String::new(),
    };
    match caps.name("action") {
        Some(action) => {
            installation_status.action = Action::from_str(action.as_str()).inspect_err(|e| eprintln!("{e}")).unwrap_or(Action::Other);
            installation_status.package_name = caps.name("package_name").unwrap().as_str().to_string();
        },
        None => eprintln!("{last_line}"),
    }
    caps.name("action").unwrap().as_str().to_string();
    installation_status
}

