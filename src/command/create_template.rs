use crossterm::{
    style::{Color, ResetColor, SetForegroundColor},
    ExecutableCommand,
};
use std::fs::{self, File};
use std::io::{self, Read, Write};

pub fn generate_template() {
    let mut year = String::new();
    let mut day = String::new();
    let mut stdout: io::Stdout = io::stdout();

    stdout.execute(SetForegroundColor(Color::Green)).unwrap();
    println!("Enter AOC year: ");
    io::stdin().read_line(&mut year).expect("cant get the year");

    stdout.execute(SetForegroundColor(Color::Blue)).unwrap();
    println!("Enter AOC day: ");
    io::stdin().read_line(&mut day).expect("cant get the day");
    stdout.execute(ResetColor).unwrap();

    let problem_first = "p_1";
    let problem_second = "p_2";

    let test_first = "tests_1";
    let test_second = "tests_2";

    let mut file = match File::open("./src/template/template.rs") {
        Ok(file) => file,
        Err(e) => panic!("Unable to open template file: {}", e),
    };
    let mut content = String::new();

    match file.read_to_string(&mut content) {
        Ok(_) => {}
        Err(e) => panic!("Error reading template file: {}", e),
    }

    let year = year.trim();
    let day = day.trim();

    let p1 = content.replace(problem_first, &format!("problem{}_1", day).to_string());
    let p2 = p1.replace(problem_second, &format!("problem{}_2", day).to_string());

    let t1 = p2.replace(
        test_first,
        &format!("tests_y{}_d{}_1", year, day).to_string(),
    );
    let modified_content = t1.replace(
        test_second,
        &format!("tests_y{}_d{}_2", year, day).to_string(),
    );

    loop {
        let mut new_file = match File::create(
            format!("./src/year{}/problems/problem{}.rs", year, day).to_string(),
        ) {
            Ok(file) => file,
            Err(_) => {
                if let Err(_) = fs::metadata(format!("./src/year{}/", year).to_string()) {
                    match fs::create_dir(format!("./src/year{}/", year).to_string()) {
                        Ok(_) => {}
                        Err(e) => panic!("Failed to create folder: {}", e),
                    };

                    match fs::create_dir(format!("./src/year{}/problems/", year).to_string()) {
                        Ok(_) => continue,
                        Err(e) => panic!("Failed to create folder: {}", e),
                    }
                } else {
                    panic!("Folder already exists.");
                }
            }
        };

        match new_file.write_all(modified_content.as_bytes()) {
            Ok(()) => {
                println!("Modified template successfully written to new file.");
                break;
            }
            Err(e) => panic!("Error writing to new file: {}", e),
        }
    }
}
