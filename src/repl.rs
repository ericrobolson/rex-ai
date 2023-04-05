use clap::Parser;
use easy_repl::anyhow::Context;
use easy_repl::{command, CommandStatus, Repl};
use serde::{Deserialize, Serialize};
use std::env;
use std::error::Error;
use std::io;
use std::time::Duration;

use crate::agent::Agent;
pub fn run() -> Result<(), Box<dyn Error>> {
    println!("Enter your question:");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    println!("You entered: {}", input);

    let mut agent = Agent::new();
    let response = agent.run(&input)?;
    println!("Response >>>>>>> \n\n{}", response);
    println!("");

    Ok(())
}
