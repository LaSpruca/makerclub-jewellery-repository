use anyhow::anyhow;
use nanoid::nanoid;
use poise::serenity_prelude::{
    self as serenity, ChannelId, CreateAttachment, CreateChannel, CreateEmbed, CreateMessage,
    Message, UserId,
};
use tracing::{error, warn};

use crate::db::DB;

pub struct UserData {}
type Context<'a> = poise::Context<'a, UserData, anyhow::Error>;

#[poise::command(slash_command)]
pub async fn submit_jewellery(
    ctx: Context<'_>,
    #[description = "An image of the peice of jewellery that has been made"]
    thumbnail: serenity::Attachment,
    #[description = "The SVG of the peice of art"] svg: serenity::Attachment,
    #[description = "The title of the peice of art"] title: String,
    #[description = "A short description of the peice"] description: String,
) -> anyhow::Result<()> {
    ctx.defer_ephemeral().await.ok();

    let author = ctx.author();
    let username = author.name.clone();
    let userid = author.id.get();
    let nickname = ctx.author_member().await.and_then(|user| user.nick.clone());
    let avatar = author
        .avatar_url()
        .unwrap_or_else(|| author.default_avatar_url());

    let db = DB.get().expect("Could not get DB, accessed before set");
    let item_id = nanoid!(20);

    let message =
        send_jewellery_notification(ctx, author.id, thumbnail.url, svg.url, &title, &description)
            .await?;

    let thumbnail = match message
        .attachments
        .iter()
        .find(|item| item.filename == thumbnail.filename)
    {
        Some(item) => item,
        None => {
            error!("Could not get thumbnail from message");
            if let Err(ex) = message.delete(ctx.http()).await {
                warn!("Could not delete message {ex}");
            }
            return Err(anyhow!("Missing thumnail in message"));
        }
    };

    let svg = match message
        .attachments
        .iter()
        .find(|item| item.filename == svg.filename)
    {
        Some(item) => item,
        None => {
            error!("Could not get thumbnail from message");
            if let Err(ex) = message.delete(ctx.http()).await {
                warn!("Could not delete message {ex}");
            }
            return Err(anyhow!("Missing svg in message"));
        }
    };

    if let Err(ex) = db
        .upload_item(
            item_id,
            userid,
            username,
            nickname,
            thumbnail.url.clone(),
            svg.url.clone(),
            title,
            description,
            avatar,
        )
        .await
    {
        error!("Could not upload item to db {ex}");
        if let Err(ex) = message.delete(ctx.http()).await {
            warn!("Could not delete message {ex}");
        }
        Err(ex)?;
        unreachable!();
    }

    ctx.reply("Item sent to execs for review!").await?;

    Ok(())
}

async fn send_jewellery_notification(
    ctx: Context<'_>,
    user_id: UserId,
    thumbnail_url: String,
    svg_url: String,
    title: &str,
    description: &str,
) -> anyhow::Result<Message> {
    let channel_id_str = match std::env::var("MESSAGE_CHANNEL_ID") {
        Ok(val) => val,
        Err(ex) => {
            error!("Could not get message channel id, Please set MESSAGE_CHANNEL_ID environment variable!");
            Err(ex)?;
            unreachable!();
        }
    };

    let channel_id = match channel_id_str.parse::<u64>() {
        Ok(val) => ChannelId::new(val),
        Err(ex) => {
            error!("MESSAGE_CHANNEL_ID is not a valid u64 {channel_id_str:?}");
            Err(ex)?;
            unreachable!();
        }
    };

    let (thumbnail, svg) = match futures::join!(
        CreateAttachment::url(ctx.http(), &thumbnail_url),
        CreateAttachment::url(ctx.http(), &svg_url)
    ) {
        (Ok(a), Ok(b)) => (a, b),
        (Err(a), Err(b)) => {
            error!("Could not download thumbnail {a}");
            error!("Could not download svg {b}");
            Err(a)?;
            unreachable!();
        }
        (Err(a), _) => {
            error!("Could not download thumbnail {a}");
            Err(a)?;
            unreachable!();
        }
        (_, Err(a)) => {
            error!("Could not download svg {a}");
            Err(a)?;
            unreachable!();
        }
    };

    match channel_id
        .send_files(
            ctx.http(),
            vec![thumbnail, svg],
            CreateMessage::new().embed(CreateEmbed::new().title("Submission uploaded").fields([
                ("Author", format!("<@{}>", user_id.get()), false),
                ("Title", title.to_string(), false),
                ("Description", description.to_string(), false),
            ])),
        )
        .await
    {
        Ok(val) => return Ok(val),
        Err(ex) => {
            error!("Could not send message about upload");
            Err(ex)?;
            unreachable!();
        }
    };
}
