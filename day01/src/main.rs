use std::error::Error;
use std::io::Read;
use std::num::ParseIntError;

fn main() -> Result<(), Box<dyn Error>> {
    let mut buffer = String::new();

    std::io::stdin().read_to_string(&mut buffer)?;

    let mut calories = get_calories(&buffer)?;
    calories.sort_unstable();

    let first = calories.pop().ok_or("missing data")?;
    let second = calories.pop().ok_or("missing data")?;
    let third = calories.pop().ok_or("missing data")?;

    println!("Part 1: {:?}", first);
    println!("Part 2: {:?}", first + second + third);

    Ok(())
}

fn get_calories(buffer: &str) -> Result<Vec<u32>, ParseIntError> {
    let mut result = vec![];

    let mut values: Vec<u32> = vec![];
    for line in buffer.lines() {
        if line.is_empty() {
            result.push(values.iter().sum());
            values = vec![];
        } else {
            let value = line.parse()?;
            values.push(value);
        }
    }

    Ok(result)
}
