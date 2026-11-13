use std::{collections::HashMap, net::TcpStream};

pub fn check_if_command_used(input: &str) -> bool {
    input.starts_with('$')
}

pub fn trigger_exact_command(
    commands: &HashMap<String, fn(Option<&str>) -> String>,
    input: &str,
) -> String {
    let command = input.split(' ').next().unwrap();
    // println!("{command:?}");

    if let Some(com_func) = commands.get(command) {
        com_func(Some(input))
    } else {
        println!("function not detected {}", command);
        String::new()
    }
}

// pub use prepare_map;
// pub use command_trigger;

// let config = setup_tokens()?;

// println!("bot: {}, channel: {}", config.bot_name, config.channel);
// println!(
//     "token loaded: {}",
//     !config.oauth_token.expose_secret().is_empty()
// );

// let messages = vec![
//     "$lurk",
//     "i love you kimi",
//     ";333",
//     "$help",
//     "lurk",
//     "$checkin",
//     "$commands",
//     "$fakecommand",
//     "$com 67",
// ];

// let commands = prepare_map::prepare_map();

// for command in messages {
//     if command_trigger::check_if_command_used(command) {
//         let command_result = command_trigger::trigger_exact_command(&commands, command);
//         println!("{}", command_result);
//     }
// }
