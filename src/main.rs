mod irc_protocol;
mod irc_structs;

use irc_structs::*;

fn print_msg(msg: &Message) {
    if let Some(prefix) = &msg.prefix {
        match prefix {
            Prefix::Server(serv) => println!("ServerPrefix: {}", serv),
            Prefix::User(user) => println!(
                "UserPrefix: Nick: {}, User: {:?}, Host: {:?}",
                user.nick, user.user, user.host
            ),
        }
    }
    match &msg.command {
        Command::String(s) => println!("StringCommand: {}", s),
        Command::Code(c) => println!("CodeCommand: {}", c),
    }
    for p in &msg.params {
        println!("Param: {}", p);
    }
}

fn main() {
    let parser = irc_protocol::MessageParser::new();
    let result = parser.parse("123\r\n");

    match result {
        Err(e) => {
            println!("Error: {}", e);
        }
        Ok(msg) => print_msg(&msg),
    }
}
