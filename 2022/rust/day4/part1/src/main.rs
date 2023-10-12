use std::env;
use std::fs;
fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path: &str = parse_configs(&args);
    let mut total_overlap = 0;
    for line in fs::read_to_string(file_path).unwrap().lines() {
        let elf_1_assignment = get_assignment(line,1);
        let elf_2_assignment = get_assignment(line,2);
        // Check if elf_1 assignment is contained in elf_2
        println!("\n");
        println!("{:?}",line);
        println!("{}",total_overlap);
        if get_lower_assignment(elf_1_assignment) >= get_lower_assignment(elf_2_assignment) && 
           get_higher_assignment(elf_1_assignment) <= get_higher_assignment(elf_2_assignment) {
            total_overlap = total_overlap+1;
        }
        println!("{:?}",elf_1_assignment);
        println!("{}",get_lower_assignment(elf_1_assignment)); 
        println!("{}",get_higher_assignment(elf_1_assignment)); 
        println!("{:?}",elf_2_assignment);
        println!("{}",get_lower_assignment(elf_2_assignment)); 
        println!("{}",get_higher_assignment(elf_2_assignment)); 
        println!("{}",total_overlap);
        // Check if elf_2 assignment is contained in elf_1
        if get_lower_assignment(elf_2_assignment) >= get_lower_assignment(elf_1_assignment) && 
           get_higher_assignment(elf_2_assignment) <= get_higher_assignment(elf_1_assignment) {
            total_overlap = total_overlap+1;
        }
        if get_lower_assignment(elf_2_assignment) == get_lower_assignment(elf_1_assignment) && 
           get_higher_assignment(elf_2_assignment) == get_higher_assignment(elf_1_assignment) {
            total_overlap = total_overlap-1;
        }
        println!("{}",total_overlap);
    }
    println!("Hello, world!");
}

fn parse_configs(args: &[String]) -> &str {
    let file_path = &args[1];
    file_path
}

fn get_assignment(text: &str, i: usize) -> Option<&str> {
    text.split(',')
        .skip(i-1)
        .next()
}

fn get_lower_assignment(text: Option<&str>) -> i32 {
    let v: Vec<&str> = text.unwrap().split(|c| c=='-').collect();
    return v[0].parse().unwrap();
}

fn get_higher_assignment(text: Option<&str>) -> i32 {
    let v: Vec<&str> = text.unwrap().split(|c| c=='-').collect();
    return v[1].parse().unwrap();
}
