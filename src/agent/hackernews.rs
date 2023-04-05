use reqwest;
use scraper::{Html, Selector};
use serde::{Deserialize, Serialize};
use std::error::Error;

use super::ToolInterface;

const ENDPOINT: &str = "https://hn.algolia.com/api/v1/search_by_date";

fn extract_text_from(url: &str, max_len: usize) -> Result<String, Box<dyn Error>> {
    let html = reqwest::blocking::get(url)?.text()?;
    let document = Html::parse_document(&html);
    let text = document.root_element().text().collect::<String>();
    let lines: Vec<String> = text
        .lines()
        .map(|line| line.trim().to_string())
        .filter(|line| !line.is_empty())
        .take(max_len)
        .collect();
    Ok(lines.join("\n"))
}

fn search_hn(query: &str, crawl_urls: bool) -> Result<String, Box<dyn Error>> {
    let params = [
        ("query", query),
        ("tags", "story"),
        ("numericFilters", "points>100"),
    ];

    let response = reqwest::blocking::get(ENDPOINT)?;
    todo!("{:?}", response);
    //let json: serde_json::Value = response.json()?;
    let json: serde_json::Value = todo!();
    let hits = json["hits"].as_array().unwrap();

    let mut result = String::new();
    for hit in hits.iter().take(5) {
        let title = hit["title"].as_str().unwrap();
        let url = hit["url"].as_str();
        result.push_str(&format!("Title: {}\n", title));

        if let Some(url) = url {
            if crawl_urls {
                let excerpt = extract_text_from(url, 2000)?;
                result.push_str(&format!("\tExcerpt: {}\n", excerpt));
            }
        } else {
            let object_id = hit["objectID"].as_str().unwrap();
            let comments_url = format!(
                "{}?tags=comment,story_{}&hitsPerPage=1",
                ENDPOINT, object_id
            );
            let comments_response = reqwest::blocking::get(&comments_url)?;
            todo!("{:?}", comments_response);
            // let comments_json: serde_json::Value = comments_response.json()?;
            let comments_json: serde_json::Value = todo!();
            let comment = comments_json["hits"][0]["comment_text"].as_str().unwrap();
            result.push_str(&format!("\tComment: {}\n", comment));
        }
    }
    Ok(result)
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct HackerNewsSearchTool {
    name: String,
    description: String,
    crawl_urls: bool,
}

impl HackerNewsSearchTool {
    pub fn new() -> Self {
        HackerNewsSearchTool {
            name: "hacker news search".to_string(),
            description:
                "Get insight from hacker news users to specific search terms. Input should be a search term (e.g. How to get rich?). The output will be the most recent stories related to it with a user comment."
                    .to_string(),
            crawl_urls: false,
        }
    }
}

impl ToolInterface for HackerNewsSearchTool {
    fn name(&self) -> String {
        self.name.clone()
    }

    fn description(&self) -> String {
        self.description.clone()
    }

    fn use_tool(&self, input_text: &str) -> Result<String, Box<dyn Error>> {
        search_hn(input_text, self.crawl_urls)
    }
}
