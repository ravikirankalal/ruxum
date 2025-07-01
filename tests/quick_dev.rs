use httpc_test::Client;

#[tokio::test]
async fn quick_dev() -> Result<(), anyhow::Error> {
    let hc:Client = httpc_test::new_client("http://localhost:3000")?;
    hc.do_get("/hello?name=Ravi").await?.print().await?;
    Ok(())
}   