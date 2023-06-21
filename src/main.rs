use std::io;

fn main() {
    println!("Welcome to this simple latin to pig latin language translator!");

    loop {
        let mut input = String::new();

        println!("Please input your set of not latin characters to be converted:");

        io::stdin()
            .read_line(&mut input)
            .expect("Enter a valid input!");

        println!("Your input is: {input}");

        let input_chars = input.trim_end().chars();

        let first_char = input_chars
            .clone()
            .nth(0)
            .clone()
            .unwrap_or_else(|| panic!("Empty input string!"));

        let first_char_is_vowel = match first_char {
            'a' | 'e' | 'i' | 'o' | 'u' | 'A' | 'E' | 'I' | 'O' | 'U' => true,
            _ => false,
        };

        if first_char_is_vowel {
            let mut output = String::new();

            output.push_str(input_chars.as_str());

            output.push_str("-hay");

            println!(
                "The pig latin translation of your input string is: {}",
                output
            );
        } else {
            let mut output = String::new();

            for (index, char) in input_chars.enumerate() {
                if index == 0 {
                    continue;
                }

                output.push(char);
            }

            output.push('-');

            output.push(first_char);

            output.push_str("ay");

            println!(
                "The pig latin translation of your input string is: {}",
                output
            );
        }
    }
}
