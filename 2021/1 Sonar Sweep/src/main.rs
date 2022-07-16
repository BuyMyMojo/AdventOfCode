use std::ops::Add;

// ! This should load it as a csv file instead so I can load as ints but for now I'll just convert
const TEST_DATA: &str = include_str!("../input.txt");

fn main() {
    let input_str: Vec<&str> = TEST_DATA.trim().split('\n').collect();

    let mut input = vec![];

    convert_vec_to_u32(input_str, &mut input);

    let mut history = vec![];

    let mut no_distance_increase: u32 = 0;

    for data in input {
        history.push(data);
        
        // * Guard statment for first number from input.
        if history.len() < 2 {
            println!("{} (N/A - no previous measurement)", data);
            continue;
        }

        match history.last().unwrap() > history.get(history.len()-2).unwrap() {
            true => increased(history.last().unwrap(), &mut no_distance_increase),
            false => println!("{} (decreased)", history.get(history.len()-2).unwrap()),
        }
    }

    println!("Increased {} times", no_distance_increase);

}

fn increased(data: &u32, count: &mut u32) {
    println!("{} (increased)", data);
    *count = count.add(1);
}

/// Convert a vector of &str into a vec of u32
fn convert_vec_to_u32(input_str: Vec<&str>, input: &mut Vec<u32>) {
    for string_in in input_str {
        let int_out: u32 = match string_in.parse() {
            Ok(int) => int,
            Err(error) => panic!("Problem converting string to int: {:?}", error)
        };

        input.push(int_out);
    }
}