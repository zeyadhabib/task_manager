use anyhow::Result;
use serde_json::json;

#[tokio::main]
async fn main() -> Result<()> {
    let hc = httpc_test::new_client("http://localhost:5050")?;
    hc.do_post(
        "/login",
        json!({"username": "zeyad", "password": "habib"}),
    )
    .await?
    .print()
    .await?;

    hc.do_post(
        "/login",
        json!({"username": "admin", "password": "admin"}),
    )
    .await?
    .print()
    .await?;

    hc.do_post(
        "/login",
        json!({"username": "admin", "password": "admin"}),
    )
    .await?
    .print()
    .await?;

    hc.do_post(
        "/task_manager",
        json!({"title": "zeyad", "description": "habib"}),
    )
    .await?
    .print()
    .await?;
    hc.do_post(
        "/task_manager",
        json!({"title": "admin", "description": "admin"}),
    )
    .await?
    .print()
    .await?;

    hc.do_get("/task_manager").await?.print().await?;
    hc.do_delete("/task_manager/0").await?.print().await?;
    hc.do_delete("/task_manager/1").await?.print().await?;
    hc.do_get("/task_manager").await?.print().await?;
    Ok(())
}
