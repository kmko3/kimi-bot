use secrecy::{ExposeSecret, SecretString};
use std::env;
use std::io::prelude::*;
use std::net::{SocketAddr, TcpStream};
use std::io::{BufRead, BufReader, Write};

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

pub fn set_up_twitch_connection(auth_data: Config) -> std::io::Result<()> {
    let mut stream = TcpStream::connect("irc.chat.twitch.tv:6667")?;

    // Your token needs the "oauth:" prefix, add it here if it's not already stored with it
    let pass = format!("PASS oauth:{}\r\n", auth_data.oauth_token.expose_secret());
    stream.write_all(pass.as_bytes())?;
    stream.write_all(b"NICK your_bot_username\r\n")?;
    stream.write_all(b"JOIN #channelname\r\n")?;

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
    }

    Ok(())
}

// SERVER = 'irc.chat.twitch.tv'
// PORT = 6667

// server = defset.SERVER
// port = defset.PORT
// nickname = defset.NICKNAME
// token = defset.TOKEN
// channel = defset.CHANNEL

// sock = socket.socket()

// sock.connect((server, port))

// sock.send(f"PASS {token}\n".encode('utf-8'))
// sock.send(F"NICK {nickname}\n".encode('utf-8'))
// sock.send(F"JOIN {channel}\n".encode('utf-8'))
// print(f"Połączono z czatem: {channel}")

// file_name = 'twitchData\\data\\' + defset.CHANNEL + '.txt'
