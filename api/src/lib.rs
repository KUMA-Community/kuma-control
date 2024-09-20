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
}

impl Iterator for PaginationRequest {
    type Item = anyhow::Result<Vec<serde_json::Value>>;

    fn next(&mut self) -> Option<Self::Item> {
        let response = match self
            .client
            .get(format!("{}?page={}", self.suburl, self.page).as_str())
        {
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
