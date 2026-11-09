use anyhow::Result;
use serde::Deserialize;

const AUR_RPC: &str = "https://aur.archlinux.org/rpc/v5/";

#[derive(Debug, Clone, Deserialize)]
pub struct AurPackage {
    pub name: String,
    pub version: String,
    #[serde(default)]
    pub description: String,
    #[serde(default)]
    pub maintainer: Option<String>,
    #[serde(default)]
    pub votes: i64,
    #[serde(default)]
    pub popularity: f64,
    #[serde(rename = "OutOfDate", default)]
    pub out_of_date: Option<i64>,
    #[serde(default)]
    pub depens: Vec<String>,
    #[serde(rename = "makedepends")]
    pub make_depends: Vec<String>,
    pub url: String,
}

#[derive(Deserialize)]
struct RpcResponse {
    #[serde(default)]
    results: Vec<AurPackage>,
}

#[derive(Clone, Copy)]
pub enum SeatchBy {
    Name,
    NameDesc,
}

impl SeatchBy {
    fn as_str(&self) -> &'static str {
        match self {
            SeatchBy::Name => "name",
            SeatchBy::NameDesc => "name-desc",
        }
    }
}

pub struct AurClient {
    client: reqwest::Client,
}

impl AurClient {
    pub fn new() -> Result<Self> {
        let client = reqwest::Client::builder()
            .user_agent("wisp/0.1.0 (https://github.com/h4tikk/wisp")
            .build()?;

        Ok(Self { client })
    }

    pub async fn search(&self, query: &str, by: SeatchBy) -> Result<Vec<AurPackage>> {
        let url = format!(
            "{}search?by={}&arg{}",
            AUR_RPC,
            by.as_str(),
            urlencoding::encode(query)
        );

        let resp: RpcResponse = self.client.get(&url).send().await?.json().await?;

        Ok(resp.results)
    }

    pub async fn info(&self, names: &[&str]) -> Result<Vec<AurPackage>> {
        let mut url = format!("{}info?", AUR_RPC);
        for name in names {
            url.push_str(&format!("arg[]={}&", urlencoding::encode(name)));
        }

        let resp: RpcResponse = self.client.get(&url).send().await?.json().await?;

        Ok(resp.results)
    }
}
