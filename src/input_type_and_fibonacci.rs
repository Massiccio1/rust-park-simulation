use std::thread;
use std::time::Duration;

fn main() {
    // Read a string from the user
    loop{
        println!(".--------------------------------.");

        println!("enter a number: ");
        let mut num:i32=0;
        num=input_type(num);
        println!("ok continue with: {}", num);

        // let fib=fibonacci_slow(num);
        
        let fib2=fibonacci_slow(num, true);
        println!("fibonacci slow di {num}: {fib2}");
        let fib=fibonacci_fast(num, true);
        println!("fibonacci fast di {num}: {fib}");
    }

}

fn input_type<T:FromStr>(_type:T) -> T {
    
    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");

        // Trim whitespace and parse the string as T
        let trimmed_input = input.trim();

        // println!("parsing: {}" , trimmed_input);

        match trimmed_input.parse::<T>() {
            Ok(number) => {
                // exit = true;
                return number;
                // println!("You entered a valid integer: {}", number);
            }
            Err(_) => {
                println!("Error: invalid input type, try agin");
            }
        }
    }

}

fn fibonacci_slow(depth:i32, verbose: bool) -> i128{
    if depth == 0 {return 0;}
    if depth == 1 {return 1;}


    let sum = fibonacci_slow(depth-1, verbose) + fibonacci_slow(depth-2, verbose);
    if verbose{
        println!("{} {}", sum, verbose);
    }
    return sum;
}

fn fibonacci_fast(depth:i32, verbose: bool) -> i128{

    if depth == 0 {return 0;}
    if depth == 1 {return 1;}

    

    let mut v: Vec<i128> = vec![0,1];

    let start:i32=2;
    for i in start..depth+1{
        let sum:i128 = v[(i-1) as usize] + v[(i-2) as usize];
        v.push(sum);
        if verbose{
            println!("{}", sum);
        }
    }

    
    return *v.last().unwrap();
}