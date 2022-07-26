use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    study_rust::lib_main::main().await
}
