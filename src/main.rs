use std::{env::args, fs::File, io::Write, process::exit};

static FILE_NAME: &str = "test3";

fn main() -> std::io::Result<()> {
    let args: Vec<String> = args().collect();

    if args.len() != 2 {
        println!("引数が正しくありません");
        exit(1);
    }

    let mut file = File::create(FILE_NAME).expect("file not found");

    let content_array = [
        ".intel_syntax noprefix\n",
        "global main\n",
        "main:\n",
        &format!("  move rax {}\n", args[1].parse().unwrap_or(0)),
        "  ret\n",
    ]
    .concat();

    let content = content_array.as_bytes();
    file.write_all(content)?;

    exit(0);
}

#[cfg(test)]
mod tests {
    use std::process::Command;

    #[test]
    fn it_works() {
        let output = Command::new("dir")
            .output()
            .expect("Failed to execute command");

        if output.status.success() {
            let stdout = String::from_utf8_lossy(&output.stdout);
            println!("Command output:\n{}", stdout);
        } else {
            let stderr = String::from_utf8_lossy(&output.stderr);
            eprintln!("Command failed with error:\n{}", stderr);
        }
    }
}
