use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

pub fn get_file_contents(filepath: &str) -> Vec<String> {
    let file_handle = File::open(filepath).unwrap();
    let reader = BufReader::new(file_handle);
    let mut contents: Vec<String> = Vec::new();

    for line in reader.lines() {
        contents.push(line.unwrap());
    }

    return contents;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn file_read() {
        let result = get_file_contents("src/test_content/test.txt");

        assert_eq!(result[0], "blargh");
    }

    #[test]
    fn file_read_multiline() {
        let result = get_file_contents("src/test_content/test_multiline.txt");

        assert_eq!(result[0], "blarghen");
        assert_eq!(result[1], "woot");
    }

    #[test]
    #[should_panic]
    fn file_notfound() {
        let _result = get_file_contents("./does_not_exist.txt");
    }
}
