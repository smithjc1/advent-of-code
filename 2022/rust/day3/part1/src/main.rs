use std::env;
use std::fs;
fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path: &str = parse_configs(&args);
    let mut total_value: i32 = 0;
    for line in fs::read_to_string(file_path).unwrap().lines() {
        let mut test = Rucksack {
            contents: line.to_string(),
            compartment1: String::from(""),
            compartment2: String::from(""),
        };
        test.check_compartments();
        let common_items = test.find_common_items();
        for each_item in common_items.chars() {
            let mut item = Item {
                r#type: each_item,
                value: 0,
            };
            item.assign_value();
            total_value = total_value + item.value;
        }
    }
    println!("{}", total_value);
}

fn parse_configs(args: &[String]) -> &str {
    let file_path = &args[1];
    file_path
}

#[derive(Debug)]
struct Rucksack {
    contents: String,
    compartment1: String,
    compartment2: String,
}

impl Rucksack {
    fn check_compartments(&mut self) {
        self.compartment1 = self.contents[..self.contents.chars().count() / 2].to_string();
        self.compartment2 = self.contents[self.contents.chars().count() / 2..].to_string();
    }
    fn find_common_items(&self) -> String {
        let mut common_items: String = String::from("");
        for item_c1 in self.compartment1.chars() {
            for item_c2 in self.compartment2.chars() {
                if item_c1 == item_c2 && !(common_items.contains(item_c1)) {
                    common_items.push(item_c2);
                }
            }
        }
        return common_items;
    }
}

struct Item {
    r#type: char,
    value: i32,
}

impl Item {
    fn assign_value(&mut self) {
        if self.r#type == 'a' {
            self.value = 1
        }
        if self.r#type == 'b' {
            self.value = 2
        }
        if self.r#type == 'c' {
            self.value = 3
        }
        if self.r#type == 'd' {
            self.value = 4
        }
        if self.r#type == 'e' {
            self.value = 5
        }
        if self.r#type == 'f' {
            self.value = 6
        }
        if self.r#type == 'g' {
            self.value = 7
        }
        if self.r#type == 'h' {
            self.value = 8
        }
        if self.r#type == 'i' {
            self.value = 9
        }
        if self.r#type == 'j' {
            self.value = 10
        }
        if self.r#type == 'k' {
            self.value = 11
        }
        if self.r#type == 'l' {
            self.value = 12
        }
        if self.r#type == 'm' {
            self.value = 13
        }
        if self.r#type == 'n' {
            self.value = 14
        }
        if self.r#type == 'o' {
            self.value = 15
        }
        if self.r#type == 'p' {
            self.value = 16
        }
        if self.r#type == 'q' {
            self.value = 17
        }
        if self.r#type == 'r' {
            self.value = 18
        }
        if self.r#type == 's' {
            self.value = 19
        }
        if self.r#type == 't' {
            self.value = 20
        }
        if self.r#type == 'u' {
            self.value = 21
        }
        if self.r#type == 'v' {
            self.value = 22
        }
        if self.r#type == 'w' {
            self.value = 23
        }
        if self.r#type == 'x' {
            self.value = 24
        }
        if self.r#type == 'y' {
            self.value = 25
        }
        if self.r#type == 'z' {
            self.value = 26
        }
        if self.r#type == 'A' {
            self.value = 27
        }
        if self.r#type == 'B' {
            self.value = 28
        }
        if self.r#type == 'C' {
            self.value = 29
        }
        if self.r#type == 'D' {
            self.value = 30
        }
        if self.r#type == 'E' {
            self.value = 31
        }
        if self.r#type == 'F' {
            self.value = 32
        }
        if self.r#type == 'G' {
            self.value = 33
        }
        if self.r#type == 'H' {
            self.value = 34
        }
        if self.r#type == 'I' {
            self.value = 35
        }
        if self.r#type == 'J' {
            self.value = 36
        }
        if self.r#type == 'K' {
            self.value = 37
        }
        if self.r#type == 'L' {
            self.value = 38
        }
        if self.r#type == 'M' {
            self.value = 39
        }
        if self.r#type == 'N' {
            self.value = 40
        }
        if self.r#type == 'O' {
            self.value = 41
        }
        if self.r#type == 'P' {
            self.value = 42
        }
        if self.r#type == 'Q' {
            self.value = 43
        }
        if self.r#type == 'R' {
            self.value = 44
        }
        if self.r#type == 'S' {
            self.value = 45
        }
        if self.r#type == 'T' {
            self.value = 46
        }
        if self.r#type == 'U' {
            self.value = 47
        }
        if self.r#type == 'V' {
            self.value = 48
        }
        if self.r#type == 'W' {
            self.value = 49
        }
        if self.r#type == 'X' {
            self.value = 50
        }
        if self.r#type == 'Y' {
            self.value = 51
        }
        if self.r#type == 'Z' {
            self.value = 52
        }
    }
}
