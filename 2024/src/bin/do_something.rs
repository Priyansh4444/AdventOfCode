use std::fs;
use std::io::Write;
use std::path::Path;

fn main() -> std::io::Result<()> {
    let base_dir = Path::new("C:/Users/Priya/Downloads/AdventOfCode/2024/src");
    let data_dir = base_dir.join("data");
    let test_data_dir = data_dir.join("test");
    let template = r#"
fn main() {
    let input: &str = include_str!("template.txt");
    println!("Answer to part1: {}", part1(input));
    println!("Answer to part2: {}", part2(input));
}

fn part1(input: &str) -> usize {
    0
}

fn part2(input: &str) -> usize {
    0
}

#[test]
fn test_part1() {
    assert_eq!(part1(include_str!("../data/test/.txt")), 0);
}

#[test]
fn test_part2() {
    assert_eq!(part2(include_str!("../data/test/.txt")), 0);
}
"#;

    // Create files 06-25.txt in the data and data/test directories
    for day in 7..=25 {
        if day >= 7 {
            let rs_file_path = base_dir.join(format!("bin/{:02}.rs", day));
            let content = template
                .replace("template.txt", &format!("../data/{:02}.txt", day))
                .replace("../data/test/.txt", &format!("../data/test/{:02}.txt", day));
            let mut rs_file = fs::File::create(rs_file_path)?;
            rs_file.write_all(content.trim_start().as_bytes())?;
        }
    }

    Ok(())
}
