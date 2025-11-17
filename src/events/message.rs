use itertools::Itertools;
use serenity::{all::Context, model::channel::Message};

pub async fn on_message_create(ctx: Context, msg: Message) {
    if msg.author.bot {
        return;
    };

    let reply = match &msg.content.to_lowercase()[..] {
        "hi" | "hi!" | "hello" | "hiii" | "hii" | "hi bongo" => "hii :3",
        "no" | "no." | "no," | "no?" => "yeah",
        "why" | "why?" | "why??" => "idk... im scared",
        "what" | "what?" => "idk",
        "dude" => "perfect",
        "stop" => "why",
        "gn chat" => "gn sava28",
        "kys" | "kill urself" | "kill yourself" | "kill yourself like actually" => {
            ":face_holding_back_tears:"
        }
        "i hate this" => "i know",
        "i hate you" => "im sorry",
        "omg" => "omg",
        "omfg" => "omfg",
        "lol" => "ikr",
        "idk" | "idfk" => "me either",
        "me too thanks" => "me too thanks",
        "lmfao" => "like the band",
        "wow" => "wow iphone",
        text if text.len() > 7 && text.ends_with("er") => "i hardly know her",
        text if text.len() > 4
            && text.len() < 1993
            && text
                .replace('’', "")
                .replacen(|c| c == '’' || char::is_ascii_punctuation(&c), "", 1)
                .starts_with("im ") =>
        {
            &format!("hi {}", &msg.content.split_whitespace().skip(1).join(" "))[..]
        }
        _ => return,
    };

    if let Err(why) = msg.channel_id.say(&ctx.http, reply).await {
        println!("Error sending message: {why:?}");
    }
}
