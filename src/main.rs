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

// pub fn atoi(hoge: String) -> &str{
//     hoge.
// }
