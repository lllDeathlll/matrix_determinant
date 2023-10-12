use matrix_determinant::*;
use std::io::Write;
use std::{io, io::Error, io::ErrorKind};

fn main() {
    println!("Matrix Determinant Calculator");

    print!("Enter matrix's size (2 or 3): ");
    io::stdout().flush().expect("flush failed."); // Allows to print one line (and gives error if failed)

    /*
    Gets integer for choice and gives error if input is wrong (can't parse)
     */
    let mut choice = String::new();
    io::stdin()
        .read_line(&mut choice)
        .expect("Failed to read line");
    let choice: i32 = match choice.trim().parse() {
        Ok(num) => num,
        Err(err) => match err.to_string().as_str() {
            "invalid float literal" => panic!("Value must be a number!"),
            "cannot parse float from empty string" => panic!("You must enter a number!"),
            _ => panic!("Error!"),
        },
    };

    /*
    Matches choice and if it is other than 2 and 3, gives error
    */
    match choice {
        2 => {
            let mut matrix = [[0.0; 2]; 2];

            for i in 0..2 {
                for j in 0..2 {
                    print!("Enter number of matrix({};{}):", i + 1, j + 1);
                    io::stdout().flush().expect("flush failed.");
                    matrix[i][j] = get_f64().expect("");
                }
            }

            let matrix = get_matrix2(matrix);
            println!(
                "Matrix's determinant is equals to: {}",
                matrix.determinant()
            );
        }
        3 => {
            let mut matrix = [[0.0; 3]; 3];

            for i in 0..3 {
                for j in 0..3 {
                    print!("Enter number of matrix({};{}):", i + 1, j + 1);
                    io::stdout().flush().expect("flush failed.");
                    matrix[i][j] = get_f64().expect("");
                }
            }

            let matrix = get_matrix3(matrix);
            println!(
                "Matrix's determinant is equals to: {}",
                matrix.determinant()
            );
        }
        _ => panic!("You must enter either 2 or 3!"),
    }
}

/*
 */
fn get_f64() -> Result<f64, Error> {
    let mut x = String::new();

    io::stdin().read_line(&mut x).expect("Failed to read line");

    let x: Result<f64, Error> = match x.trim().parse() {
        Ok(num) => Ok(num),
        Err(err) => match err.to_string().as_str() {
            "invalid float literal" => Err(std::io::Error::new(
                ErrorKind::InvalidInput,
                "Value must be a number",
            )),
            "cannot parse float from empty string" => Ok(0.0),
            _ => Err(std::io::Error::new(ErrorKind::Other, "Error!")),
        },
    };

    x
}
