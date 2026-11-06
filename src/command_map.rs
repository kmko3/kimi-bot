use std::collections::HashMap;

pub fn trigger_lurk(_message: Option<&str>) -> String {
    String::from("lurk")
}

pub fn trigger_checkin(_message: Option<&str>) -> String {
    String::from("checkin")
}

pub fn trigger_help(_message: Option<&str>) -> String {
    String::from("help")
}

pub fn trigger_commands(_message: Option<&str>) -> String {
    String::from("trigger_commands")
}

pub fn prepare_map() -> HashMap<String, fn(Option<&str>) -> String> {
    let mut repl_env: HashMap<String, fn(Option<&str>) -> String> = HashMap::new();

    repl_env.insert("$help".to_string(), trigger_help);
    repl_env.insert("$lurk".to_string(), trigger_lurk);
    repl_env.insert("$checkin".to_string(), trigger_checkin);
    repl_env.insert("$commands".to_string(), trigger_commands);

    repl_env
}
