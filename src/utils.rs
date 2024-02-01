
use anyhow::{anyhow, Result};


// 全ての要素に対して改行を入れる関数
pub fn vec_plus_n(asm_line: Vec<String>) -> Result<Vec<String>> {

    let x = asm_line.into_iter().map(|mut string| {
        string.push('\n');
        string
    }).collect::<Vec<String>>();

    Ok(x)
}

mod tests {
    use super::vec_plus_n;

    #[test]
    fn test_hoge() {
        let x = vec!["hoge".to_owned(), "test".to_owned(), "bar".to_owned()];

        assert_eq!(
            vec!["hoge\n".to_owned(), "test\n".to_owned(), "bar\n".to_owned()],
            vec_plus_n(x).unwrap()
        )
    }
}
