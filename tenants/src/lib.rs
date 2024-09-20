pub struct Client {
    client: http::Client,
}

impl Client {
    pub fn new(client: http::Client) -> Self {
        Self { client }
    }

    pub fn list(&self) -> anyhow::Result<()> {
        let pagination = api::PaginationRequest::new(self.client.clone(), "tenants");
        let mut print_header = true;

        for page in pagination {
            let tenants = page?;

            if print_header {
                println!("{0: <38} {1: <50}", "ID", "NAME");
                print_header = false;
            }

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
