use super::*;
use pretty_assertions::assert_eq;
use std::time::{Duration, SystemTime};
use std::collections::HashMap;
use crate::WriteStats;
use std::thread;

#[test]
fn duration_should_be_added() {
    let (mut d1, d2) = (Duration::from_secs(2), Duration::from_secs(3));
    d1 += d2;
    assert_eq!(d1, Duration::from_secs(5));
}

#[test]
fn new_stats_based_on_duration_should_be_added() {
    let mut stats = HashMap::<String, Duration>::new();
    Duration::from_secs(3).update_stats(
        String::from("some operation"),
        &mut stats,
    );
    let d = stats.get("some operation").unwrap();
    assert_eq!(3, d.as_secs());
}

#[test]
fn existing_stats_based_on_duration_should_be_updated() {
    let mut stats = HashMap::<String, Duration>::new();
    Duration::from_secs(3).update_stats(
        String::from("some operation"),
        &mut stats,
    );
    Duration::from_secs(5).update_stats(
        String::from("some operation"),
        &mut stats,
    );
    let d = stats.get("some operation").unwrap();
    assert_eq!(d.as_secs(), 8,
        "{}",
        format!(
            "Duration is: {}, should be: 8", d.as_secs()
        ),
    );
}

#[test]
fn new_stats_based_on_systemtime_should_be_added() {
    let time_begin = SystemTime::now();
    thread::sleep(
        Duration::from_secs(3)
    );

    let mut stats = HashMap::<String, Duration>::new();
    time_begin.update_stats(
        String::from("some operation"),
        &mut stats,
    );
    let d = stats.get("some operation").unwrap();
    assert!(
        matches!(d.as_secs(), 3..=4),
        "{}",
        format!(
            "Duration is: {}", d.as_secs()
        ),
    );
}

#[test]
fn existing_stats_based_on_systemtime_should_be_updated() {
    let mut time_begin = SystemTime::now();
    thread::sleep(
        Duration::from_secs(3)
    );

    let mut stats = HashMap::<String, Duration>::new();
    time_begin.update_stats(
        String::from("some operation"),
        &mut stats,
    );
    
    time_begin = SystemTime::now();
    thread::sleep(
        Duration::from_secs(5)
    );
    time_begin.update_stats(
        String::from("some operation"),
        &mut stats,
    );

    let d = stats.get("some operation").unwrap();
    assert!(
        matches!(d.as_secs(), 8..=9),
        "{}",
        format!(
            "Duration is: {}, should be: 8", d.as_secs()
        ),
    );
}

