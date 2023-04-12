extern crate unescape;

use aoc_lib::*;

fn main() {
    let input_contents = get_file_contents("./input.txt");

    println!("{}", part1(input_contents.clone()));
    println!("{}", part2(input_contents));
}

fn part1(input: Vec<String>) -> usize {
    let mut sum = 0;

    for line in input {
        sum += count_code_chars(&line.clone()) - count_memory_chars(&line);
    }

    sum
}

fn part2(input: Vec<String>) -> usize {
    let mut sum = 0;

    for line in input {
        sum += encode_string_count(&line.clone()) - count_code_chars(&line);
    }

    sum
}

fn count_code_chars(line: &str) -> usize {
    line.len()
}

fn count_memory_chars(line: &str) -> usize {
    let mut escaped = 0;
    let temp = line.chars().collect::<Vec<char>>();
    let mut index = 1;

    while index < temp.len() - 1 {
        escaped += 1;
        if temp[index] == '\\' {
            if temp[index + 1] == 'x' {
                index += 4;
            } else {
                index += 2;
            }
        } else {
            index += 1;
        }
    }
    
    escaped
}

fn encode_string_count(line: &str) -> usize {
    let mut encoded_str = String::from("\"");

    for character in line.chars() {
        match character {
            '"' | '\\' => {
                encoded_str.push('\\');
                encoded_str.push(character);
            },
            _ => encoded_str.push(character)
        }
    }
    
    encoded_str.push('\"');

    encoded_str.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &[&str] = &[r#""""#, r#""abc""#, r#""aaa\"aaa""#, r#""\x27""#];

    #[test]
    fn part1_example() {
        let mut temp_string_vec: Vec<String> = Vec::new();

        for line in EXAMPLE_INPUT {
            temp_string_vec.push(line.to_string());
        }

        let result = part1(temp_string_vec);

        assert_eq!(result, 12);
    }

    #[test]
    fn code_correct_count() {
        let mut sum_result = 0;

        for line in EXAMPLE_INPUT {
            sum_result += count_code_chars(line);
        }

        assert_eq!(sum_result, 23);
    }

    #[test]
    fn memory_correct_count() {
        let mut sum_result = 0;

        for line in EXAMPLE_INPUT {
            sum_result += count_memory_chars(line);
        }

        assert_eq!(sum_result, 11);
    }

    #[test]
    fn part2_code() {
        let mut sum_result = 0;

        for line in EXAMPLE_INPUT {
            sum_result += encode_string_count(line);
        }

        assert_eq!(sum_result, 42);
    }

    #[test]
    fn diff_code_memory() {
        let mut code_count = 0;

        for line in EXAMPLE_INPUT {
            code_count += count_code_chars(line);
        }

        let mut mem_count = 0;

        for line in EXAMPLE_INPUT {
            mem_count += count_memory_chars(line);
        }

        assert_eq!(code_count - mem_count, 12);
    }

    #[test]
    fn escaped_backslash_code() {
        let input = r#""\\""#;

        let result = count_code_chars(input);

        assert_eq!(result, 4);
    }

    #[test]
    fn escaped_backslash_memory() {
        let input = r#""\\""#;

        let result = count_memory_chars(input);

        assert_eq!(result, 1);
    }

    #[test]
    fn escaped_hex_code() {
        let input = r#""\x27""#;

        let result = count_code_chars(input);
        
        assert_eq!(result, 6);
    }

    #[test]
    fn escaped_hex_memory() {
        let input = r#""\x27""#;

        let result = count_memory_chars(input);
        
        assert_eq!(result, 1);
    }
}
