use crate::aur::rpc::{AurClient, AurPackage, SearchBy};
use anyhow::Result;

pub async fn run(tokens: &[String], json: bool) -> Result<()> {
    let client = AurClient::new()?;
    let query = tokens.join(" ");
    let mut results = client.search(&query, SearchBy::NameDesc).await?;

    for token in tokens {
        let t = token.to_lowercase();
        results.retain(|pkg| {
            pkg.name.to_lowercase().contains(&t) || pkg.description.to_lowercase().contains(&t)
        });
    }

    results.sort_by(|a, b| score(&query, b).total_cmp(&score(&query, a)));
    if json {
        println!("{}", serde_json::to_string_pretty(&results)?);
    } else {
        for p in &results {
            println!(
                "{:<30} {:<12} [{}] {}",
                p.name,
                p.version,
                p.votes,
                p.description.lines().next().unwrap_or("")
            );
        }
    }

    Ok(())
}

fn score(query: &str, pkg: &AurPackage) -> f64 {
    let mut score = 0.0;
    let query_lower = query.to_lowercase();
    let name_lower = pkg.name.to_lowercase();
    let desc_lower = pkg.description.to_lowercase();

    if name_lower == query_lower {
        score += 1000.0;
    } else if name_lower.starts_with(&query_lower) {
        score += 500.0;
    } else if name_lower.contains(&query_lower) {
        score += 200.0;
    } else if desc_lower.contains(&query_lower) {
        score += 50.0;
    }
    score += pkg.votes as f64 * 10.0 * pkg.popularity;

    score
}
