/*
This example code counts the frequency of each number in the vector.
 */
use std::{borrow::Borrow, collections::HashMap};

fn logic(numbers: Vec<i32>) -> Vec<(i32, u32)> {
    let mut frequencies = HashMap::new();

    for num in numbers {
        let frequency = frequencies.entry(num).or_insert(0);
        *frequency += 1;
    }

    let mut result = Vec::new();

    for (num, frequency) in frequencies {
        result.push((num, frequency));
    }

    result
}

fn get_frequency_of_words(sentence: String) -> Vec<(String, u32)> {
    let mut frequencies: HashMap<String, u32> = HashMap::new();

    for (_, word) in sentence.trim().to_lowercase().split(" ").into_iter().enumerate() {
        let frequency = frequencies.entry(String::from(word)).or_insert(0);
        *frequency += 1;
    }

    let mut result: Vec<(String, u32)> = Vec::new();

    for (key, value) in frequencies {
        result.push((key, value));
    }

    result
}

fn read_input_nums() -> Vec<i32> {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Please an array of numbers.");

    let mut numbers = Vec::new();

    for (_, value) in input.split(" ").into_iter().enumerate() {
        println!("{}", value);
        let num: i32 = value.trim().parse::<i32>().expect("An error occurred while parsing a string to an integer");
        numbers.push(num);
    }

    numbers
}

fn main() {
    // let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 1, 3];
    // println!("Please enter a list of numbers. Seperate them by using a space character.");

    // let input = read_input();

    // let result = logic(input);

    println!("Please enter a sentence");

    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Please enter something");

    let mut result = get_frequency_of_words(input);
    result.sort_by(|a, b| b.1.cmp(&a.1));

    //print the results in a human readable format that explains what the result is.
    println!(
        "The frequency of each word in the vector is: {:?}",
        result
    );
}

// Test sentence: This should be a very nice and very very very long sentence. You can see this 
