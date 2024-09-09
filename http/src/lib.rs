use std::error::Error;

use anyhow::anyhow;
use serde::{Deserialize, Serialize};

#[derive(Clone)]
pub struct Client {
    cfg: config::Config,
    prefix: &'static str,
}

impl Client {
    pub fn from_config() -> anyhow::Result<Client> {
        let cfg = config::Config::from_file()?;

        Ok(Client {
            cfg,
            prefix: "api/v2.2",
        })
    }

    pub fn get(&self, url: &str) -> anyhow::Result<serde_json::Value> {
        let mut response = self.get_response(url)?;
        let mut buffer = Vec::new();
        response
            .copy_to(&mut buffer)
            .map_err(|e| anyhow!("failed read response {:?}", e))?;
        let body = std::str::from_utf8(buffer.as_slice()).unwrap();

        if !response.status().is_success() {
            response
                .copy_to(&mut std::io::stdout())
                .map_err(|e| anyhow!("failed read response {}", extract_errors_chain(&e)))?;
            anyhow::bail!("{}: {}: {}", url, response.status(), body);
        }

        Ok(serde_json::from_str(body).unwrap())
    }

    pub fn post(&self, url: &str, json: serde_json::Value) -> anyhow::Result<String> {
        let url = self.build_url(url);
        let mut response = self
            .build_http_client()
            .post(url.as_str())
            .json(&json)
            .send()
            .map_err(|e| anyhow!("failed to send post request: {}", extract_errors_chain(&e)))?;

        let mut buffer = Vec::new();
        response
            .copy_to(&mut buffer)
            .map_err(|e| anyhow!("failed read response {:?}", e))?;

        let body = std::str::from_utf8(buffer.as_slice()).unwrap();

        if response.status().is_client_error() {
            anyhow::bail!("failed to import: {}: {}", url, body);
        }

        Ok(body.to_string())
    }

    pub fn get_response(&self, url: &str) -> anyhow::Result<reqwest::blocking::Response> {
        let url = self.build_url(url);
        let response = self
            .build_http_client()
            .get(url.as_str())
            .send()
            .map_err(|e| anyhow!("failed to send get request: {}", extract_errors_chain(&e)))?;

        Ok(response)
    }

    fn build_http_client(&self) -> reqwest::blocking::Client {
        reqwest::blocking::Client::builder()
            .danger_accept_invalid_certs(true)
            .default_headers(self.default_headers())
            .build()
            .unwrap()
    }

    fn default_headers(&self) -> reqwest::header::HeaderMap {
        let mut auth_value =
            reqwest::header::HeaderValue::from_str(format!("Bearer {}", self.cfg.token).as_str())
                .unwrap();
        auth_value.set_sensitive(true);
        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert("Authorization", auth_value);
        headers
    }

    fn build_url(&self, url: &str) -> String {
        format!("https://{}/{}/{}", self.cfg.url, self.prefix, url)
    }
}

#[derive(Serialize, Deserialize)]
struct Config {
    token: String,
    url: String,
}

fn extract_errors_chain(e: &dyn Error) -> String {
    let mut msg = String::new();
    if let Some(source) = e.source() {
        let smsg = extract_errors_chain(source);
        if smsg.is_empty() {
            msg = e.to_string()
        } else {
            msg += &format!("{}: {}", e, smsg);
        };
    };

    msg
}
