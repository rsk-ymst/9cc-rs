use std::{env::args, fs::File, io::Write, process::exit, vec};

static FILE_NAME: &str = "tmp.s";

mod cmd;
mod file;
mod generator;
mod parser;
mod tokenizer;
mod utils;

fn main() -> anyhow::Result<()> {
    let args: Vec<String> = args().collect();

    if args.len() != 2 {
        println!("引数が正しくありません");
        exit(1);
    }

    let mut file: File = File::create(FILE_NAME).expect("file not found");

    let content_array = [
        ".intel_syntax noprefix\n",
        ".globl main\n",
        "main:\n",
        &format!("  mov rax, {}\n", args[1].parse().unwrap_or(0)),
        "  ret\n",
    ]
    .concat();

    let mut assembly_line: Vec<String> = vec![];

    for s in content_array.chars() {
        if s == '+' {
            assembly_line.push(format!("  add rax, {}\n", s));
            println!("test");
        }
    }

    let content = content_array.as_bytes();

    file.write_all(content)?;

    exit(0);
}

mod cmd_utils {
    use anyhow::*;
    use std::process::Command;

    pub fn run_cmd(command: &str, args: &[&str]) -> anyhow::Result<()> {
        let output = Command::new(command)
            .args(args)
            .output()
            .with_context(|| "output error")?;

        let print_content = String::from_utf8_lossy(match output.status.success() {
            true => &output.stdout,
            false => &output.stderr,
        });

        if !print_content.is_empty() {
            println!("{}", print_content);
        }

        Ok(())
    }

    pub fn run_cmd_status(command: &str, args: &[&str]) -> anyhow::Result<usize> {
        let output = Command::new(command)
            .args(args)
            .output()
            .with_context(|| "output error")?;

        let status_string = format!("{}", output.status);
        let split_status_string = status_string.split(' ').collect::<Vec<&str>>();
        let status = split_status_string
            .last()
            .unwrap()
            .parse::<usize>()
            .unwrap();

        Ok(status)
    }
}

#[cfg(test)]
mod tests {
    use crate::parser::Parser;
    use std::{
        io::{self, Write},
        process::Command,
    };

    use super::*;
    use anyhow::*;

    #[test]
    fn status_test_1() -> anyhow::Result<()> {
        cmd_utils::run_cmd("cargo", &["run", "--", "123"])?;
        cmd_utils::run_cmd("cc", &["-o", "tmp", "tmp.s"])?;
        cmd_utils::run_cmd("cat", &["./tmp.s"])?;

        if let anyhow::Result::Ok(status) = cmd_utils::run_cmd_status("./tmp", &[""]) {
            println!("status: {}", status);
            assert_eq!(status, 123)
        }

        Ok(())
    }

    #[test]
    fn status_test_2() {
        cmd_utils::run_cmd("cargo", &["run", "--", "43"]).unwrap();
        cmd_utils::run_cmd("cc", &["-o", "tmp", "tmp.s"]).unwrap();
        cmd_utils::run_cmd("cat", &["./tmp.s"]).unwrap();

        if let anyhow::Result::Ok(status) = cmd_utils::run_cmd_status("./tmp", &[""]) {
            println!("status: {}", status);
            assert_eq!(status, 43)
        }
    }
}
