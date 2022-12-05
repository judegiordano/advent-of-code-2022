pub mod utils {
    use tracing_subscriber::FmtSubscriber;

    pub fn init_logger() {
        if cfg!(debug_assertions) {
            use dotenv::dotenv;
            dotenv().ok();
        }
        let level = std::env::var("LOG_LEVEL").map_or(tracing::Level::ERROR, |found| {
            match found.trim().to_uppercase().as_ref() {
                "INFO" => tracing::Level::INFO,
                "DEBUG" => tracing::Level::DEBUG,
                "WARN" => tracing::Level::WARN,
                "TRACE" => tracing::Level::TRACE,
                _ => tracing::Level::ERROR,
            }
        });
        match tracing::subscriber::set_global_default(
            FmtSubscriber::builder().with_max_level(level).finish(),
        ) {
            Ok(_) => (),
            Err(err) => {
                tracing::error!("{err}");
                std::process::exit(1)
            }
        }
    }
}
