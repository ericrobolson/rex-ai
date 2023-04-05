const FINAL_ANSWER_TOKEN: &str = "Final Answer:";
const OBSERVATION_TOKEN: &str = "Observation:";
const THOUGHT_TOKEN: &str = "Thought:";
const PROMPT_TEMPLATE: &str = "Today is {today} and you can use tools to get new information. Answer the question as best as you can using the following tools: 
{tool_description}
Use the following format:
Question: the input question you must answer
Thought: comment on what you want to do next
Action: the action to take, exactly one element of [{tool_names}]
Action Input: the input to the action
Observation: the result of the action
... (this Thought/Action/Action Input/Observation repeats N times, use it until you are sure of the answer)
Thought: I now know the final answer
Final Answer: your final answer to the original input question
Begin!
Question: {question}
Thought: {previous_responses}
";

use chrono::{DateTime, Utc};
use regex::Regex;
use std::collections::HashMap;
use std::fmt;

use crate::agent;
use crate::chatllm::ChatLLM;
use agent::ToolInterface;

enum FinishReason {
    /// API returned complete model output
    Stop,
    /// Incomplete model output due to max_tokens parameter or token limit
    Length,
    /// Omitted content due to a flag from our content filters
    ContentFilter,
    /// API response still in progress or incomplete
    Null,
}
impl FinishReason {
    fn from_str(s: &str) -> Result<Self, String> {
        match s {
            "stop" => Ok(FinishReason::Stop),
            "length" => Ok(FinishReason::Length),
            "content filter" => Ok(FinishReason::ContentFilter),
            "null" => Ok(FinishReason::Null),
            _ => Err(format!("Unknown finish reason: {}", s)),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Agent {
    llm: ChatLLM,
    tools: Vec<Box<agent::HackerNewsSearchTool>>,
    prompt_template: String,
    max_loops: usize,
    stop_pattern: Option<Vec<String>>,
}

impl Agent {
    pub fn new() -> Self {
        Self {
            llm: ChatLLM::new(),
            tools: vec![Box::new(agent::HackerNewsSearchTool::new())],
            prompt_template: PROMPT_TEMPLATE.to_string(),
            max_loops: 10,
            stop_pattern: None,
        }
    }

    fn tool_description(&self) -> String {
        self.tools
            .iter()
            .map(|tool| format!("{}: {}", tool.name(), tool.description()))
            .collect::<Vec<_>>()
            .join("\n")
    }

    fn tool_names(&self) -> String {
        self.tools
            .iter()
            .map(|tool| tool.name().clone())
            .collect::<Vec<_>>()
            .join(",")
    }

    fn tool_by_names(&self) -> HashMap<String, Box<agent::HackerNewsSearchTool>> {
        let mut map = HashMap::new();
        for tool in &self.tools {
            map.insert(tool.name().clone(), tool.clone());
        }
        map
    }

    pub fn run(&mut self, question: &str) -> Result<String, Box<dyn std::error::Error>> {
        let mut previous_responses = vec![];
        let mut num_loops = 0;
        let prompt = self
            .prompt_template
            .replace("{today}", &Utc::now().format("%Y-%m-%d").to_string())
            .replace("{tool_description}", &self.tool_description())
            .replace("{tool_names}", &self.tool_names())
            .replace("{question}", question)
            .replace("{previous_responses}", "{previous_responses}");
        println!("{}", prompt.replace("{previous_responses}", ""));

        while num_loops < self.max_loops {
            num_loops += 1;
            let curr_prompt =
                prompt.replace("{previous_responses}", &previous_responses.join("\n"));
            let (generated, tool, tool_input) = self.decide_next_action(&curr_prompt);
            if tool == "Final Answer" {
                return Ok(tool_input);
            }
            let tool_by_names = self.tool_by_names();
            let tool_instance = tool_by_names
                .get(&tool)
                .expect(&format!("Unknown tool: {}", tool));
            let tool_result = tool_instance.use_tool(&tool_input)?;
            let generated = format!(
                "{}\n{} {}\n{}",
                generated, OBSERVATION_TOKEN, tool_result, THOUGHT_TOKEN
            );
            println!("{}", generated);
            previous_responses.push(generated);
        }

        Ok("".to_string()) // Return an empty string if max_loops is reached
    }

    fn decide_next_action(&mut self, prompt: &str) -> (String, String, String) {
        let generated = self
            .llm
            .generate(prompt, self.stop_pattern.clone())
            .unwrap();
        self._parse(&generated)
    }

    fn _parse(&self, generated: &str) -> (String, String, String) {
        if generated.contains(FINAL_ANSWER_TOKEN) {
            let tool_input = generated
                .split(FINAL_ANSWER_TOKEN)
                .last()
                .unwrap()
                .trim()
                .to_string();
            return (
                generated.to_string(),
                "Final Answer".to_string(),
                tool_input,
            );
        }

        let regex = Regex::new(r"Action: [\[]?(.*?)[\]]?[\n]*Action Input:[\s]*(.*)").unwrap();
        let match_ = regex.captures(generated).expect(&format!(
            "Output of LLM is not parsable for next tool use: `{}`",
            generated
        ));

        todo!();
    }
}
