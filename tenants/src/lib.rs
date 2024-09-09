pub struct Client {
    client: http::Client,
}

impl Client {
    pub fn new(client: http::Client) -> Self {
        Self { client }
    }

    pub fn list(&self) -> anyhow::Result<()> {
        let response = self.client.get("tenants")?;
        let tenants = response.as_array().unwrap();

        if !tenants.is_empty() {
            println!("{0: <38} {1: <50}", "ID", "NAME");
            for tenant in tenants {
                let tenant = tenant.as_object().unwrap();
                let id = tenant.get("id").unwrap().as_str().unwrap();
                let name = tenant.get("name").unwrap().as_str().unwrap();
                println!("{0: <38} {1: <50}", id, name,);
            }
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
                    tenant.as_object().unwrap().get("id").unwrap().to_string(),
                ));
            }
        }

        Ok(None)
    }
}
