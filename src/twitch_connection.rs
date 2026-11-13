use crate::command_trigger::{check_if_command_used, trigger_exact_command};
use secrecy::{ExposeSecret, SecretString};
use std::collections::HashMap;
use std::env;
use std::io::prelude::*;
use std::io::{BufRead, BufReader, Write};
use std::net::{SocketAddr, TcpStream};

#[derive(Debug)]
pub struct Config {
    bot_name: String,
    channel: String,
    oauth_token: SecretString,
}

pub fn setup_tokens() -> Result<Config, Box<dyn std::error::Error>> {
    let path = dirs::config_dir()
        .ok_or("couldn't find config dir")?
        .join("D:/DONOTOPENONSTREAM/secrets.env");

    dotenvy::from_path(&path)
        .map_err(|e| format!("couldn't load config file {}: {}", path.display(), e))?;

    Ok(Config {
        bot_name: env::var("TWITCH_BOT_NAME")?,
        channel: env::var("TWITCH_CHANNEL")?,
        oauth_token: SecretString::from(env::var("TWITCH_OAUTH_TOKEN")?),
    })
}

pub fn set_up_twitch_connection(
    auth_data: Config,
    commands: &HashMap<String, fn(Option<&str>) -> String>,
) -> std::io::Result<()> {
    let mut stream = TcpStream::connect("irc.chat.twitch.tv:6667")?;

    let pass = format!("PASS oauth:{}\r\n", auth_data.oauth_token.expose_secret());
    stream.write_all(pass.as_bytes())?;
    let nick = format!("NICK {}\r\n", auth_data.bot_name);
    stream.write_all(nick.as_bytes())?;
    let channel = format!("JOIN #{}\r\n", auth_data.channel);
    stream.write_all(channel.as_bytes())?;

    let mut reader = BufReader::new(stream);
    let mut line = String::new();

    loop {
        line.clear();
        if reader.read_line(&mut line)? == 0 {
            println!("Connection closed by server");
            break;
        }
        print!("{}", line);

        if line.starts_with("PING") {
            reader.get_mut().write_all(b"PONG :tmi.twitch.tv\r\n")?;
        }

        let prep_split = format!("#{} :", auth_data.channel);

        if line.contains("PRIVMSG")
            && let Some((irc_stuff, message)) = line.split_once(&prep_split)
        {
            let formated_message = message.trim_end_matches(['\r', '\n']);
            if check_if_command_used(formated_message) {
                let msg = trigger_exact_command(commands, formated_message);
                send_message(reader.get_mut(), &msg, &auth_data.channel);
            }
        }
    }

    Ok(())
}

pub fn send_message(stream: &mut TcpStream, text: &str, channel: &str) {
    let message = format!("PRIVMSG #{} :{}\r\n", channel, text);
    println!("{}", message);
    stream.write_all(message.as_bytes()).expect("empty message");
}

// async def connect_to_chat_and_receive_messages():

//     xyllieAi = MythoMax()

//     server = defset.SERVER
//     port = defset.PORT
//     nickname = defset.NICKNAME
//     token = defset.TOKEN
//     channel = defset.CHANNEL

//     sock = socket.socket()

//     sock.connect((server, port))

//     sock.send(f"PASS {token}\n".encode('utf-8'))
//     sock.send(F"NICK {nickname}\n".encode('utf-8'))
//     sock.send(F"JOIN {channel}\n".encode('utf-8'))
//     print(f"Połączono z czatem: {channel}")

//     file_name = 'twitchData\\data\\' + defset.CHANNEL + '.txt'

//     with open(file_name, 'a', encoding='utf-8', errors="ignore") as chat_data:

//         while True:
//             await receive_message(sock, chat_data, xyllieAi)

// async def receive_message(sock, chat_data, ai_model):

//             message = sock.recv(2048).decode('utf-8')
//             print(message)
//             if message.startswith('PING'):
//                 sock.send('PONG\n'.encode('utf-8'))

//             elif 'PRIVMSG' in message:
//                 channel_name = defset.CHANNEL.lower()
//                 message = message.split(f'{channel_name} :', 1)[-1]
//                 answer = await ai_model.generate_answer(message)
//                 print(answer)
//                 await save_message(message, answer, chat_data)
//             await asyncio.sleep(0.2)
