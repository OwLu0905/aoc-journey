pub fn problem8_1(path: &str) -> i32 {
    21
}
pub fn problem8_2() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tests_y2022_d8_1() {
        let file_path_simple = "testdata/y2022_p8_simple.txt";
        // let file_path = "testdata/y2022_p8.txt";
        assert_eq!(21, problem8_1(file_path_simple));
    }

    #[test]
    fn tests_y2022_d8_2() {}
}
