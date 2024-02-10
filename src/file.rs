use std::{fs::File, io::Write};

use crate::FILE_NAME;
use anyhow::{anyhow, Ok, Result};

// ベクタの全ての値をファイルに書き込む
pub fn write_vec_in_file(path: &str, target: Vec<String>) -> Result<()> {
    let mut file: File = File::create(FILE_NAME).expect("file not found");

    let binding = target.concat();
    let content = binding.as_bytes();

    file.write_all(content)?;

    Ok(())
}
