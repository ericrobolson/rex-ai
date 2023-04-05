mod agent;
mod hackernews;

pub use agent::Agent;
use std::{error::Error, fmt::Debug};

pub use hackernews::HackerNewsSearchTool;

pub trait ToolInterface: Debug + Clone {
    fn name(&self) -> String;
    fn description(&self) -> String;
    fn use_tool(&self, input_text: &str) -> Result<String, Box<dyn Error>>;
    fn format_tool(&self, result: &str) -> String {
        format!("Result from {}: {}", self.name(), result)
    }
}
