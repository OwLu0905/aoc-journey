use aoc::command::{calculate_stars::calculate_stars, create_template::generate_template};

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() > 1 {
        let command = &args[1];
        match command.as_str() {
            "stars" => {
                calculate_stars();
            }
            "temp" => generate_template(),

            _ => {
                println!("Command not found.");
            }
        }
    }
}
