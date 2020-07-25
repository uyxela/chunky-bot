use std::env;
use serenity::{
    model::{channel::Message, gateway::Ready},
    prelude::*,
    utils::MessageBuilder,
};

struct Handler;

impl EventHandler for Handler {
    fn message(&self, context: Context, msg: Message) {
        if msg.content.contains("chunky") || msg.content.contains("Chunky") {
            let text = r#"            88                                  88                    
            88                                  88                    
            88                                  88                    
  ,adPPYba, 88,dPPYba,  88       88 8b,dPPYba,  88   ,d8 8b       d8  
 a8"     "" 88P'    "8a 88       88 88P'   `"8a 88 ,a8"  `8b     d8'  
 8b         88       88 88       88 88       88 8888[     `8b   d8'   
 "8a,   ,aa 88       88 "8a,   ,a88 88       88 88`"Yba,   `8b,d8'    
  `"Ybbd8"' 88       88  `"YbbdP'Y8 88       88 88   `Y8a    Y88'     
                                                             d8'      
                                                            d8'       "#;


            let response = MessageBuilder::new()
                .push_codeblock(text, None)
                .build();

            if let Err(why) = msg.channel_id.say(&context.http, &response) {
                println!("Error sending message: {:?}", why);
            }
        } else if msg.content.contains("squidward") || msg.content.contains("Squidward") {
            let text = r#"               .--'''''''''--.
            .'      .---.      '.
           /    .-----------.    \
          /        .-----.        \
          |       .-.   .-.       |
          |      /   \ /   \      |
           \    | .-. | .-. |    /
            '-._| | | | | | |_.-'
                | '-' | '-' |
                 \___/ \___/
              _.-'  /   \  `-._
            .' _.--|     |--._ '.
            ' _...-|     |-..._ '
                   |     |
                   '.___.'
                     | |
                    _| |_
                   /\( )/\
                  /  ` '  \
                 | |     | |
                 '-'     '-'
                 | |     | |
                 | |     | |
                 | |-----| |
              .`/  |     | |/`.
              |    |     |    |
              '._.'| .-. |'._.'
                    \ | /
                    | | |
                    | | |
                    | | |
                   /| | |\
                 .'_| | |_`.
                 `. | | | .'
              .    /  |  \    .
             /o`.-'  / \  `-.`o\
            /o  o\ .'   `. /o  o\
            `.___.'       `.___.'"#;


            let response = MessageBuilder::new()
                .push_codeblock(text, None)
                .build();

            if let Err(why) = msg.channel_id.say(&context.http, &response) {
                println!("Error sending message: {:?}", why);
            }
        }
    }

    fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}

fn main() {
    // Configure the client with your Discord bot token in the environment.
    let token = env::var("DISCORD_TOKEN")
        .expect("Expected a token in the environment");
    let mut client = Client::new(&token, Handler).expect("Err creating client");

    if let Err(why) = client.start() {
        println!("Client error: {:?}", why);
    }
}
