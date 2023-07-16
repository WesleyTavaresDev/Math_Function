mod affine_function;

use affine_function::affine;
use std::io;
fn main() {
    println!("Affine Function: \n f(x) = ax + b \n");

    println!("Enter a: ");
    let mut angular_coefficient = String::new();
    io::stdin()
        .read_line(&mut angular_coefficient)
        .expect("failed to read the input");

    println!("Enter b:");
    let mut linar_coefficient = String::new();
    io::stdin()
        .read_line(&mut linar_coefficient)
        .expect("failed to read the input");

    let affine = affine::AffineCoefficient {
        angular: match angular_coefficient.trim().parse() {
            Ok(num) => num,
            Err(_) => panic!("Please enter a number"),
        },
        linear: match linar_coefficient.trim().parse() {
            Ok(num) => num,
            Err(_) => panic!("Please enter a number"),
        },
    };

    loop {
        println!("1.Is crescent?\n2.Is decrescent?\n3.Funtion Summary");
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read");

        match input.trim().parse::<i8>() {
            Ok(num) => {
                println!("{}", num);
                if num == 1 {
                    println!("{}\n", affine::AffineCoefficient::is_crescent(&affine));
                } else if num == 2 {
                    println!("{}\n", affine::AffineCoefficient::is_decrescent(&affine));
                } else if num == 3 {
                    affine::AffineCoefficient::function_summary(&affine);
                } else {
                    continue;
                }
            }
            Err(_error) => break,
        }
    }
}
