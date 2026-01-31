use anyhow::Result;

fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_env("KANATA_LOG")
                .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("info")),
        )
        .init();

    let _rt = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()?;

    tracing::info!("Kanata starting");
    Ok(())
}
