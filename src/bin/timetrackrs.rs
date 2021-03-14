#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let db = sqlx::sqlite::SqlitePoolOptions::new()
        .connect("sqlite://foo.sqlite3")
        .await?;

    let q = sqlx::query!("select count(*) as coint from foo.events")
        .fetch_one(&db)
        .await?;
    println!("qq = {:?}", q);

    Ok(())
}
