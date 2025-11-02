extern crate dotenv;

use teloxide::prelude::*;
use dotenv::dotenv;
use std::env;


#[tokio::main]
async fn main() {
    
    let (chat_id, bot) = setup_bot();
    
    send_test_message(&bot, chat_id).await;
    
}

fn setup_bot() -> (i64, Bot){
    dotenv().ok();
    
    let token = env::var("TELOXIDE_TOKEN").expect("Missing TELOXIDE_TOKEN in .env");
    let chat_id: i64 = env::var("CHAT_ID")
        .expect("Missing CHAT_ID in .env")
        .parse()
        .expect("CHAT_ID must be an integer");

    let bot = Bot::new(token);

    (chat_id, bot)
}

async fn send_test_message(bot: &Bot, chat_id: i64) {
    let result = bot
        .send_message(ChatId(chat_id), "Bot test successful 🚀")
        .send()
        .await;

    match result {
        Ok(_) => println!("✅ Message sent successfully!"),
        Err(err) => eprintln!("❌ Failed to send message: {err:?}"),
    }
}
