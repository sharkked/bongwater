use regex::{Regex, RegexBuilder, RegexSet, RegexSetBuilder};
use serenity::model::channel::Message;
use serenity::prelude::*;
use unicode_segmentation::UnicodeSegmentation;
use titlecase::titlecase;

use crate::Handler;

pub struct Patterns {
    er: Regex,
    any: RegexSet,
}

impl Patterns {
    pub fn compile() -> Self {
        let patterns = [
            r"^i['`]?m\b",
            r"([^ ]{2,}er|impostor)$",
            r"^n[-n ]*[wy ]*[o0][ o0]*\W*$",
            r"^nope\W*$",
        ];

        let rgx = |p: &str| RegexBuilder::new(p).case_insensitive(true).build();

        Patterns {
            er: rgx(patterns[1]).expect("'I'm _' regex is invalid."),
            any: RegexSetBuilder::new(&patterns)
                .case_insensitive(true)
                .build()
                .expect("Failed to compile RegexSet"),
        }
    }
}

pub async fn on_message(handler: &Handler, ctx: Context, msg: Message) {
    if !msg.author.bot {
        if let Some(response) = parse_message(handler, msg.content) {
            if let Err(why) = msg.channel_id.say(&ctx.http, response).await {
                println!("Error sending message: {:?}", why);
            }
        }
    }
}

fn capitalize(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
        None => String::new(),
    }
}

fn parse_message(handler: &Handler, msg: String) -> Option<String> {
    match &msg[..] {
        "!ping" => Some(String::from("who asked")),
        "no" => Some(String::from("Yes!")),
        _ => {
            let text = &msg[..];
            let patterns = &handler.patterns;
            let matches = patterns.any.matches(text);
            if matches.matched_any() {
                let words = msg.unicode_words().collect::<Vec<&str>>();

                if matches.matched(0) && words.len() > 1 {
                    return Some(format!("Hi {}, I'm dad!", words[1..].join(" ")))
                } else if matches.matched(1) {
                    if let Some(caps) = patterns.er.captures(text) {
                        let word = caps.get(0).map_or("", |m| m.as_str());
                        if word.len() > 2 {
                            return Some(format!("{}? I hardly know her!", titlecase(word)))
                        }
                    }
                } else if matches.matched(2) {
                    return Some(String::from("Yes!"));
                } else if matches.matched(3) {
                    return Some(String::from("Yep!"));
                }
            } 
            None
        }
    }
}
