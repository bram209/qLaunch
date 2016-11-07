use engine::SearchResult;
use std::process::Command;
use std::process::Output;
use std::str;
use std::io;

#[derive(Debug, Clone)]
pub enum ExecutionResult {
    Succeeded,
    Failed(String),
    Output(String)
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

pub fn execute_and_output(cmd: String) -> Result<String, io::Error> {
    match Command::new("sh")
        .arg("-c")
        .arg(&cmd)
        .output() {
        Ok(output) => Ok( str::from_utf8(&output.stdout).unwrap().trim().to_owned()),
        Err(err) => Err(err)
    }
}