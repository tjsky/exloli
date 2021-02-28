use crate::bot::utils::{check_is_channel_admin, MessageExt};
use crate::database::Gallery;
use crate::*;
use once_cell::sync::Lazy;
use std::convert::TryInto;
use std::str::FromStr;
use teloxide::prelude::UpdateWithCx;
use teloxide::types::Message;

static EXHENTAI_URL: Lazy<regex::Regex> =
    Lazy::new(|| regex::Regex::new(r"https://e.hentai\.org/g/\d+/[0-9a-f]+/?").unwrap());

pub enum CommandError {
    /// 命令解析错误
    WrongCommand(&'static str),
    /// 不是自己的命令
    NotACommand,
}

#[derive(PartialEq, Debug)]
pub enum RuaCommand {
    // 上传指定画廊
    Upload(Vec<String>),
    // 查询指定画廊
    Query(String),
    // Ping bot
    Ping,
    // 用该命令回复一条画廊以将其删除
    Delete,
    // 按评分高低查询一段时间内的本子，格式 /best 最少几天前 最多几天前 多少本
    Best([i64; 3]),
    // 用该命令回复一条画廊以上传其完整版本
    Full(Gallery),
}

impl RuaCommand {
    /// 将消息解析为命令
    pub fn parse(message: &UpdateWithCx<Message>, bot_id: &str) -> Result<Self, CommandError> {
        use CommandError::*;

        let text = message.update.text().unwrap_or("");

        if !text.starts_with('/') {
            return Err(NotACommand);
        }

        // TODO: split_once
        let (cmd, args) = match text.find(|c| c == ' ' || c == '\n') {
            Some(pos) => (&text[1..pos], text[pos + 1..].trim()),
            _ => (&text[1..], ""),
        };
        let (cmd, bot) = match cmd.find('@') {
            Some(pos) => (&cmd[..pos], &cmd[pos + 1..]),
            None => (cmd, ""),
        };

        if !bot.is_empty() && bot != bot_id {
            return Err(NotACommand);
        }

        info!("收到命令：/{} {}", cmd, args);

        let is_admin = check_is_channel_admin(message);

        match (cmd, is_admin) {
            ("ping", _) => Ok(Self::Ping),
            ("full", true) => {
                let arg = if !args.is_empty() {
                    args.split('/')
                        .last()
                        .and_then(|s| s.parse::<i32>().ok())
                        .and_then(|id| DB.query_gallery_by_message_id(id).ok())
                } else {
                    None
                };
                let gallery = message.update.reply_to_gallery().or(arg);
                match gallery {
                    Some(v) => Ok(Self::Full(v)),
                    _ => Err(WrongCommand("用法：请回复一个需要上传的画廊")),
                }
            }
            ("delete", true) => {
                if message.update.reply_to_gallery().is_none() {
                    return Err(WrongCommand("用法：请回复一个需要删除的画廊"));
                }
                Ok(Self::Delete)
            }
            ("upload", true) => {
                let urls = EXHENTAI_URL
                    .captures_iter(message.update.text().unwrap_or_default())
                    .filter_map(|c| c.get(0).map(|m| m.as_str().to_owned()))
                    .collect::<Vec<_>>();
                if urls.is_empty() {
                    Err(WrongCommand("用法：/upload 画廊地址"))
                } else {
                    Ok(Self::Upload(urls))
                }
            }
            ("best", _) => match parse_command_best(args) {
                Some(mut v) => {
                    v[0] = v[0].min(3650);
                    v[1] = v[1].min(3650);
                    v[2] = v[2].min(20).max(-20);
                    Ok(RuaCommand::Best(v))
                }
                _ => Err(WrongCommand("用法：/best 起始时间 终止时间 最大数量")),
            },
            ("query", _) => {
                if !EXHENTAI_URL.is_match(message.update.text().unwrap_or_default()) {
                    Err(WrongCommand("用法：/query 画廊地址"))
                } else {
                    Ok(Self::Query(args.to_owned()))
                }
            }
            _ => {
                if bot == bot_id {
                    Err(WrongCommand(""))
                } else {
                    Err(NotACommand)
                }
            }
        }
    }
}

/// 将字符串解析为三个数字
fn parse_command_best(input: &str) -> Option<[i64; 3]> {
    let v = input
        .split_ascii_whitespace()
        .map(i64::from_str)
        .collect::<Result<Vec<_>, _>>()
        .ok();
    if let Some(v) = v.and_then(|v| TryInto::<[i64; 3]>::try_into(v).ok()) {
        return Some(v);
    }
    None
}