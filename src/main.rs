use crate::{
    command_map::prepare_map,
    twitch_connection::{set_up_twitch_connection, setup_tokens},
};

pub mod command_map;
pub mod command_trigger;
pub mod twitch_connection;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = setup_tokens()?;
    let commands = prepare_map();
    let connection = set_up_twitch_connection(config, &commands);

    Ok(())
}
