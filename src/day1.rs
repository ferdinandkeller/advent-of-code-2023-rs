use std::error::Error;

/// For each line in the input file,
/// 1. Filter out all non-digit characters
/// 2. Collect the remaining characters into a vector
/// 3. Create a string from the first and last characters of the vector
/// 4. Parse the string into a u64
/// 5. Sum all the u64s
/// 6. Print the sum
pub fn solve_a(input: &str) -> Result<(), Box<dyn Error>> {
    println!(
        "solution 1-A : {}",
        input
            .lines()
            .map(|l| {
                l.chars()
                    .filter(|&c| '0' <= c && c <= '9')
                    .collect::<Vec<char>>()
            })
            .map(|l| format!("{}{}", l[0], l.last().unwrap()))
            .filter_map(|l| l.parse::<u64>().ok())
            .fold(0, |acc, x| acc + x)
    );

    Ok(())
}

/// Same as before, but with a bunch of replace calls to convert the words to numbers before parsing them.
/// We can't just replace all the letters with their corresponding numbers, because there is some overlap
/// (e.g. "zero" and "one" both contain "o", so "zerone" should give "01", not "0" or "1").
pub fn solve_b(input: &str) -> Result<(), Box<dyn Error>> {
    println!(
        "solution 1-B : {}",
        input
            .lines()
            .map(|l| {
                l.replace("zero", "0o")
                    .replace("one", "o1e")
                    .replace("two", "t2o")
                    .replace("three", "t3e")
                    .replace("four", "4")
                    .replace("five", "5e")
                    .replace("six", "6")
                    .replace("seven", "7n")
                    .replace("eight", "e8")
                    .replace("nine", "9")
            })
            .map(|l| {
                l.chars()
                    .filter(|&c| '0' <= c && c <= '9')
                    .collect::<Vec<char>>()
            })
            .map(|l| format!("{}{}", l[0], l.last().unwrap()))
            .filter_map(|l| l.parse::<u64>().ok())
            .fold(0, |acc, x| acc + x)
    );

    Ok(())
}
