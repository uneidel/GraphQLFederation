
#[cfg(feature = "macros")]
pub mod configuration {
    #[macro_export]
    #[allow(unused_macros)]
   

    macro_rules! configmaker {
    ($($element: ident, $ty: ty, $doc:expr); *) => {

        use config::{Config, ConfigError};
        #[derive(Deserialize, Clone)]
        pub struct Settings {
            $(
                #[doc=$doc]
                pub $element: $ty
            ),*
        }
        impl Settings {
            pub fn new() -> Result<Self, ConfigError> {
                let env = std::env::var("env").unwrap_or_else(|_| "Development".into());
                let mut configpath ="./config/Development.toml";
                if !cfg!(debug_assertions) ||  env == "Production" {
                    configpath = "./Production.toml";
                }
                let settings = Config::builder()
                .add_source(config::File::with_name(configpath))
                // Eg.. `APP_DEBUG=1 ./target/app` would set the `debug` key
                .add_source(config::Environment::with_prefix("env"))
                .set_override("env", "Development")?
                .build()?
                .try_deserialize::<Settings>();


                return settings;
            }
        }
    }
    }
    pub(crate) use configmaker; 
}
