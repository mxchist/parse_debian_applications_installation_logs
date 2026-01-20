//use super::super::*;
use crate::{Action, InstallationStatusAptHistory, TimeBegin};
use pretty_assertions::assert_eq;
use std::collections::HashMap;
use std::time::Duration;

#[test]
fn check_properly_removed_from_installation_candidates() {
    let (mut to_install, mut to_remove) = (
        vec![
            String::from("brave"),
            String::from("firefox"),
            String::from("thunderbird"),
        ],
        vec![String::from("inet-utils")],
    );
    let (mut time_begin, mut stats) = (TimeBegin::new(), HashMap::<String, Duration>::new());
    let package = String::from("brave");
    (Action::Remove).manage_installation_candidates_on_action(
        package,
        &mut to_install,
        &mut to_remove,
        &mut time_begin,
        &mut stats,
    );

    assert_eq!(
        to_install,
        vec![String::from("firefox"), String::from("thunderbird"),]
    );

    assert_eq!(to_remove, vec![String::from("inet-utils")]);
}

#[test]
fn check_properly_removed_from_remove_candidates() {
    let (mut to_install, mut to_remove) = (
        vec![
            String::from("brave"),
            String::from("firefox"),
            String::from("thunderbird"),
        ],
        vec![String::from("inet-utils")],
    );
    let (mut time_begin, mut stats) = (TimeBegin::new(), HashMap::<String, Duration>::new());
    let package = String::from("inet-utils");
    (Action::Installed).manage_installation_candidates_on_action(
        package,
        &mut to_install,
        &mut to_remove,
        &mut time_begin,
        &mut stats,
    );
    assert_eq!(
        to_install,
        vec![
            String::from("brave"),
            String::from("firefox"),
            String::from("thunderbird"),
        ],
    );

    assert_eq!(to_remove, Vec::<String>::new(),);
}

#[test]
fn check_properly_adding_to_installation_candidates() {
    let (mut to_install, mut to_remove) = (
        vec![String::from("brave"), String::from("firefox")],
        vec![String::from("inet-utils")],
    );
    let (mut time_begin, mut stats) = (TimeBegin::new(), HashMap::<String, Duration>::new());
    let package = String::from("thunderbird");
    (Action::Installed).manage_installation_candidates_on_action(
        package,
        &mut to_install,
        &mut to_remove,
        &mut time_begin,
        &mut stats,
    );

    assert_eq!(
        to_install,
        vec![
            String::from("brave"),
            String::from("firefox"),
            String::from("thunderbird"),
        ],
    );

    assert_eq!(to_remove, vec![String::from("inet-utils")]);
}

#[test]
fn check_properly_adding_to_remove_candidates() {
    let (mut to_install, mut to_remove) = (
        vec![String::from("brave"), String::from("firefox")],
        vec![String::from("inet-utils")],
    );
    let (mut time_begin, mut stats) = (TimeBegin::new(), HashMap::<String, Duration>::new());
    let package = String::from("thunderbird");
    (Action::Remove).manage_installation_candidates_on_action(
        package,
        &mut to_install,
        &mut to_remove,
        &mut time_begin,
        &mut stats,
    );

    assert_eq!(
        to_install,
        vec![String::from("brave"), String::from("firefox")],
    );
    assert_eq!(
        to_remove,
        vec![String::from("inet-utils"), String::from("thunderbird")]
    );
}
