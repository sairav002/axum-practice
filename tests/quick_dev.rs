
#![allow(unused)]

use anyhow::Result;

#[tokio::test]
async fn quick_dev() -> Result<()> {
    let hc = httpc_test::new_client("http://localhost:8080")?;

    hc.do_get("/hello/Mike").await?.print().await?;

    hc.do_get("/hello?name=Jen").await?.print().await?;

    hc.do_get("/src/main.rs").await?.print().await?;

    Ok(())
}