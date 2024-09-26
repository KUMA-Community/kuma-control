pub const NOT_SPECIFIED_TENANT_MSG: &str =
    "tenant is not specified, please specify it like this 'kc assets import tenant=Main ...'";
pub struct Client {
    client: http::Client,
}

impl Client {
    pub fn new(client: http::Client) -> Self {
        Self { client }
    }

    pub fn list(&self) -> anyhow::Result<()> {
        let pagination = api::PaginationRequest::new(self.client.clone(), "assets?preview=");
        let mut print_header = true;

        for page in pagination {
            let assets = page?;

            if print_header {
                println!("{0: <38} {1: <15} {2: <40}  FQDN", "ID", "TENANT", "NAME");
                print_header = false;
            }

            for asset in assets {
                let asset = asset.as_object().unwrap();
                let id = asset.get("id").unwrap().as_str().unwrap();
                let fqdns = asset.get("fqdn").unwrap().as_array().unwrap();
                let name = asset.get("name").unwrap().as_str().unwrap();
                let tenant_name = asset.get("tenantName").unwrap().as_str().unwrap();

                let fqdns = if fqdns.is_empty() {
                    "-".to_string()
                } else {
                    fqdns
                        .iter()
                        .map(|a| a.as_str().unwrap())
                        .collect::<Vec<&str>>()
                        .join(", ")
                };

                println!(
                    "{0: <38} {1: <15} {2: <40} {3}",
                    id, tenant_name, name, fqdns
                );
            }
        }

        Ok(())
    }

    pub fn import_from_fields(&self, tenant_id: &str, fields: &[String]) -> anyhow::Result<()> {
        let mut assets: Vec<serde_json::Value> = Vec::new();
        let mut asset: Option<serde_json::Value> = None;

        if fields.is_empty() {
            anyhow::bail!("fields is not specified");
        }

        for field in fields {
            if field.starts_with("name=") {
                if let Some(asset) = asset {
                    assets.push(asset);
                }

                asset = Some(serde_json::json!({}));
                asset.as_mut().unwrap().as_object_mut().unwrap().insert(
                    "name".to_string(),
                    serde_json::Value::String(field.strip_prefix("name=").unwrap().to_string()),
                );

                continue;
            }

            if asset.is_none() {
                anyhow::bail!(
                    "name must be first field but '{}' is specified instead",
                    field
                );
            }

            if field.starts_with("owner=") {
                asset.as_mut().unwrap().as_object_mut().unwrap().insert(
                    "owner".to_string(),
                    serde_json::Value::String(field.strip_prefix("owner=").unwrap().to_string()),
                );
            } else if field.starts_with("ipAddresses=") {
                let addresses: Vec<serde_json::Value> = field
                    .strip_prefix("ipAddresses=")
                    .unwrap()
                    .split(",")
                    .map(|v| v.to_string().into())
                    .collect();
                asset.as_mut().unwrap().as_object_mut().unwrap().insert(
                    "ipAddresses".to_string(),
                    serde_json::Value::Array(addresses),
                );
            } else if field.starts_with("fqdn=") {
                let addresses: Vec<serde_json::Value> = field
                    .strip_prefix("fqdn=")
                    .unwrap()
                    .split(",")
                    .map(|v| v.to_string().into())
                    .collect();
                asset
                    .as_mut()
                    .unwrap()
                    .as_object_mut()
                    .unwrap()
                    .insert("fqdn".to_string(), serde_json::Value::Array(addresses));
            } else {
                anyhow::bail!("unknown field '{}'", field);
            }
        }

        if let Some(asset) = asset {
            assets.push(asset);
        }

        let request = serde_json::json!({
            "tenantID": tenant_id,
            "assets": assets,
        });

        self.client.post("assets/import", request)?;

        Ok(())
    }

    pub fn import_from_input(
        &self,
        tenant_id: Option<String>,
        input: std::io::Stdin,
    ) -> anyhow::Result<()> {
        let mut data: serde_json::Value = serde_json::from_reader(input)
            .map_err(|e| anyhow::anyhow!("failed to read data from stdin: {:?}", e))?;

        let request = if data.is_array() {
            if tenant_id.is_none() {
                anyhow::bail!("{}", NOT_SPECIFIED_TENANT_MSG);
            }

            serde_json::json!({
                "tenantID": tenant_id,
                "assets": data,
            })
        } else if data.is_object() {
            if data.as_object().unwrap().get("assets").is_some() {
                if let Some(tenant_id) = tenant_id {
                    data.as_object_mut()
                        .unwrap()
                        .insert("tenantID".to_string(), tenant_id.into());
                }
                data
            } else {
                if tenant_id.is_none() {
                    anyhow::bail!("{}", NOT_SPECIFIED_TENANT_MSG);
                }

                serde_json::json!({
                    "tenantID": tenant_id,
                    "assets": serde_json::Value::Array(vec![data]),
                })
            }
        } else {
            anyhow::bail!("unrecognize data type");
        };

        self.client.post("assets/import", request)?;

        Ok(())
    }
}
