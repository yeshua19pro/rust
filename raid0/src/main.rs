use std::fs::{self, File};
use std::io::{self, BufReader, Read, Write};

fn main() {
    //Print the options to the user in a loop
    loop {
        println!("Choose an action:");
        println!("1. Raid0 files creation");
        println!("2. Recreate Raid0 File");
        println!("3. Delete Raid0 files");
        println!("4. Raid1 files creation");
        println!("5. Delete Raid1 files");
        println!("6. Raid10 files creation");
        println!("7. Recreate Raid10 Files");
        println!("8. Delete Raid10 files");
        println!("9. Exit");

        //Mut the entered string in the console to an unsigned integer
        let mut choice = String::new();
        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read line");
        let choice: u32 = choice.trim().parse().expect("Please enter a number");

        //Match the number with the possible options and call the methods
        match choice {
            1 => raid0(),
            2 => recreate_raid0(),
            3 => delete_raid0(),
            4 => create_raid1(),
            5 => delete_raid1(),
            6 => create_raid10(),
            7 => recreate_raid10(),
            8 => delete_raid10(),
            9 => {
                println!("Exiting...");
                break;
            }
            _ => println!("Invalid choice, please try again."),
        }
    }
}

//Methods
fn raid0() {
    println!("Enter the text to process:");
    let mut input_text = String::new();
    io::stdin()
        .read_line(&mut input_text)
        .expect("Failed to read line");
    let input_text = input_text.trim();

    // Separate odd and even characters
    let odd_chars: String = input_text
        .chars()
        .enumerate()
        .filter(|(i, _)| i % 2 == 0) // Odd positions (index 0, 2, ...)
        .map(|(_, c)| c)
        .collect();

    let even_chars: String = input_text
        .chars()
        .enumerate()
        .filter(|(i, _)| i % 2 != 0) // Even positions (index 1, 3, ...)
        .map(|(_, c)| c)
        .collect();

    // Write to file1
    match File::create("file1.txt") {
        Ok(mut file) => {
            file.write_all(odd_chars.as_bytes())
                .expect("Failed to write to file1");
            println!("File1 created successfully with odd-positioned letters.");
        }
        Err(e) => println!("Failed to create file1: {}", e),
    }

    // Write to file2
    match File::create("file2.txt") {
        Ok(mut file) => {
            file.write_all(even_chars.as_bytes())
                .expect("Failed to write to file2");
            println!("File2 created successfully with even-positioned letters.");
        }
        Err(e) => println!("Failed to create file2: {}", e),
    }
}

fn delete_raid0() {
    let files_to_delete = ["file1.txt", "file2.txt", "fileRaid0.txt"];

    for file in files_to_delete {
        match fs::remove_file(file) {
            Ok(_) => println!("Successfully deleted {}", file),
            Err(e) => println!("Failed to delete {}: {}", file, e),
        }
    }
}

fn recreate_raid0() {
    // Open file1 and file2 for reading
    let file1 = File::open("file1.txt");
    let file2 = File::open("file2.txt");

    match (file1, file2) {
        (Ok(file1), Ok(file2)) => {
            let mut content1 = String::new();
            let mut content2 = String::new();

            // Read the contents of file1 and file2
            BufReader::new(file1)
                .read_to_string(&mut content1)
                .expect("Failed to read file1");
            BufReader::new(file2)
                .read_to_string(&mut content2)
                .expect("Failed to read file2");

            // Open the target fileRaid0 for writing
            match File::create("fileRaid0.txt") {
                Ok(mut file_raid0) => {
                    let mut iter1 = content1.chars();
                    let mut iter2 = content2.chars();

                    // Interleave characters from both files
                    loop {
                        match (iter1.next(), iter2.next()) {
                            (Some(c1), Some(c2)) => {
                                file_raid0
                                    .write_all(c1.to_string().as_bytes())
                                    .expect("Failed to write to fileRaid0");
                                file_raid0
                                    .write_all(c2.to_string().as_bytes())
                                    .expect("Failed to write to fileRaid0");
                            }
                            (Some(c1), None) => {
                                file_raid0
                                    .write_all(c1.to_string().as_bytes())
                                    .expect("Failed to write to fileRaid0");
                            }
                            (None, Some(c2)) => {
                                file_raid0
                                    .write_all(c2.to_string().as_bytes())
                                    .expect("Failed to write to fileRaid0");
                            }
                            (None, None) => break,
                        }
                    }
                    println!("fileRaid0.txt successfully created by merging file1 and file2.");
                }
                Err(e) => println!("Failed to create fileRaid0: {}", e),
            }
        }
        (Err(e1), Err(e2)) => {
            println!("Failed to open file1 and file2: {}, {}", e1, e2);
        }
        (Err(e), _) => println!("Failed to open file1: {}", e),
        (_, Err(e)) => println!("Failed to open file2: {}", e),
    }
}

fn create_raid1() {
    println!("Enter the text to process:");
    let mut input_text = String::new();
    io::stdin()
        .read_line(&mut input_text)
        .expect("Failed to read line");
    let input_text = input_text.trim();

    // Create file11 and write input_text into it
    match File::create("file11.txt") {
        Ok(mut file) => {
            file.write_all(input_text.as_bytes()) // Writes input_text to the file
                .expect("Failed to write to file11");
            println!("file11.txt has been successfully created with the provided text.");
        }
        Err(e) => println!("Failed to create file11: {}", e),
    }
    match File::create("file12.txt") {
        Ok(mut file) => {
            file.write_all(input_text.as_bytes()) // Writes input_text to the file
                .expect("Failed to write to file11");
            println!("file11.txt has been successfully created with the provided text.");
        }
        Err(e) => println!("Failed to create file11: {}", e),
    }
}

fn delete_raid1() {
    let files_to_delete = ["file11.txt", "file12.txt"];

    for file in files_to_delete {
        match fs::remove_file(file) {
            Ok(_) => println!("Successfully deleted {}", file),
            Err(e) => println!("Failed to delete {}: {}", file, e),
        }
    }
}

fn create_raid10() {
    println!("Enter the text to process:");
    let mut input_text = String::new();
    io::stdin()
        .read_line(&mut input_text)
        .expect("Failed to read line");
    let input_text = input_text.trim();

    // Separate odd and even characters
    let odd_chars: String = input_text
        .chars()
        .enumerate()
        .filter(|(i, _)| i % 2 == 0) // Odd positions (index 0, 2, ...)
        .map(|(_, c)| c)
        .collect();

    let even_chars: String = input_text
        .chars()
        .enumerate()
        .filter(|(i, _)| i % 2 != 0) // Even positions (index 1, 3, ...)
        .map(|(_, c)| c)
        .collect();

    // Write to file01
    match File::create("file01.txt") {
        Ok(mut file) => {
            file.write_all(odd_chars.as_bytes())
                .expect("Failed to write to file01");
            println!("file01 created successfully with odd-positioned letters.");
        }
        Err(e) => println!("Failed to create file01: {}", e),
    }

    // Write to file02
    match File::create("file02.txt") {
        Ok(mut file) => {
            file.write_all(even_chars.as_bytes())
                .expect("Failed to write to file02");
            println!("file02 created successfully with even-positioned letters.");
        }
        Err(e) => println!("Failed to create file02: {}", e),
    }

    // Write to file21
    match File::create("file21.txt") {
        Ok(mut file) => {
            file.write_all(odd_chars.as_bytes())
                .expect("Failed to write to file21");
            println!("file21 created successfully with odd-positioned letters.");
        }
        Err(e) => println!("Failed to create file21: {}", e),
    }

    // Write to file22
    match File::create("file22.txt") {
        Ok(mut file) => {
            file.write_all(even_chars.as_bytes())
                .expect("Failed to write to file22");
            println!("file22 created successfully with even-positioned letters.");
        }
        Err(e) => println!("Failed to create file22: {}", e),
    }
}

fn delete_raid10() {
    let files_to_delete = ["file01.txt", "file02.txt", "file21.txt", "file22.txt", "fileRaid01.txt", "fileRaid21.txt"];

    for file in files_to_delete {
        match fs::remove_file(file) {
            Ok(_) => println!("Successfully deleted {}", file),
            Err(e) => println!("Failed to delete {}: {}", file, e),
        }
    }
}

fn recreate_raid10() {
    // Open file01, file02, file21, file22 for reading
    let file01 = File::open("file01.txt");
    let file02 = File::open("file02.txt");
    let file21 = File::open("file21.txt");
    let file22 = File::open("file22.txt");

    match (file01, file02) {
        (Ok(file01), Ok(file02)) => {
            let mut content1 = String::new();
            let mut content2 = String::new();

            // Read the contents of file01 and file02
            BufReader::new(file01)
                .read_to_string(&mut content1)
                .expect("Failed to read file01");
            BufReader::new(file02)
                .read_to_string(&mut content2)
                .expect("Failed to read file02");

            // Open the target fileRaid0 for writing
            match File::create("fileRaid01.txt") {
                Ok(mut file_raid01) => {
                    let mut iter1 = content1.chars();
                    let mut iter2 = content2.chars();

                    // Interleave characters from both files
                    loop {
                        match (iter1.next(), iter2.next()) {
                            (Some(c1), Some(c2)) => {
                                file_raid01
                                    .write_all(c1.to_string().as_bytes())
                                    .expect("Failed to write to fileRaid01");
                                file_raid01
                                    .write_all(c2.to_string().as_bytes())
                                    .expect("Failed to write to fileRaid01");
                            }
                            (Some(c1), None) => {
                                file_raid01
                                    .write_all(c1.to_string().as_bytes())
                                    .expect("Failed to write to fileRaid01");
                            }
                            (None, Some(c2)) => {
                                file_raid01
                                    .write_all(c2.to_string().as_bytes())
                                    .expect("Failed to write to fileRaid01");
                            }
                            (None, None) => break,
                        }
                    }
                    println!("fileRaid01.txt successfully created by merging file01 and file02.");
                }
                Err(e) => println!("Failed to create fileRaid01: {}", e),
            }
        }
        (Err(e1), Err(e2)) => {
            println!("Failed to open file1 and file2: {}, {}", e1, e2);
        }
        (Err(e), _) => println!("Failed to open file01: {}", e),
        (_, Err(e)) => println!("Failed to open file02: {}", e),
    }

    match (file21, file22) {
        (Ok(file21), Ok(file22)) => {
            let mut content1 = String::new();
            let mut content2 = String::new();

            // Read the contents of file01 and file02
            BufReader::new(file21)
                .read_to_string(&mut content1)
                .expect("Failed to read file21");
            BufReader::new(file22)
                .read_to_string(&mut content2)
                .expect("Failed to read file22");

            // Open the target fileRaid0 for writing
            match File::create("fileRaid21.txt") {
                Ok(mut file_raid21) => {
                    let mut iter1 = content1.chars();
                    let mut iter2 = content2.chars();

                    // Interleave characters from both files
                    loop {
                        match (iter1.next(), iter2.next()) {
                            (Some(c1), Some(c2)) => {
                                file_raid21
                                    .write_all(c1.to_string().as_bytes())
                                    .expect("Failed to write to fileRaid21");
                                file_raid21
                                    .write_all(c2.to_string().as_bytes())
                                    .expect("Failed to write to fileRaid21");
                            }
                            (Some(c1), None) => {
                                file_raid21
                                    .write_all(c1.to_string().as_bytes())
                                    .expect("Failed to write to fileRaid21");
                            }
                            (None, Some(c2)) => {
                                file_raid21
                                    .write_all(c2.to_string().as_bytes())
                                    .expect("Failed to write to fileRaid21");
                            }
                            (None, None) => break,
                        }
                    }
                    println!("fileRaid21.txt successfully created by merging file21 and file22.");
                }
                Err(e) => println!("Failed to create fileRaid21: {}", e),
            }
        }
        (Err(e1), Err(e2)) => {
            println!("Failed to open file21 and file22: {}, {}", e1, e2);
        }
        (Err(e), _) => println!("Failed to open file21: {}", e),
        (_, Err(e)) => println!("Failed to open file22: {}", e),
    }

   
    
}
