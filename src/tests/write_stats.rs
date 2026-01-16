use super::*;
use pretty_assertions::assert_eq;
use std::time::Duration;

#[test]
fn duration_should_be_added() {
    let (mut d1, d2) = (Duration::from_secs(2), Duration::from_secs(3));
    d1 += d2;
    assert_eq!(d1, Duration::from_secs(5));
}

#[test]
fn new_stats_based_on_duration_should_be_added() {
    assert!(false);
}

#[test]
fn existing_stats_based_on_duration_should_be_updated() {
    assert!(false);
}

#[test]
fn new_stats_based_on_systemtime_should_be_added() {
    assert!(false);
}

#[test]
fn existing_stats_based_on_systemtime_should_be_updated() {
    assert!(false);
}
