use std::{env::args, fs::File, io::Write, process::exit};

static FILE_NAME: &str = "tmp.s";

fn main() -> std::io::Result<()> {
    let args: Vec<String> = args().collect();

    if args.len() != 2 {
        println!("引数が正しくありません");
        exit(1);
    }

    let mut file = File::create(FILE_NAME).expect("file not found");

    let content_array = [
        ".intel_syntax noprefix\n",
        ".globl main\n",
        "main:\n",
        &format!("  mov rax, {}\n", args[1].parse().unwrap_or(0)),
        "  ret\n",
    ]
    .concat();

    let content = content_array.as_bytes();
    file.write_all(content)?;

    exit(0);
}

mod cmd_utils {
    use std::process::Command;

    pub fn run_cmd(command: &str, args: &[&str]) -> Result<(), String> {
        let output = Command::new(command)
            .args(args)
            .output()
            .map_err(|e| format!("Failed to execute command: {}", e))?;

        let print_content = String::from_utf8_lossy(match output.status.success() {
            true => &output.stdout,
            false => &output.stderr,
        });

        if !print_content.is_empty() {
            println!("{}", print_content);
        }

        Ok(())
    }

    pub fn run_cmd_status(command: &str, args: &[&str]) -> Result<usize, String> {
        let output = Command::new(command)
            .args(args)
            .output()
            .map_err(|e| format!("Failed to execute command: {}", e))?;

        let status_string = format!("{}", output.status);
        let split_status_string = status_string.split(' ').collect::<Vec<&str>>();
        let status = split_status_string.last().unwrap().parse::<usize>().unwrap();

        Ok(status)
    }

    // pub fn run_cmd_with_stdout(command: &str, args: &[&str]) -> std::io::Result<()> {
    //     let output = Command::new(command)
    //         .args(args)
    //         .output()
    //         .expect("Failed to execute command");

    //     if !output.status.success() {
    //         let stdout = String::from_utf8_lossy(&output.stdout);
    //         println!("Command output:\n{}", stdout);
    //     } else {
    //         let stderr = String::from_utf8_lossy(&output.stderr);
    //         eprintln!("Command failed with error:\n{}", stderr);
    //     }

    //     Ok(())
    // }
}

#[cfg(test)]
mod tests {
    use std::{process::Command, io::{self, Write}};
    use super::*;

    #[test]
    fn status_test_1() {
        cmd_utils::run_cmd("cargo", &["run", "--", "123"]).unwrap();
        cmd_utils::run_cmd("cc", &["-o", "tmp", "tmp.s"]).unwrap();
        cmd_utils::run_cmd("cat", &["./tmp.s"]).unwrap();

        if let Ok(status) = cmd_utils::run_cmd_status("./tmp", &[""]) {
            println!("status: {}", status);
            assert_eq!(status, 123)
        }
    }

    #[test]
    fn status_test_2() {
        cmd_utils::run_cmd("cargo", &["run", "--", "43"]).unwrap();
        cmd_utils::run_cmd("cc", &["-o", "tmp", "tmp.s"]).unwrap();
        cmd_utils::run_cmd("cat", &["./tmp.s"]).unwrap();

        if let Ok(status) = cmd_utils::run_cmd_status("./tmp", &[""]) {
            println!("status: {}", status);
            assert_eq!(status, 43)
        }
    }
}
