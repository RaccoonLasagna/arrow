use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() == 1 {
        println!("{}", "Input a number!")
    } else {
        let length: i64 = args[1].parse().unwrap();
        for outside_loops1 in 1..length+1 {
            for inside_loops1 in 1..outside_loops1+1{
                print!("{}", "*")
            }
            print!("{}", "\n")
        }
        for outside_loops2 in (1..length).rev() {
            for inside_loops2 in 1..outside_loops2+1{
                print!("{}", "*")
            }
            print!("{}", "\n")
        }
    }
}
