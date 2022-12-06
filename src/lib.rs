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

pub mod types {
    use std::str::FromStr;

    pub trait VecHelpers {
        fn get_first(&self) -> String;
        fn get_last(&self) -> String;
        fn pop_last(&mut self) -> String;
    }

    pub trait StringHelpers {
        fn parse_safe<T: FromStr>(&self) -> T;
    }

    impl VecHelpers for Vec<String> {
        fn get_first(&self) -> String {
            self.first().map_or_else(
                || {
                    tracing::error!("no elem found");
                    std::process::exit(1)
                },
                std::string::ToString::to_string,
            )
        }

        fn get_last(&self) -> String {
            self.last().map_or_else(
                || {
                    tracing::error!("no elem found");
                    std::process::exit(1)
                },
                std::string::ToString::to_string,
            )
        }

        fn pop_last(&mut self) -> String {
            self.pop().map_or_else(
                || {
                    tracing::error!("no elem found for pop");
                    std::process::exit(1)
                },
                |str| str,
            )
        }
    }

    impl StringHelpers for String {
        fn parse_safe<T: FromStr>(&self) -> T {
            self.parse::<T>().map_or_else(
                |_| {
                    tracing::error!("error parsing integer");
                    std::process::exit(1)
                },
                |int| int,
            )
        }
    }
}
