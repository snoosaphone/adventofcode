fn main() {
    let input = "ckczppom";
    // let input = "abcdef";
    // let input = "pqrstuv";

    let mut iterator = 0;

    loop {
        iterator += 1;
        
        let combo = [input, iterator.to_string().as_str()].concat();

        let result = format!("{:?}", md5::compute(combo));

        if result.starts_with("000000") {
            println!("Num: {iterator}");
            println!("Hash: {result}");
            break;
        }
    }
}
