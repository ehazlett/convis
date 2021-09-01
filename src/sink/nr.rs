use std::convert::TryInto;
use anyhow::{anyhow, Result};
use reqwest::{Client as HttpClient, Method, Request, Url};
use reqwest::header::{CONTENT_TYPE, HeaderMap, HeaderValue};
use serde_json::json;
use crate::data::Record;
use super::Args;

pub struct Client {
    client:   HttpClient,
    endpoint: Url,
}

impl Client {
    pub fn new(args: Args) -> Result<Self> {
        let account = args.get("account")?;
        let key     = args.get("key")?;
        let region  = args.opt("region").unwrap_or("US");

        let host = match region.to_ascii_uppercase().as_str() {
            "US" => "insights-collector.newrelic.com",
            "EU" => "insights-collector.eu01.nr-data.net",
            _    => return Err(anyhow!("invalid region: {}", region)),
        };

        let endpoint = format!("https://{}/v1/accounts/{}/events", host, account);
        let endpoint = Url::parse(&endpoint)?;

        let mut headers = HeaderMap::new();
        headers.insert("X-Insert-Key", HeaderValue::from_str(key)?);
        headers.insert(CONTENT_TYPE, "application/json".try_into()?);

        let client = HttpClient::builder().default_headers(headers).build()?;

        Ok(Self { client, endpoint })
    }

    pub async fn send(&self, record: Record) -> Result<()> {
        let (id, name, image) = record.process.container.as_ref().map(|c| {
            (c.id.as_str(), c.name.as_str(), c.image.as_str())
        }).unwrap_or_default();

        let payload = json!([{
            "eventType":        "ContainerVisibility",
            "event":            &record.event,
            "source.ip":        record.src.ip(),
            "source.port":      record.src.port(),
            "source.host":      &record.hostname,
            "destination.ip":   record.dst.ip(),
            "destination.port": record.dst.port(),
            "process.pid":      record.process.pid,
            "process.cmd":      &record.process.command.join(" "),
            "container.id":     id,
            "container.name":   name,
            "container.image":  image,
        }]);

        let endpoint = self.endpoint.clone();
        let mut req  = Request::new(Method::POST, endpoint);
        *req.body_mut() = Some(serde_json::to_vec(&payload)?.into());

        self.client.execute(req).await?;

        Ok(())
    }
}
