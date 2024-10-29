use std::fs;
use std::io::Error;

/*
    ### Result enum ###
        - Result enum is used when we need to know if something worked or failed
            - Ok() variant is used when something went well
            - Err() variant is used when something bad happened

    ### Option enum ###
        - Option enum is used when we need to know if a value is present or not
            - Some() variant is used when we have a value
            - None variant is used when there is no value
*/

fn result_demo_1() {
    match divide(5.0, 0.0) {
        Ok(result_of_division) => {
            println!("{}", result_of_division)
        }
        Err(what_went_wrong) => {
            println!("{}", what_went_wrong)
        }
    }

    match validate_email(String::from("hello@world.com")) {
        Ok(..) => println!("email is valid"),
        Err(reason_for_email_validation_failure) => {
            println!("{}", reason_for_email_validation_failure)
        }
    }

    let ingredients = vec![
        String::from("Cheese"),
        String::from("Tomato"),
        String::from("Salt"),
        String::from("Onions"),
    ];

    match validate_ingredients(&ingredients) {
        Ok(..) => println!("adequate ingredients"),
        Err(reason_for_inadequate_ingredients) => {
            println!("{}", reason_for_inadequate_ingredients)
        }
    }
}

fn main() {
    match fs::read_to_string("logs.txt") {
        Ok(text_that_was_read) => {
            println!("{}", text_that_was_read.len())
        }
        Err(reason_text_was_not_read) => {
            println!("Failed to read file: {}", reason_text_was_not_read)
        }
    }

    println!();

    result_demo_1();
}

// Result is a generic 'enum'
fn divide(a: f64, b: f64) -> Result<f64, Error> {
    if b == 0.0 {
        /*
            ### Error ###
                - Error::other(...) is the instance of the 'Error' struct
                - Many modules in the std lib have their own custom error types
                    - use std::str::Utf8Error

                    - use std::string::FromUtf8Error

                    - use std::num::ParseIntError
                    - use std::num::ParseFloatError
                    - use std::num::TryFromIntError
                - You can also create your own custom types of errors
                - There isn't really a general-purpose catch-all type of error
                    - JavaScript has 'Error'
                    - Python has 'Exception'
        */
        Err(Error::other("can't divide by 0"))
    } else {
        Ok(a / b)
    }
}

fn validate_email(email: String) -> Result<(), Error> {
    if email.contains("@") {
        // Success
        Ok(()) // We are returning an 'empty tuple' here
    } else {
        Err(Error::other("emails must have an @"))
    }
}

fn validate_ingredients(ingredients: &Vec<String>) -> Result<(), Error> {
    if ingredients.len() > 3 {
        Err(Error::other("too many ingredients"))
    } else {
        Ok(())
    }
}
