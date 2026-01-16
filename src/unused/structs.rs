enum DurationSinceBeginning {
    ActionInstalledRemove{duration: Duration},
    ActionInstalledPushr{uration: Duration},
    ActionRemoveRemove{duration: Duration},
    ActionRemovePush{duration: Duration},
}

impl DurationSinceBeginning {
    fn to_string(&self) -> String {
        match self {
            action_installed_remove: Duration::from_secs(0),
            action_installed_push: Duration::from_secs(0),
            action_remove_remove: Duration::from_secs(0),
            action_remove_push: Duration::from_secs(0),
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

struct DurationsVectorOperations {
    Remove: Duration,
    Push: Duration,
}
