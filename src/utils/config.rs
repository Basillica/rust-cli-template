use serde::{Deserialize, Serialize};


#[derive(Debug, Serialize, Deserialize)]
pub struct AppConfig {
    app_name: String,
    defaul_path: String,
    pub db_conn_str: String,
}


impl ::std::default::Default for AppConfig {
    fn default() -> Self {
        let app_name: &'static str = env!("CARGO_PKG_NAME");
        Self {
            app_name: app_name.to_string(),
            defaul_path: "makedeveasy/rust-cli".into(),
            db_conn_str: String::new(),
        }
    }
}


impl AppConfig {
    pub fn update(&mut self, cfg: &AppConfig) {
        match confy::store(&self.app_name, None, cfg) {
            Ok(_) => println!("app config successfully updated"),
            Err(e) => println!("there was an error updating app config. error: {:?}", e)
        }
    }
}

pub trait Builder {
    fn new() -> Self;
    fn set_config_path(&mut self, config_path: &str) -> &mut Self;
    fn set_db_conn_str(&mut self, conn_str: &str) -> &mut Self;
    fn build(&self) -> AppConfig;
}

impl Builder for AppConfig{
    fn new() -> Self {
        let app_name: &'static str = env!("CARGO_PKG_NAME");
        AppConfig{
            app_name: app_name.to_string(),
            defaul_path: "".into(),
            db_conn_str: String::new(),
        }
    }

    fn set_config_path(&mut self, config_path: &str) -> &mut Self {
        self.defaul_path = config_path.into();
        self
    }

    fn set_db_conn_str(&mut self, conn_str: &str) -> &mut Self {
        self.db_conn_str = conn_str.to_string();
        self
    }

    fn build(&self) -> AppConfig {
        AppConfig{
            app_name: self.app_name.clone(),
            defaul_path: self.defaul_path.clone(),
            db_conn_str: self.db_conn_str.clone(),
        }
    }
}