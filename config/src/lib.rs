use std::io::Write;
use std::os::unix::fs::OpenOptionsExt;

use anyhow::anyhow;

const CONFIG_PATH: &str = "~/.config/kuma/kc.conf";

pub fn config_path() -> String {
    let home = std::env::var("HOME").unwrap();
    let cfg_path = CONFIG_PATH.to_string();
    cfg_path.replace("~", home.as_str())
}

#[derive(serde::Serialize, serde::Deserialize, Clone)]
pub struct Config {
    pub token: String,
    pub url: String,
}

impl Config {
    pub fn interactive_init() -> anyhow::Result<()> {
        print!("Please enter kuma-core hostname: ");
        std::io::stdout().flush().unwrap();

        let mut core_url = String::new();
        std::io::stdin().read_line(&mut core_url).unwrap();
        let core_url = core_url.trim();

        print!("Please enter REST API port [7223]: ");
        std::io::stdout().flush().unwrap();

        let mut port = String::new();
        std::io::stdin().read_line(&mut port).unwrap();
        let mut port = port.trim().to_string();
        if port.is_empty() {
            port = "7223".to_string();
        }

        print!("Please enter token (input will be hidden): ");
        std::io::stdout().flush().unwrap();
        let token = rpassword::read_password().unwrap();

        let cfg = Config {
            token,
            url: format!("{}:{}", core_url, port),
        };

        let cfg = serde_json::to_string_pretty(&cfg).unwrap();
        let cfg_path = config_path();

        let dirs = std::path::Path::new(cfg_path.as_str()).parent().unwrap();
        std::fs::create_dir_all(dirs).map_err(|e| anyhow!("failed to create: {}", e))?;

        let mut fd = std::fs::OpenOptions::new()
            .create(true)
            .truncate(true)
            .write(true)
            .mode(0o600)
            .open(cfg_path.as_str())
            .map_err(|e| anyhow!("failed to open file {}: {}", cfg_path, e))?;

        fd.write_all(cfg.as_bytes())
            .map_err(|e| anyhow!("failed to write new config: {}", e))?;

        fd.flush().unwrap();

        println!("config paramters written here {}", cfg_path);

        Ok(())
    }

    pub fn from_file() -> anyhow::Result<Self> {
        let cfg_path = config_path();
        let cfg = std::fs::read_to_string(cfg_path.as_str())
            .map_err(|e| anyhow!("failed to read {}: {}", cfg_path, e))?;
        let cfg: Config = serde_json::from_str(&cfg)
            .map_err(|e| anyhow!("wrong config format {}: {}", cfg_path, e))?;

        cfg.validate()?;

        Ok(cfg)
    }

    pub fn validate(&self) -> anyhow::Result<()> {
        if self.token.is_empty() {
            anyhow::bail!("token is empty")
        }

        if self.url.is_empty() {
            anyhow::bail!("url is empty")
        }

        Ok(())
    }
}
