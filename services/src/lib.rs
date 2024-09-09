use colored::{ColoredString, Colorize};

pub struct Client {
    client: http::Client,
}

impl Client {
    pub fn new(client: http::Client) -> Self {
        Self { client }
    }

    pub fn list(&self) -> anyhow::Result<()> {
        let response = self.client.get("services")?;
        let services = response.as_array().unwrap();
        if !services.is_empty() {
            println!(
                "{0: <20} {1: <10} {2: <38} {3: <15} {4: <8} {5: <10}",
                "NAME", "KIND", "ID", "FQDN", "STATUS", "TENANT"
            );
            for service in services {
                let service = service.as_object().unwrap();
                let id = service.get("id").unwrap().as_str().unwrap();
                let fqdn = service.get("fqdn").unwrap().as_str().unwrap();
                let name = service.get("name").unwrap().as_str().unwrap();
                let status = service.get("status").unwrap().as_str().unwrap();
                let kind = service.get("kind").unwrap().as_str().unwrap();
                let tenant = service.get("tenantName").unwrap().as_str().unwrap();
                println!(
                    "{0: <20} {1: <10} {2: <38} {3: <15} {4: <8} {5: <10}",
                    name,
                    kind,
                    id,
                    fqdn,
                    colorize(status),
                    tenant,
                );
            }
        }
        Ok(())
    }
}

fn colorize(color_name: &str) -> ColoredString {
    let result = match color_name {
        "blue" => color_name.blue(),
        "green" => color_name.green(),
        "red" => color_name.red(),
        "yellow" => color_name.yellow(),
        "grey" => color_name.black(),
        _ => unreachable!(),
    };

    result.bold()
}
