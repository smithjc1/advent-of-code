use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path: &str = parse_configs(&args);
    let mut sequence: [char; 4] = ['a', 'a', 'a', 'a'];
    let mut value: usize = 0;
    for line in fs::read_to_string(file_path).unwrap().lines() {
        for (i, c) in line.chars().enumerate() {
            sequence[i%4]=c;
            let k = sequence.clone();
            let mut flag =0;
            for (h, x) in k.iter().enumerate(){
                for (i, y) in k.iter().enumerate(){
                    if x==y && i!=h {flag=1};
                }
            }
            if flag==0 && i>=4 && value==0{
                value=i+1;
                println!{"{}",value}
                println!{"{:?}",sequence}
            }
        }
    }
}

fn parse_configs(args: &[String]) -> &str {
    let file_path = &args[1];
    file_path
}
