//! golden-god-bot
//! golden-god-bot
//! golden-god-bot
//! golden-god-bot

use std::env;
use std::sync::Arc;

use regex::Regex;
use dotenv::dotenv;
use serde::Deserialize;
use rand::seq::SliceRandom;

use serenity::async_trait;
use serenity::model::channel::Message;
use serenity::model::gateway::Ready;
use serenity::prelude::*;

#[derive(Deserialize)]
struct Responses {
    golden_god: Vec<String>,
    good_bot: Vec<String>
}

struct GlobalStatic;

impl TypeMapKey for GlobalStatic {
    type Value = Arc<Responses>;
}

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    // Set a handler for the `message` event - so that whenever a new message
    // is received - the closure (or function) passed will be called.
    //
    // Event handlers are dispatched through a threadpool, and so multiple
    // events can be dispatched simultaneously.
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.author.id == ctx.cache.current_user_id() {
            return; // is the bot
        }
        
        /* strip the message of non-word characters */
        let re = Regex::new(r"\W+").unwrap();
        let ms = re.replace_all(&msg.content, "");
        let msg_stripped = ms.to_ascii_lowercase();

        /* grab the strings of responses */
        let responses = {
            let data_read = ctx.data.read().await;
            data_read.get::<GlobalStatic>().expect("Expected GlobalStatic in TypeMap.").clone()
        };

        // Sending a message can fail, due to a network error, an
        // authentication error, or lack of permissions to post in the
        // channel, so log to stdout when some error happens, with a
        // description of it.
        if msg_stripped.contains("goldengod") {
            let reply = responses.golden_god.choose(&mut rand::thread_rng()).expect("[goldengod] random choice is empty");
            if let Err(why) = msg
                .channel_id
                .say(&ctx.http, reply)
                .await {
                    println!("Error sending message: {:?}", why);
            }
        }
        if msg_stripped.contains("goodbot") {
            let image = responses.good_bot.choose(&mut rand::thread_rng()).expect("[goodbot] random choice is empty");
            if let Err(why) = msg
                .channel_id
                .send_message(&ctx.http, |m| {
                    m.embed(|e| e
                        .title("You're the one that's good.")
                        .image(image)
                        .color(0xf1c40f))
                })
                .await {
                    println!("Error sending message: {:?}", why);
            }
        }
    }

    // Set a handler to be called on the `ready` event. This is called when a
    // shard is booted, and a READY payload is sent by Discord. This payload
    // contains data like the current user's guild Ids, current user data,
    // private channels, and more.
    async fn ready(&self, _: Context, ready: Ready) {
        println!("------");
        println!("{} v{}", ready.user.name, env!("CARGO_PKG_VERSION"));
        println!("ID: {}", ready.user.id);
        println!("Rust version: {}", env!("CARGO_PKG_RUST_VERSION"));
        // println!("Serenity API version: {}", serenity::prelude::);
        println!("Running on: {} ({})", env::consts::OS, env::consts::ARCH);
        println!("------");
    }
}

#[tokio::main]
async fn main() {
    /* load the environment variables */
    dotenv().ok();

    /* initialize the logger to use environment variables */
    tracing_subscriber::fmt::init();

    /* grab the client token from the environment */
    let token = env::var("DISCORD_TOKEN").expect("DISCORD_TOKEN must be set.");

    /* set gateway intents, which decides what events the bot will be notified about */
    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;
    
    /* create a new instance of the Client, logging in as a bot. */
    let mut client = Client::builder(&token, intents)
        .event_handler(Handler)
        .await
        .expect("Err creating client");

    /* set the global static data */
    {
        /* open the data lock in write mode, so keys can be inserted to it */
        let mut data = client.data.write().await;

        /* read the responses json file */
        let file = std::fs::File::open("json/responses.json").expect("File should open read-only.");
        let responses: Responses = serde_json::from_reader(file).expect("JSON was not well-formatted.");

        /* insert the data */
        data.insert::<GlobalStatic>(Arc::new(responses));
    }

    /* start a single shard, and start listening to events */
    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}