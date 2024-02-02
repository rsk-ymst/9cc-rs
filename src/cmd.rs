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

    println!("status: {split_status_string:#?}");

    let status = split_status_string
        .last()
        .unwrap()
        .parse::<usize>()
        .map_err(|e| anyhow!("parse error in run_cmd_status {e}"))?;

    Ok(status)
}

// アセンブリファイルをアセンブルするコマンドを打つ関数
pub fn assemble(target: &str, out_name: &str) -> anyhow::Result<()> {
    run_cmd("cc", &["-o", out_name, target])?;
    Ok(())
}

pub fn cat(target: &str) -> anyhow::Result<()> {
    run_cmd("cat", &[target])?;
    Ok(())
}

pub fn run(target: &str) -> anyhow::Result<usize> {
    run_cmd_status(&format!("./{target}"), &[])
}

pub fn run_assembly(target: &str) -> anyhow::Result<usize> {
    let bin = target.split('.').collect::<Vec<&str>>().into_iter().next().unwrap();
    println!("{bin}");
    assemble(target, bin).map_err(|e| anyhow!("assemble error in run_assembly {e:#?}"))?;
    run(bin).map_err(|e| anyhow!("run error in run_assembly {e:#?}"))
}


#[cfg(test)]
mod tests {
    use crate::parser::Parser;
    use crate::cmd;

    use std::{
        io::{self, Write},
        process::Command,
    };

    use super::*;

    #[test]
    fn status_test_1() -> anyhow::Result<()> {
        run_cmd("cargo", &["run", "--", "123"])?;
        run_cmd("cc", &["-o", "tmp", "tmp.s"])?;
        run_cmd("cat", &["./tmp.s"])?;

        if let anyhow::Result::Ok(status) = cmd::run_cmd_status("./tmp", &[""]) {
            println!("status: {}", status);
            assert_eq!(status, 123)
        }

        Ok(())
    }

    #[test]
    fn assemble_test() -> anyhow::Result<()>  {
        assemble("tmp.s", "tmp")?;
        cat("tmp.s")
    }

    #[test]
    fn run_bin_test() {
        let status = run("./tmp").expect("run_bin_tes err");
        println!("{}", status);
    }

    #[test]
    fn run_bin_test2() {
        assemble("tmp.s", "tmp").unwrap();
        cat("tmp.s");
        run("./tmp").expect("run_err");
    }
}
