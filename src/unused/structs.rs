use std::error;

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

#[derive(Debug)]
struct ParseError {
    source: ParseErrorSource,
    line_number: usize,
    line: String,
}

impl ParseError {
    fn from(source: ParseErrorSource, line: &str) -> Self {
        Self {
            source,
            line: line.to_string(),
            line_number: usize::MAX,
            //line: String::from(
            //    "The line field of the ParseError struct hasn't initialized yet."
            //),
        }
    }

    fn set_line_number(&mut self, line_number: usize) {
        self.line_number = line_number;
    }
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{},\nline_number: {},\n line:\n{}\n",
            self.source.error_message(), self.line_number, self.line,
        )
    }
}

impl error::Error for ParseError {}
