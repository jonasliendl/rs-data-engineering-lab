/*
This program creates a fruit salad by scrambling (shuffling) a list of fruit.
A vector is a growable array. It can grow or shrink in size and is one of the most
useful data structures in Rust. A vector is represented using the Vec<T> type.
*/

use rand::seq::SliceRandom;

fn main() {
    let fruit_options: Vec<&str> = vec![
        "Orange",
        "Fig",
        "Pomegranate",
        "Cherry",
        "Apple",
        "Pear",
        "Peach",
        "Strawberry",
        "Blueberry",
        "Pinapple"
    ];

    let mut fruit: Vec<String> = Vec::new();

    println!("Welcome to the fruit salad maker! You can use the following commands:");
    println!("add_fruit: Add a fruit to the salad");
    println!("random_fruit: Get a random fruit from the salad");
    println!("add_random_fruits: Add a defined amount of random fruits to the salad");
    println!("ingredients: List the ingredients of the salad");
    println!("exit: Exit the program");

    loop {
        println!("Please enter a command:");
        let mut command = String::new();
        std::io::stdin().read_line(&mut command).unwrap();
        match command.trim() {
            "add_fruit" => {
                println!("Please enter the name of the fruit you want to add:");
                let mut new_fruit = String::new();
                std::io::stdin().read_line(&mut new_fruit).unwrap();
                fruit.push(new_fruit.trim().to_string());
                println!("Added {} to the salad.", new_fruit.trim());
            },
            "random_fruit" => {
                let random_fruit = fruit.choose(&mut rand::thread_rng());
                match random_fruit {
                    Some(fruit) => println!("Enjoy your random fruit: {}", fruit),
                    None => println!("No fruit in the salad."),
                }
            },
            "add_random_fruits" => {
                println!("Please enter the number of random fruits that you want to add to the salad:");
                let mut rand_num = String::new();
                std::io::stdin().read_line(&mut rand_num).unwrap();
                let input: u32 = rand_num.trim().parse().expect("Input has to be a positive number.");
                let mut iterator = 0;
                while iterator < input {
                    iterator += 1;
                    match fruit_options.choose(&mut rand::thread_rng()) {
                        Some(rnd_fruit) => {
                            fruit.push(rnd_fruit.to_string());
                            println!("Added {} to the salad.", rnd_fruit);
                        },
                        None => {
                            println!("Unable to find random fruit.");
                        }
                    }
                }
            },
            "ingredients" => {
                println!("Ingredients:");
                for (i, item) in fruit.iter().enumerate() {
                    if i != fruit.len() - 1 {
                        print!("{}, ", item);
                    } else {
                        println!("{}", item);
                    }
                }
            },
            "exit" => {
                println!("Exiting the program.");
                break;
            },
            _ => {
                println!("Invalid command. Please enter a valid command.");
                continue;
            }
        }
    }
}
