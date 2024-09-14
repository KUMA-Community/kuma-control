use std::io::prelude::*;

pub struct Client {
    client: http::Client,
}

impl Client {
    pub fn new(client: http::Client) -> Self {
        Self { client }
    }

    pub fn list(&self) -> anyhow::Result<()> {
        let response = self.client.get_response("tenants?stream=")?;
        let buffered_reader = std::io::BufReader::new(response);
        let mut print_header = true;

        for line in buffered_reader.lines() {
            if print_header {
                println!("{0: <38} {1: <50}", "ID", "NAME");
                print_header = false;
            }

            let line = line.map_err(|e| anyhow::anyhow!("failed to read line: {}", e))?;
            let tenant: serde_json::Value = serde_json::from_str(line.trim_end())
                .map_err(|e| anyhow::anyhow!("failed to deserialize json: {}", e))?;
            let tenant = tenant.as_object().unwrap();
            let id = tenant.get("id").unwrap().as_str().unwrap();
            let name = tenant.get("name").unwrap().as_str().unwrap();
            println!("{0: <38} {1: <50}", id, name,);
        }

        Ok(())
    }

    pub fn find_by_name(&self, name: &str) -> anyhow::Result<Option<String>> {
        let url = format!("tenants?name={}", name);
        let response = self.client.get(&url)?;
        let tenants = response.as_array().unwrap();
        for tenant in tenants {
            let found_name = tenant.as_object().unwrap().get("name").unwrap();
            if found_name.as_str().unwrap() == name {
                return Ok(Some(
                    tenant
                        .as_object()
                        .unwrap()
                        .get("id")
                        .unwrap()
                        .as_str()
                        .unwrap()
                        .to_string(),
                ));
            }
        }

        Ok(None)
    }
}
