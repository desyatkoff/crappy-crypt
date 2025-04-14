use std::io::{
    BufReader,
    BufWriter,
    Read,
    Write,
};

fn input(query: &str) -> String {
    print!("{query}");

    std::io::stdout()
        .flush()
        .unwrap();

    let mut user_input = String::new();

    std::io::stdin()
        .read_line(&mut user_input)
        .unwrap();

    return user_input
        .trim()
        .to_owned();
}

fn process_data(data: &Vec<u8>, key: u8) -> Vec<u8> {
    let mut processed_data = Vec::with_capacity(data.len());

    for byte in data {
        processed_data.push(byte ^ key);
    }

    return processed_data;
}

fn main() {
    println!(r"                                                  
  ____                                ____                  _   
 / ___|_ __ __ _ _ __  _ __  _   _   / ___|_ __ _   _ _ __ | |_ 
| |   | '__/ _` | '_ \| '_ \| | | | | |   | '__| | | | '_ \| __|
| |___| | | (_| | |_) | |_) | |_| | | |___| |  | |_| | |_) | |_ 
 \____|_|  \__,_| .__/| .__/ \__, |  \____|_|   \__, | .__/ \__|
                |_|   |_|    |___/              |___/|_|

Crappy Crypt v1.0.0
By @desyatkoff
    ");

    loop {
        println!(":: Enter file to encrypt or decrypt");

        let file_name_input = input("==> ");
        let file_input = match std::fs::File::open(&file_name_input) {
            Ok(file) => file,
            Err(err) => {
                println!("\n:: Could not open your file. Raised error: {err}\n");

                continue;
            }
        };

        println!(":: Enter the encryption key");

        let key_input = match input("==> ").parse::<u8>() {
            Ok(key) => key,
            Err(err) => {
                println!("\n:: Could not getting your key. Raised error: {err}\n");

                continue;
            },
        };

        let mut buf_reader = BufReader::new(file_input);
        let mut data_input = Vec::new();

        if let Err(err) = buf_reader.read_to_end(&mut data_input) {
            println!("\n:: Could not read your file. Raised error: {err}\n");

            continue;
        }

        let processed_data = process_data(&data_input, key_input);

        println!(":: Enter file name to output");

        let file_name_output = input("==> ");
        let file_output = match std::fs::File::create(&file_name_output) {
            Ok(file) => file,
            Err(err) => {
                println!("\n:: Could not create output file. Raised error: {err}\n");

                continue;
            }
        };

        let mut buf_writer = BufWriter::new(file_output);

        if let Err(err) = buf_writer.write_all(&processed_data) {
            println!("\n:: Could not write data to your output file. Raised error: {err}\n");

            continue;
        }

        println!("\n\n");
    }
}

