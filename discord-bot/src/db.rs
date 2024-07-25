use nanoid::nanoid;
use once_cell::sync::OnceCell;
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};

pub static DB: OnceCell<DbConnection> = OnceCell::new();

#[inline]
fn u64_to_i64(num: u64) -> i64 {
    i64::from_le_bytes(num.to_le_bytes())
}

#[inline]
fn i64_to_u64(num: i64) -> u64 {
    u64::from_le_bytes(num.to_le_bytes())
}

#[derive(Clone)]
pub struct DbConnection {
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
        avatar_url: String,
    ) -> anyhow::Result<()> {
        sqlx::query!("INSERT INTO users (userid, username, nickname, avatar_url) VALUES ($1, $2, $3, $4) ON CONFLICT (userid) DO UPDATE SET username = EXCLUDED.username, nickname = EXCLUDED.nickname", 
            u64_to_i64(user_id), username, nickname, avatar_url).execute(&self.db).await?;
        Ok(())
    }

    pub async fn upload_item(
        &self,
        id: String,
        user_id: u64,
        username: String,
        nickname: Option<String>,
        thumbnail_url: String,
        svg_url: String,
        title: String,
        description: String,
        avatar_url: String,
    ) -> anyhow::Result<()> {
        self.upsert_user(user_id, username, nickname, avatar_url)
            .await?;

        sqlx::query!("INSERT INTO uploads (id, userid, title, description, svg_url, thumbnail_url) VALUES ($1, $2, $3, $4, $5, $6)", 
            id, u64_to_i64(user_id), title, description, svg_url, thumbnail_url).execute(&self.db).await?;

        Ok(())
    }
}
