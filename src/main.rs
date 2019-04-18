#[macro_use] extern crate text_io;

fn main() {
    let mut total : f64 = 0_f64;
    let instructions = "    Enter a calculation to begin.
    The expression must be space separated, however you can
    also split your expression into multiple lines if desired.
    e.g. 1 + 2 * 4 / 3  will result in 4. The calculator follows
    left -> right order of operations only.\nType 'quit' to exit.\n\n";

    print!("{}", instructions);
    let mut operator : String = "".to_string();
    // set our operator as mut so we can change it later
    print!("Total: {}\n", total);

    // loop infinitely
    loop {
        let input : String = text_io::read!(); // get our input as a String
        if input.eq("quit") {
            // exit the loop if "quit"
            break;
        }
        if operator.eq("") {
            // if we don't have an operator already saved
            if is_op(&input) {
                operator = input; // save our new operator
            } else {
                // check if input is a number, if so set total as it
                total = match input.parse::<f64>() {
                    Ok(number) => number,
                    Err(_) => total,
                };
            }
        } else {
            // we already have an operator
            match operator.as_ref() {
                /* match the operator and parse the input as a 64 bit int
                basically, we do some error checking and see if it can
                even be parsed to an int. If it can, we handle the calculation.
                If it can't, we still handle the calculation, but we do it
                will a number that does not change the result (i.e. 5 * 1 == 5).
                */
                
                "*" => total *= match input.parse::<f64>() {
                    Ok(number) => number,
                    Err(_) => {
                        print!("Wrong!\n");
                        1_f64
                    }
                },
                "+" => total += match input.parse::<f64>() {
                    Ok(number) => number,
                    Err(_) => {
                        print!("Wrong!\n");
                        0_f64
                    }
                },
                "-" => total -= match input.parse::<f64>() {
                    Ok(number) => number,
                    Err(_) => {
                        print!("Wrong!\n");
                        0_f64
                    }
                },
                "/" => total /= match input.parse::<f64>() {
                    Ok(number) => {
                        if number != 0_f64 {
                            number
                        } else {
                            print!("Good try!\n");
                            1_f64
                        }

                    },
                    Err(_) => {
                        print!("Wrong!\n");
                        1_f64
                    }
                },
                "^" => {
                        let exp = match input.parse::<f64>() {
                            Ok(number) => number,
                            Err(_) => {
                                print!("Wrong!\n");
                                1_f64
                            }
                        };
                        total = total.powf(exp);
                },
                _ => print!("How did you get here?\n"),
            }
            operator = "".to_string();
            print!("Total: {}\n", total);
        }
    }
}
/**
 * Returns whether or not the given string reference is an operator
 * symbol or not. The possible operators are [+ - * / ^].
 */
fn is_op(input : &str) -> bool {
    // is_op takes a reference to our string, otherwise it will "steal" ownership
    input.eq("*") || input.eq("/") || input.eq("-") || input.eq("+") || input.eq("^")
}