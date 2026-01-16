use super::*;

#[test]
fn check_properly_removed_from_installation_candidates() {
    let (mut to_install, mut to_remove) = (
        vec![
            String::from("brave"),
            String::from("firefox")
        ],
        vec![
            String::from("brave")
        ]
    );
    assert!(false, "There is no implementation of the specialized method of deleteing from installation candidates");
}

#[test]
fn check_properly_removed_from_remove_candidates() {
    let (mut to_install, mut to_remove) = (
        vec![
            String::from("brave"),
            String::from("firefox")
        ],
        vec![
            String::from("brave")
        ]
    );
    assert!(false, "There is no implementation of the specialized method of deleteing from removing candidates");
}

#[test]
fn check_properly_adding_to_installation_candidates() {
    let (mut to_install, mut to_remove) = (
        vec![
            String::from("brave"),
            String::from("firefox")
        ],
        vec![
            String::from("brave")
        ]
    );
    assert!(false, "There is no implementation of the specialized method of adding to deleting candidates");
}

#[test]
fn check_properly_adding_to_remove_candidates() {
    let (mut to_install, mut to_remove) = (
        vec![
            String::from("brave"),
            String::from("firefox")
        ],
        vec![
            String::from("brave")
        ]
    );
    assert!(false, "There is no implementation of the specialized method of adding to removing candidates");
}

