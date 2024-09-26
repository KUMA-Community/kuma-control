pub struct PaginationRequest {
    client: http::Client,
    suburl: String,
    page: u32,
}

impl PaginationRequest {
    pub fn new(client: http::Client, suburl: &str) -> Self {
        Self {
            client,
            suburl: suburl.to_string(),
            page: 1,
        }
    }

    fn with_param(&self, param: &str) -> String {
        let p = if self.suburl.contains("?") { "&" } else { "?" };
        format!("{}{}{}", self.suburl, p, param)
    }
}

impl Iterator for PaginationRequest {
    type Item = anyhow::Result<Vec<serde_json::Value>>;

    fn next(&mut self) -> Option<Self::Item> {
        let page = format!("page={}", self.page);
        let url = self.with_param(page.as_str());
        let response = match self.client.get(url.as_str()) {
            Ok(response) => response,
            Err(err) => {
                return Some(Err(anyhow::anyhow!(
                    "faled to fetch next page {}: {}: {:?}",
                    self.page,
                    self.suburl,
                    err,
                )));
            }
        };

        self.page += 1;

        if response.as_array().unwrap().is_empty() {
            None
        } else {
            Some(Ok(response.as_array().unwrap().clone()))
        }
    }
}
