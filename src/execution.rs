use engine::SearchResult;
use std::process::Command;
use std::process::Output;

#[derive(Debug, Clone)]
pub enum ExecutionResult {
    Succeeded,
    Failed(String),
}

pub fn execute(cmd: String) -> ExecutionResult {
    println!("cmd = {:?}", cmd);
    match Command::new("bash")
        .arg("-c")
        .arg(&cmd)
        .spawn() {
        Ok(_) => ExecutionResult::Succeeded,
        Err(x) => ExecutionResult::Failed(format!("failed to run command {}: {}", cmd, x)),
    }
}