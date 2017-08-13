use std::io;

#[derive(Debug)]
enum Start {
    Vowel,
    Consonant(char),
    Undefined,
}

fn main() {
    let mut input = String::new();

    let vowels = vec!['a', 'e', 'i', 'o', 'u'];

    io::stdin().read_line(&mut input)
        .expect("Failed to read input");

    let mut result = String::from(" ");

    for word in input.trim().split_whitespace() {
        let mut pig_latin_word = String::new();
        let mut start = Start::Undefined;

        for (n, c) in word.chars().enumerate() {
            match n {
                0 => {
                    if vowels.contains(&c) {
                        pig_latin_word.push(c);
                        start = Start::Vowel;
                    } else {
                        start = Start::Consonant(c);
                    }
                },
                _ => pig_latin_word.push(c),
            }
        }

        let ending = match start {
            Start::Vowel => "-hay".to_string(),
            Start::Consonant(c) => format!("-{}ay", c),
            _ => panic!("Bad error"),
        };

        pig_latin_word = pig_latin_word + &ending;

        result += &format!("{} ", &pig_latin_word);
    }

    println!("{}", result.trim());
}
