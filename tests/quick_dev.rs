#![allow(unused)]

use anyhow::Result;

#[tokio::test]
async fn quick_dev() -> Result<()> {
    const BASE_URL: &str = "http://localhost:8080";
    let hc = httpc_test::new_client(BASE_URL)?;

    hc.do_get("/hello?name=David").await?.print().await?;

    hc.do_get("/hello2/Sam").await?.print().await?;

    Ok(())
}