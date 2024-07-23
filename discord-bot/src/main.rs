use nanoid::nanoid;
use once_cell::sync::OnceCell;
use poise::{
    serenity_prelude::{self as serenity, MessageBuilder},
    CreateReply,
};
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
use tracing::{error, info};

static DB: OnceCell<DbConnection> = OnceCell::new();

#[inline]
fn u64_to_i64(num: u64) -> i64 {
    i64::from_le_bytes(num.to_le_bytes())
}

#[inline]
fn i64_to_u64(num: i64) -> u64 {
    u64::from_le_bytes(num.to_le_bytes())
}

#[derive(Clone)]
struct DbConnection {
    db: Pool<Postgres>,
}

impl DbConnection {
    pub async fn new(connection_url: String) -> anyhow::Result<Self> {
        let db = PgPoolOptions::new()
            .max_connections(5)
            .connect(&connection_url)
            .await?;

        Ok(Self { db })
    }

    pub async fn upsert_user(
        &self,
        user_id: u64,
        username: String,
        nickname: Option<String>,
    ) -> anyhow::Result<()> {
        sqlx::query!("INSERT INTO users (userid, username, nickname) VALUES ($1, $2, $3) ON CONFLICT (userid) DO UPDATE SET username = EXCLUDED.username, nickname = EXCLUDED.nickname", 
            u64_to_i64(user_id), username, nickname).execute(&self.db).await?;
        Ok(())
    }

    pub async fn upload_item(
        &self,
        user_id: u64,
        username: String,
        nickname: Option<String>,
        thumbnail_url: String,
        svg_url: String,
        title: String,
        description: String,
    ) -> anyhow::Result<()> {
        self.upsert_user(user_id, username, nickname).await?;

        sqlx::query!("INSERT INTO uploads (id, userid, title, description, svg_url, thumbnail_url) VALUES ($1, $2, $3, $4, $5, $6)", nanoid!(20), u64_to_i64(user_id), title, description, svg_url, thumbnail_url).execute(&self.db).await?;

        Ok(())
    }
}

struct UserData {}
type Context<'a> = poise::Context<'a, UserData, anyhow::Error>;

#[poise::command(slash_command)]
async fn submit_jewellery(
    ctx: Context<'_>,
    #[description = "An image of the peice of jewellery that has been made"]
    thumbnail: serenity::Attachment,
    #[description = "The SVG of the peice of art"] svg: serenity::Attachment,
    #[description = "The title of the peice of art"] title: String,
    #[description = "A short description of the peice"] description: String,
) -> anyhow::Result<()> {
    let author = ctx.author();
    let username = author.name.clone();
    let userid = author.id.get();
    let nickname = ctx.author_member().await.and_then(|user| user.nick.clone());
    let db = DB.get().expect("Could not get DB, accessed before set");

    info!("Uploaded image by {nickname:?} ({username}): \"{title}: {description}\" (thumbnail: {}, svg: {})", thumbnail.url, svg.url);
    db.upload_item(
        userid,
        username,
        nickname,
        thumbnail.url,
        svg.url,
        title,
        description,
    )
    .await?;

    ctx.send(
        ctx.reply_builder(
            CreateReply::default()
                .reply(true)
                .content("Successfully submitted your design for review by exec!"),
        ),
    )
    .await?;

    Ok(())
}

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    tracing_subscriber::fmt()
        .with_level(true)
        .with_file(true)
        .with_line_number(true)
        .init();

    let Ok(connection_url) = std::env::var("DATABASE_URL") else {
        error!("Could not get connection url, please set the POSTGRES_URL environment variable");
        return;
    };

    let Ok(discord_token) = std::env::var("DISCORD_TOKEN") else {
        error!("Could not get discord token, please set the DISCORD_TOKEN environment variable");
        return;
    };

    let db = match DbConnection::new(connection_url).await {
        Ok(val) => val,
        Err(ex) => {
            error!("Could not connect to postgres {ex}");
            return;
        }
    };

    if let Err(_) = DB.set(db) {
        error!("Attempted to set DB twice");
        return;
    };

    let intents =
        serenity::GatewayIntents::non_privileged() | serenity::GatewayIntents::MESSAGE_CONTENT;

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![submit_jewellery()],
            ..Default::default()
        })
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;

                Ok(UserData {})
            })
        })
        .build();

    let client = serenity::ClientBuilder::new(discord_token, intents)
        .framework(framework)
        .await;

    client.unwrap().start().await.unwrap();
}
