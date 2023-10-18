use std::fs;
fn main() {
    let mut my_array: [Vec<char>; 9] = Default::default();
    my_array[0] = vec!['S', 'Z', 'P', 'D', 'L', 'B', 'F', 'C'];
    my_array[1] = vec!['N', 'V', 'G', 'P', 'H', 'W', 'B'];
    my_array[2] = vec!['F', 'W', 'B', 'J', 'G'];
    my_array[3] = vec!['G', 'J', 'N', 'F', 'L', 'W', 'C', 'S'];
    my_array[4] = vec!['W', 'J', 'L', 'T', 'P', 'M', 'S', 'H'];
    my_array[5] = vec!['B', 'C', 'W', 'G', 'F', 'S'];
    my_array[6] = vec!['H', 'T', 'P', 'M', 'Q', 'B', 'W'];
    my_array[7] = vec!['F', 'S', 'W', 'T'];
    my_array[8] = vec!['N', 'C', 'R'];
    let amount_to_move = 2;
    let from_column = 5;
    let to_column = 9;
    println!("{:?}", my_array[from_column - 1]);
    println!("{:?}", my_array[to_column - 1]);
    let file = "data/input";
    for line in fs::read_to_string(file).unwrap().lines() {
        if line.len() > 1 {
            if &line[0..4] == "move" {
                println!("{:?}", line);
                let mut line_iter = line.split_whitespace();
                let _ = line_iter.next();
                let amount_to_move: i32 = line_iter.next().unwrap().parse::<i32>().unwrap();
                let _ = line_iter.next();
                let from_column = line_iter.next().unwrap().parse::<usize>().unwrap();
                let _ = line_iter.next();
                let to_column = line_iter.next().unwrap().parse::<usize>().unwrap();
                for i in 0..amount_to_move {
                    let x = my_array[from_column - 1].pop();
                    my_array[to_column - 1].push(x.unwrap());
                }
                for i in 0..9 {
                    println!("{:?}", my_array[i])
                }
            }
        }
    }
    for i in 0..9 {
        println!("{:?}", my_array[i].pop())
    }
}
