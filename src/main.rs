use regex::Regex;
use std::fs;
//use std::str::FromStr;
use pretty_assertions::Comparison;
use rsbash::rashf;
use std::collections::HashMap;
use std::fmt;
use std::time::{Duration, SystemTime};
use std::vec::Vec;
use time::{PrimitiveDateTime, macros::format_description, parsing::Parsable};

trait LogEvent<Rhs=Self> {
    type Output;

    fn get_event(last_line: &str, line_number: &usize, log_type: LogType) -> Self::Output;
}

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
            _ => Action::Other(CouldntParse {
                line: String::from(s),
            }),
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

type InstallationStatusAptHistory = (Action, Vec<String>);

struct TimeBegin {
    old: SystemTime,
    program_start: SystemTime,
    action_installed: SystemTime,
    action_installed_remove: SystemTime,
    action_remove: SystemTime,
}

impl TimeBegin {
    fn new() -> Self {
        Self {
            old: SystemTime::UNIX_EPOCH,
            program_start: SystemTime::now(),
            action_installed: SystemTime::UNIX_EPOCH,
            action_installed_remove: SystemTime::UNIX_EPOCH,
            action_remove: SystemTime::UNIX_EPOCH,
        }
    }
}

struct TimeBeginRemoveDependecies {
    old: SystemTime,
    package_list_iterating: SystemTime,
    package_list_remove: SystemTime,
}

impl TimeBeginRemoveDependecies {
    fn new() -> Self {
        Self {
            old: SystemTime::UNIX_EPOCH,
            package_list_iterating: SystemTime::UNIX_EPOCH,
            package_list_remove: SystemTime::UNIX_EPOCH,
        }
    }
}

enum LogType {
    GreppedDpkgLog,
    AptHistoryGzip,
}

fn main() {
    analyze_grepped_dpkg_log();
    //assert_log_lines_order();
}

fn analyze_grepped_dpkg_log() {
    let (mut to_install, mut to_remove) = (Vec::<String>::new(), Vec::<String>::new());
    let mut stats = HashMap::<String, Duration>::new();
    let (mut time_begin, mut time_begin_remove_dependencies) =
        (TimeBegin::new(), TimeBeginRemoveDependecies::new());

    time_begin.old = SystemTime::now();

    // "/home/max/Documents/system_config/var/log/dpkg.log"
    let contents = fs::read_to_string(get_path(LogType::GreppedDpkgLog)).unwrap();
    write_stats(
        String::from("file reading"),
        &mut stats,
        &mut time_begin.old,
    );
    //for (k, v) in stats.iter() {
    //    println!("{k}\t{}", format_duration(v));
    //}
    //panic!("Immediate interruption");
    let mut lines = contents.lines();
    //println!("The initial lines count is: {}", lines.clone().len());
    for (line_number, last_line) in lines.rev().enumerate() {
        time_begin.old = SystemTime::now();
        let event = InstallationStatus::get_event(last_line, &line_number, LogType::GreppedDpkgLog);
        write_stats(String::from("get_event"), &mut stats, &mut time_begin.old);
        match event.action {
            Action::Installed => {
                time_begin.action_installed = SystemTime::now();
                match to_remove.iter().position(|key| key.eq(&event.package_name)) {
                    Some(p) => {
                        //let to_remove_before = to_remove.clone();
                        time_begin.action_installed_remove = SystemTime::now();
                        to_remove.remove(p);
                        write_stats(
                            String::from("Action::Installed, remove"),
                            &mut stats,
                            &mut time_begin.action_installed_remove,
                        );
                        //println!("the package to remove: {}, the to_remove:\n{}", &event.package_name, Comparison::new(&to_remove_before, &to_remove));
                    }
                    None => to_install.push(event.package_name),
                };
                write_stats(
                    String::from("Action::Installed, processing"),
                    &mut stats,
                    &mut time_begin.action_installed,
                );
            }
            Action::Remove => {
                time_begin.action_remove = SystemTime::now();
                to_remove.push(event.package_name);
                write_stats(
                    String::from("Action::Remove"),
                    &mut stats,
                    &mut time_begin.action_remove,
                );
            }
            Action::Other(couldnt_parse) => eprintln!("not parsed line: {couldnt_parse}"),
            Action::StartupPackagesRemove => (),
        };
    }
    //let to_install_before = to_install.clone();
    time_begin.old = SystemTime::now();
    to_install.sort();
    write_stats(
        String::from("to_install, sorting"),
        &mut stats,
        &mut time_begin.old,
    );
    time_begin.old = SystemTime::now();
    to_install.dedup();
    write_stats(
        String::from("to_install, deduplicating"),
        &mut stats,
        &mut time_begin.old,
    );
    time_begin.old = SystemTime::now();
    remove_dependencies_from_packages(
        &mut to_install,
        &mut stats,
        &mut time_begin_remove_dependencies,
    );
    write_stats(
        String::from("remove dependencies from packages"),
        &mut stats,
        &mut time_begin.old,
    );
    //println!("{}", Comparison::new(&to_install_before, &to_install));
    print!("[");
    for package in to_install {
        print!("{package} ");
    }
    println!("]");

    for (k, v) in stats.iter() {
        println!("{k}\t{}", format_duration(v));
    }
}

fn write_stats(
    stats_name: String,
    stats: &mut HashMap<String, Duration>,
    time_begin_old: &mut SystemTime,
) {
    let duration_new = time_begin_old.elapsed().unwrap();
    let mut duration_old = stats.entry(stats_name).or_insert(duration_new);
    *duration_old += duration_new;
}

fn get_path(log_type: LogType) -> String {
     match log_type {
        LogType::GreppedDpkgLog => std::env::var("GREPPED_LOG").unwrap(),
        LogType::AptHistoryGzip => std::env::var("APT_HISTORY_GZIP").unwrap(),
    }
}

impl LogEvent<LogType> for InstallationStatus {
    type Output = InstallationStatus;

    fn get_event(last_line: &str, line_number: &usize, log_type: LogType) -> InstallationStatus {
        match log_type {
            LogType::GreppedDpkgLog => {
                let re = Regex::new(r"^2\d{3}\-\d\d\-\d\d \d\d:\d\d:\d\d (status )?(?<action>installed|remove) (?<package_name>[^:]+):").unwrap();
                let caps = re.captures(last_line);
                let mut installation_status = InstallationStatus {
                    action: Action::Other(CouldntParse {
                        line: String::from("init"),
                    }),
                    package_name: String::new(),
                };
                match caps {
                    Some(caps) => match caps.name("action") {
                        Some(action) => {
                            installation_status.action = Action::from_str(action.as_str());
                            installation_status.package_name =
                                caps.name("package_name").unwrap().as_str().to_string();
                        }
                        None => eprintln!("Caption is empty. last_line is: {last_line}"),
                    },
                    None => {
                        installation_status.action = check_startup_packages_remove(last_line)
                            .or_else(|| {
                                eprintln!(
                                    "{}\n{}",
                                    "The is no rule to parse the line.",
                                    format!(
                                        "The current line number is: {line_number}, the last_line is: {last_line}"
                                    )
                                );
                                None
                            })
                            .unwrap()
                    }
                }
                installation_status
            },
            LogType::AptHistoryGzip => panic!("An atomic InstallationStatus is not implemented for LogType::AptHistoryGzip, only the vector of InstallationStatus allowed"),
        }
    }
}

impl LogEvent<LogType> for InstallationStatusAptHistory {
    type Output = InstallationStatusAptHistory;

    fn get_event(last_line: &str, line_number: &usize, log_type: LogType) -> InstallationStatusAptHistory {
        match log_type {
            LogType::GreppedDpkgLog => panic!("The vector of InstllationStatus is not implemented for LogType::AptHistoryGzip, only an atomic InstallationStatus allowed"),
            LogType::AptHistoryGzip => {
                let command_with_arguments: Vec<&str> = last_line.split(' ').collect();
                if let Some(command) = command_with_arguments.first() {
                    match *command {
                        "apt" | "apt-get" => analyze_apt_command_in_apt_history_log(
                            command_with_arguments.clone()[1..].into_iter()
                                .map(
                                    |s| s.to_string()
                                ).collect()
                        ),
                        "aptdaemon" => (
                            Action::Other(
                                CouldntParse{
                                    line: String::from("aptdaemon")
                                }
                            ),
                            Vec::<String>::new()
                        ),
                        unknown_command => panic!("Here is an unknown command: {unknown_command}"),
                    }
                } else {
                    panic!("The line is empty") 
                }
            }
        }
    }
}

fn check_startup_packages_remove(last_line: &str) -> Option<Action> {
    let re = Regex::new(
        r"^2\d{3}\-\d\d\-\d\d \d\d:\d\d:\d\d (?<startup_packages>startup packages remove)",
    )
    .unwrap();
    match re.captures(last_line) {
        Some(caps) => {
            if let Some(_) = caps.name("startup_packages") {
                Some(Action::StartupPackagesRemove)
            } else {
                None
            }
        }
        None => None,
    }
}

fn remove_dependencies_from_packages(
    packages_list: &mut Vec<String>,
    stats: &mut HashMap<String, Duration>,
    time_begin: &mut TimeBeginRemoveDependecies,
) {
    for package in packages_list.clone() {
        time_begin.old = SystemTime::now();
        let (_, output, _) = rashf!("apt-cache rdepends {}", package).unwrap();
        write_stats(
            String::from("remove dependencies, apt-cache rdepends"),
            stats,
            &mut time_begin.old,
        );
        let mut lines = output.lines();
        time_begin.old = SystemTime::now();
        if let Some(package_name) = lines.next() && package.eq(package_name) {
            if let Some(reverse_depends) = lines.next() && reverse_depends.eq("Reverse Depends:") {
               if let Some(_) = lines.next() {
                   //eprintln!("the first line of reverse dependensies is: {}", first_line);
                   time_begin.package_list_iterating = SystemTime::now();
                   while let Some(p) =
                       packages_list.iter().position(|key| key.eq(&package))
                   {
                       time_begin.package_list_remove = SystemTime::now();
                       packages_list.remove(p);
                       write_stats(
                           String::from("remove dependencies, remove from package_list"),
                           stats,
                           &mut time_begin.package_list_remove,
                       );
                   }
                   write_stats(
                       String::from("remove dependencies, iterating over package_list"),
                       stats,
                       &mut time_begin.package_list_iterating,
                   );
                }
            }
        };
        write_stats(
            String::from("remove dependencies, iterationg over apt-cache output"),
            stats,
            &mut time_begin.old,
        );
    }
}

fn format_duration(d: &std::time::Duration) -> String {
    let seconds = d.as_secs();
    match seconds {
        2.. => format!("{}min {}sec", seconds / 60, seconds % 60),
        1.. => format!("{}sec {}ms", seconds, d.subsec_millis()),
        _ => format! {"{}ms", d.as_millis()},
    }
}

fn assert_log_lines_order() {
    let contents = fs::read_to_string(String::from(get_path(LogType::GreppedDpkgLog))).unwrap();
    let re = Regex::new(r"^(?<time>\d{4}\-\d\d\-\d\d \d\d:\d\d:\d\d) ").unwrap();
    let format = format_description!("[year]-[month]-[day] [hour]:[minute]:[second]");

    let lines = contents.lines();
    //println!("The initial lines count is: {}", lines.clone().len());
    let (mut time_current, mut time_before) = (PrimitiveDateTime::MIN, PrimitiveDateTime::MIN);
    for line in lines {
        time_before = time_current;
        time_current = parse_to_datetime(&re, line, &format);
        assert!(
            &time_before.le(&time_current),
            "{}",
            format!(
                "The time before is less than the time in a current line.\nThe time_before: {time_before}, the current_line: {line}"
            )
        );
    }
}

fn parse_to_datetime(
    re: &Regex,
    line: &str,
    format: &(impl Parsable + ?Sized),
) -> PrimitiveDateTime {
    match re.captures(line) {
        Some(caps) => match caps.name("time") {
            Some(caps) => {
                let time_str = caps.as_str();
                match PrimitiveDateTime::parse(time_str, &format) {
                    Ok(dt) => dt,
                    Err(e) => panic!("{e}, the line: {line}"),
                }
            }
            None => panic!("the is no capture group time, the line: {line}"),
        },
        None => panic!("regex is no matches, the line: {line}"),
    }
}

fn analyze_apt_history_log() {
    let (mut to_install, mut to_install_global, mut to_remove) = (Vec::<String>::new(), Vec::<String>::new(), Vec::<String>::new());
    let mut stats = HashMap::<String, Duration>::new();
    let (mut time_begin, mut time_begin_remove_dependencies) =
        (TimeBegin::new(), TimeBeginRemoveDependecies::new());

    time_begin.old = SystemTime::now();

    // "/home/max/Documents/system_config/var/log/dpkg.log"
    let apt_history_logs = get_path(LogType::GreppedDpkgLog);
    let (_, contents, _) = rashf!("rg --search-zip --no-filename --sort=path --replace '$1' -P '^Commandline: (.+)' {}", apt_history_logs).unwrap();
    write_stats(
        String::from("logs reading"),
        &mut stats,
        &mut time_begin.old,
    );
    for (line_number, line) in contents.lines().enumerate() {
        match InstallationStatusAptHistory::get_event(line, &line_number, LogType::AptHistoryGzip) {
            (Action::Installed, packages_list) => {

            },
            (Action::Remove, packages_list) => {

            },
            (_, _) => eprintln!(
                "Here is unknown operation. The line number is: {line_number}, the line is:\n{line}"
            ),
        };

        // ==============================
        // remove from to_install returned packages with action Remove, add to to_install packages with action Installed 
        // ==============================
    }
}

fn analyze_apt_command_in_apt_history_log(arguments: Vec::<String>) -> InstallationStatusAptHistory {
    let action = match arguments.clone().first().map(|s| s.as_str()) {
        Some("autoremove") => {
            assert_eq!(arguments.len(), 1, "{}",
                format!(
                    "Autoremove arguments count is not equal to 1. The autoremove arguments:\n{}",
                    arguments.join(" ")
                )
            );
            Action::Other(
                CouldntParse{
                    line: String::from("autoremove")
                }
            )
        },
        Some("install") => Action::Installed,
        Some("remove") => Action::Remove,
        Some("reinstall") => Action::Installed,
        Some(other_action) => panic!(
            "apt or apt-get action is unknown, action: {},\nThe line:\n{} ",
            other_action,
            arguments.join(" ")
        ),
        None => panic!("There are no any arguments after command"),
    };
    (
        action,
        analyze_apt_arguments(arguments[1..].into_iter()
            .map(
                |s| s.to_string()).collect()
            )
    )
}

fn analyze_apt_arguments(arguments: Vec<String>) -> Vec<String> {
    let mut is_flags_ended = false;
    let mut packages_list = Vec::<String>::new();
    
    for argument in arguments.clone() {
        match argument.as_str() {
            "-y" | "--yes" => {
                if is_flags_ended {
                    panic!{
                        "{}", format!(
                            "{}\n{} {}\n{}\n{}",
                            "Flags are no longer expected when positional arguments was started, but a flag was encountered.",
                            "The flag:",
                            argument,
                            "The line:",
                            arguments.join(" ")
                        )
                    }
                } else {
                   continue
                }
            },
            _ => {
                match argument.starts_with("-") {
                    true => {
                        match is_flags_ended {
                            true =>
                                panic!{
                                    "{}", format!(
                                        "{}\n{} {}\n{}\n{}",
                                        "Flags are no longer expected when positional arguments was started, but a flag was encountered.",
                                        "The flag:",
                                        argument,
                                        "The line:",
                                        arguments.join(" ")
                                    )
                                },
                            false => 
                                panic!{
                                    "{}", format!(
                                        "{} {}\n{}\n{}",
                                        "Here is an unknown flag:",
                                        argument,
                                        "The line:",
                                        arguments.join(" ")
                                    )
                                }
                            }
                        },
                    false => {
                        is_flags_ended = true;
                        packages_list.push(argument);
                    }
                }
            }

        }
    }
    packages_list
}
