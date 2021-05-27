use std::{collections::HashMap, env, fmt, fs, io::Write, time::Instant, process};

struct BinaryString(u16);

impl fmt::Binary for BinaryString {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let val = self.0;

        fmt::Binary::fmt(&val, f)
    }
}

fn compose_case(character: char, letter: &str) -> String {    
    if character == '1' {
        return letter.to_uppercase()
    }

    letter.to_owned()
}

fn yeet(index: u16) -> String {
    let binary_string = format!("{:b}", BinaryString(index));
    
    let mut builder= String::new();

    binary_string
        .chars()
        .into_iter()
        .for_each(|character| {
            let is_first = builder.len() == 0;
            let is_last = builder.len() + 1 == binary_string.len();

            if is_first {
                return builder += &compose_case(character, "y")
            }

            if is_last {
                return builder += &compose_case(character, "t")
            }

            builder += &compose_case(character, "e")
        });

    builder
}

fn space(source: String, keys: Vec<&str>) -> String {
    let mut word: String = source.trim().to_owned();

    for key in keys {
        word = word.replace(key, &format!(" {} ", key));
    }

    word
}


fn remove(source: String, keys: Vec<&str>) -> String {
    let mut word: String = source.trim().to_owned();

    for key in keys {
        word = word.replace(key, "");
    }

    word
}


fn main() {
    let time = Instant::now();

    let filename = env::args().nth(1);

    if filename.is_none() {
        println!("File name is required");

        process::exit(1)
    }

    let filename: String = filename.unwrap();

    if !filename.ends_with(".c") && !filename.ends_with(".cpp") {
        println!("Only .c and .cpp file is supported");

        process::exit(1)
    }

    let source = fs::read_to_string(filename.to_owned());

    if source.is_err() {
        println!("Cannot read file.");
        println!("Check if file existed and permission");

        process::exit(1)
    }

    let source: String = source.unwrap();

    let special_chars: Vec<&str> = vec![";", "(", ")", "{", "}", ",", "&", "<<", ">>", "<", ">", "::", ":", "\"", "[", "]", "++", "--", "+=", "-=", ">=", "<=", "+", "-", "=", "return"];

    let mut meta_header = String::new();
    let mut total_meta: u16 = 0;

    let one_line_code: String = source
        .to_owned()
        .split("\n")
        .filter(|line| {
            if line.trim_start().starts_with("#") || line.starts_with("using") {
                total_meta += 1;

                meta_header += line;
                meta_header += "\n";

                return false
            }

            true
        })
        .collect();

    let mut code: String = source
        .to_owned()
        .split("\n")
        .enumerate()
        .filter(|(index, _)| index > &(total_meta as usize))
        .map(|(_, line)| line)
        .collect::<Vec<&str>>()
        .join("\n");

    let source = space(
        remove(one_line_code.to_owned(),vec!["\n"]),
        special_chars.to_owned()
    );

    let known_words: Vec<&str> = source
        .split(" ")
        .filter(|word| word != &"")
        .collect();

    let mut in_yeet_building_mode = false;
    
    let mut known_yeet: Vec<&str> = special_chars
        .to_owned()
        .into_iter()
        .filter(|value| value != &"\"")
        .collect();

    known_words
        .into_iter()
        .for_each(|word| {
            if special_chars.contains(&word) && word != "\"" {
                return
            }

            if !in_yeet_building_mode {
                known_yeet.push(word);
            }

            if word == "\"" {
                in_yeet_building_mode = !in_yeet_building_mode;
            }
        });

    let known_quotation_yeet: Vec<&str> = source
        .split("\"")
        .enumerate()
        .filter(|(index, _)| index % 2 == 1)
        .map(|(_, value)| {
            known_yeet.push(value);

            value
        })
        .collect();

    let mut dictionary = HashMap::<&str, String>::new();
    let mut yeet_index = 8;

    let mut yeet_defination = String::new();

    for (index, quote) in known_quotation_yeet.iter().enumerate() {
        code = code.replace(&format!("\"{}\"", quote.trim()), &format!("~yeet{}~", index));
    }

    let mut yeet_quotation_found = 0;

    for word in known_yeet.iter() {
        if word != &"\"" {
            let yeet_word: String;

            if dictionary.contains_key(word) {
                yeet_word = dictionary.get(word).unwrap().to_owned();
            } else {
                yeet_word = yeet(yeet_index);
                yeet_index += 1;
                
                dictionary.insert(word, yeet_word.to_owned());
                
                if known_quotation_yeet.contains(word) {
                    yeet_defination += &format!("#define {} \"{}\"\n", yeet_word, word.trim());
                } else {
                    yeet_defination += &format!("#define {} {}\n", yeet_word, word);
                }
            }

            if known_quotation_yeet.contains(word) {
                code = code.replace(&format!("~yeet{}~", yeet_quotation_found), &format!("{}", yeet_word.trim()));
                yeet_quotation_found += 1;
            } else  if special_chars.contains(&word) {
                code = code.replace(word, &format!(" {} ", &format!(" {} ", yeet_word)));
            } else {
                code = code.replace(&format!("{} ", &word), &format!(" {} ", yeet_word));
            }
        }
    }

    let compiled = format!("{}\n{}\n{}", meta_header, yeet_defination, code);

    let yeet_file = filename.replace(".cpp", ".yeet.cpp");
    let mut file = fs::File::create(yeet_file.to_owned())
        .expect("Cannot create file");

    file
        .write_all(compiled.as_bytes())
        .expect("Cannot write file");

    println!("Transpiled to {}", yeet_file);
    println!("Process take {} ms", time.elapsed().as_micros() as f64 / 1000 as f64)
}
