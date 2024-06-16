use regex::Regex;
use std::fs;
use std::process::Command;

fn calculate_tested_functions() -> usize {
    let output = Command::new("cargo")
        .arg("test")
        .output()
        .expect("Failed to execute command.");

    let stdout = String::from_utf8_lossy(&output.stdout);
    dbg!(&stdout);
    let count = stdout
        .lines()
        .filter(|line| {
            line.contains("test") && line.contains("... ok") && !line.contains("optimize")
        })
        .count();
    count
}

pub fn calculate_stars() {
    let count = calculate_tested_functions();
    println!("Number of tested functions: {}", count);

    let readme_path = "README.md";
    let readme_content = fs::read_to_string(readme_path).expect("Failed to read README.md");

    let re = Regex::new(r"Total stars: \d+ ⭐️").unwrap();
    if let Some(_caps) = re.captures(&readme_content) {
        let updated_content = re.replace(&readme_content, |_caps: &regex::Captures| {
            format!("Total stars: {} ⭐️", count)
        });
        fs::write(readme_path, updated_content.to_string()).expect("Failed to write to README.md");
    } else {
        println!("No match found in the README.md file.");
    }
}
