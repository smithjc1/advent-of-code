use csv::ReaderBuilder;
use std::env;
use std::error::Error;
use std::fs;

fn main() {
    /* We're gonna do this with a command line input so let's use args as a vector */
    let args: Vec<String> = env::args().collect();
    let file_path = parse_configs(&args);
    println!("{}", file_path);
    let mut file_vector = Vec::new();
    let mut old_sum: Vec<i32> = Vec::new();
    let mut new_sum: i32 = 0;
    println!("{}", file_path);
    for line in fs::read_to_string(file_path).unwrap().lines() {
        if line.to_string() != "" {
            file_vector.push(line.to_string());
        } else {
            println!("{:?}", file_vector.clone());
            for item in file_vector.clone() {
                let sum = item.parse::<i32>().unwrap();
                new_sum = new_sum + sum;
            }
            old_sum.push(new_sum);
            new_sum = 0;
            file_vector.clear();
        }
    }
    old_sum.sort();
    old_sum.reverse();
    println!("{:?}", old_sum);
    let x: i32 = old_sum[..3].iter().sum();
    println!("{:?}", x);
    /* Example of how to print vector */
    /*    println!("{:?}", all_elves);*/
    example(file_path);
}

fn parse_configs(args: &[String]) -> &str {
    let file_path = &args[1];
    file_path
}

/*#[derive(Debug, serde::Deserialize, Eq, PartialEq)]
struct Row {
    calories: i32,
}
*/

fn example(path: &str) -> Result<(), Box<dyn Error>> {
    let mut rdr = ReaderBuilder::new()
        .has_headers(false)
        .flexible(true)
        .from_path(path)?;
    for result in rdr.records() {
        let record = result?;
    }
    Ok(())
}
