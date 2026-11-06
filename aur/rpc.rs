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

impl SeatchBy {}
