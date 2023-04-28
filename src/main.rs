use std::{env::args, process::exit, fs::File, io::Write};

static FILE_NAME: &str = "test";

fn main() -> std::io::Result<()> {
    let args: Vec<String> = args().collect();

    if args.len() != 2 {
        println!("引数が正しくありません");
        exit(1);
    }

    let mut file = File::create(FILE_NAME).expect("file not found");
    file.write_all(b"hoge")?;

    // printf(".intel_syntax noprefix\n");
    // printf(".global main\n");
    // printf("main:\n");
    // printf("  mov rax, %d\n", atoi(argv[1]));
    // printf("  ret\n");

    exit(0);
}
