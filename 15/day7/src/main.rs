use aoc_lib::*;
use std::collections::HashMap;

fn main() {
    let input_contents = get_file_contents("./input.txt");

    println!("{:?}", day1(input_contents).get("a"));
    // println!("{}", day2(input_contents));
}

fn day1(input: Vec<String>) -> HashMap<String, u16> {
    let mut circuit_mapping: HashMap<String, u16> = HashMap::new();

    let mut mut_input = input.clone();

    while !mut_input.is_empty() {
        let input_copy = mut_input.clone();
        for line in input_copy.clone() {
            if parse_circuit_line(line.clone(), &mut circuit_mapping) {
                mut_input.retain(|value| *value != line);
                // println!("{line}");
                // println!("{:?}", circuit_mapping);
                // println!("{}", input_copy.len());
                // println!("{}", mut_input.len());
            }
        }
        // println!("{}", mut_input.len());
        // for entry in circuit_mapping.iter() {
        //     println!("{entry:?}");
        // }
    }

    return circuit_mapping;
}

// fn day2(_input: Vec<String>) - > u16 {
//
// }

fn parse_circuit_line(line: String, circuit_mapping: &mut HashMap<String, u16>) -> bool {
    let split: Vec<&str> = line.split(" -> ").collect();
    let instruction = split[0].to_string();
    let destination = split[1].to_string();

    let parsed_instruction: Vec<&str> = instruction.split(" ").collect();
    let parsed_len = parsed_instruction.len();

    if parsed_len == 1 {
        let source = parsed_instruction[0];

        if let Some(wire) = circuit_mapping.get(source) {
            let value = wire.clone();
            // println!(
            //     "Setting signal for {destination} from {}",
            //     parsed_instruction[0]
            // );
            // println!("{:?}", circuit_mapping.entry(destination));
            circuit_mapping.entry(destination).or_insert(value);
            return true;
        }
        if let Ok(signal_value) = source.parse::<u16>() {
            // println!("Setting signal for {destination}");
            circuit_mapping
                .entry(destination)
                .or_insert(signal_value);
            return true;
        }

        return false;
    } else if parsed_len == 2 {
        let source = parsed_instruction[1];
        if let Some(value) = circuit_mapping.get(source) {
            let notted_value = value.clone();
            circuit_mapping
                .entry(destination.to_string())
                .or_insert(!notted_value);
            return true;
        } else {
            return false;
        }
    } else if parsed_len == 3 {
        return parse_gate(&instruction, destination, circuit_mapping);
    } else {
        println!("Instruction unknown: {instruction}");
        return false;
    }
}

fn parse_gate<'a>(
    gate: &str,
    destination: String,
    circuit_mapping: &mut HashMap<String, u16>,
) -> bool {
    let mut parsed_gate = gate.split(" ");

    let operand_one = parsed_gate.next().unwrap();
    let gate_type = parsed_gate.next().unwrap();
    let operand_two = parsed_gate.next().unwrap();

    let lhs: u16;
    let rhs: u16;

    if let Some(i) = circuit_mapping.get(operand_one) {
        lhs = *i;
    } else if let Ok(i) = operand_one.parse::<u16>() {
        lhs = i;
    } else {
        return false;
    }

    if let Some(i) = circuit_mapping.get(operand_two) {
        rhs = *i;
    } else if let Ok(i) = operand_two.parse::<u16>() {
        rhs = i;
    } else {
        return false;
    }

    if gate_type == "RSHIFT" {
        let shift_value = lhs >> rhs;
        circuit_mapping
            .entry(destination.to_string())
            .or_insert(shift_value);
        return true;
    }
    if gate_type == "LSHIFT" {
        let shift_value = lhs << rhs;
        circuit_mapping
            .entry(destination.to_string())
            .or_insert(shift_value);
        return true;
    }
    if gate_type == "AND" {
        let and_value = lhs & rhs;
        circuit_mapping
            .entry(destination.to_string())
            .or_insert(and_value);
        return true;
    } 
    if gate_type == "OR" {
        let or_value = lhs | rhs;
        circuit_mapping
            .entry(destination.to_string())
            .or_insert(or_value);
        return true;
    }

    return false;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day1_example() {
        let mut input = Vec::new();
        input.push("123 -> x".to_string());
        input.push("456 -> y".to_string());
        input.push("x AND y -> d".to_string());
        input.push("x OR y -> e".to_string());
        input.push("x LSHIFT 2 -> f".to_string());
        input.push("y RSHIFT 2 -> g".to_string());
        input.push("NOT x -> h".to_string());
        input.push("NOT y -> i".to_string());

        let result = day1(input);

        assert_eq!(*result.get("x").unwrap(), 123u16);
        assert_eq!(*result.get("y").unwrap(), 456u16);
        assert_eq!(*result.get("d").unwrap(), 72u16);
        assert_eq!(*result.get("e").unwrap(), 507u16);
        assert_eq!(*result.get("f").unwrap(), 492u16);
        assert_eq!(*result.get("g").unwrap(), 114u16);
        assert_eq!(*result.get("h").unwrap(), 65412u16);
        assert_eq!(*result.get("i").unwrap(), 65079u16);
    }

    #[test]
    fn day1_outoforder() {
        let mut input = Vec::new();
        input.push("123 -> x".to_string());
        input.push("x AND y -> d".to_string());
        input.push("x OR y -> e".to_string());
        input.push("x LSHIFT 2 -> f".to_string());
        input.push("y RSHIFT 2 -> g".to_string());
        input.push("456 -> y".to_string());
        input.push("NOT x -> h".to_string());
        input.push("NOT y -> i".to_string());

        let result = day1(input);

        assert_eq!(*result.get("x").unwrap(), 123u16);
        assert_eq!(*result.get("y").unwrap(), 456u16);
        assert_eq!(*result.get("d").unwrap(), 72u16);
        assert_eq!(*result.get("e").unwrap(), 507u16);
        assert_eq!(*result.get("f").unwrap(), 492u16);
        assert_eq!(*result.get("g").unwrap(), 114u16);
        assert_eq!(*result.get("h").unwrap(), 65412u16);
        assert_eq!(*result.get("i").unwrap(), 65079u16);
    }

    #[test]
    fn day1_outoforder_withwireassignal() {
        let mut input = Vec::new();
        input.push("123 -> x".to_string());
        input.push("x AND y -> d".to_string());
        input.push("x OR y -> e".to_string());
        input.push("x LSHIFT 2 -> f".to_string());
        input.push("y RSHIFT 2 -> g".to_string());
        input.push("456 -> y".to_string());
        input.push("ah RSHIFT 15 -> j".to_string());
        input.push("NOT x -> h".to_string());
        input.push("NOT y -> i".to_string());
        input.push("ah -> i".to_string());
        input.push("65533 -> ah".to_string());

        let result = day1(input);

        assert_eq!(*result.get("x").unwrap(), 123u16);
        assert_eq!(*result.get("y").unwrap(), 456u16);
        assert_eq!(*result.get("d").unwrap(), 72u16);
        assert_eq!(*result.get("e").unwrap(), 507u16);
        assert_eq!(*result.get("f").unwrap(), 492u16);
        assert_eq!(*result.get("g").unwrap(), 114u16);
        assert_eq!(*result.get("h").unwrap(), 65412u16);
        assert_eq!(*result.get("i").unwrap(), 65079u16);
        assert_eq!(*result.get("ah").unwrap(), 65533u16);
        assert_eq!(*result.get("j").unwrap(), 1u16);
    }

    #[test]
    fn parse_testline() {
        let input = "123 -> a".to_string();
        let mut test_mapping: HashMap<String, u16> = HashMap::new();
        parse_circuit_line(input, &mut test_mapping);

        assert_eq!(test_mapping, HashMap::from([("a".to_string(), 123u16)]));
    }
}
