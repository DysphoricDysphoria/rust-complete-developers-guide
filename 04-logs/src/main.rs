use std::fs;
use std::io::Error;

/*
    ### 73. The Stack and Heap (AGAIN) ###
        - Stack
            - Fast, but limited size (2 - 8 MB)
        - Heap
            - Slow, but can grow to store a lot of data (Gigabytes
              worth of data)
        - Data/Data segment/Read-only-data segment/Static segment
            - Stores literal values that we write directly into our
              source code
                - Ex: let num = 45; let color = "red"; => 45 and
                  "red" are stored into 'Data'
        - Ex:
            - let numbers = vec![1, 2, 3, 4, 5];
                - The initial values 1, 2, 3, 4, 5 will be stored
                  into 'Data' and when we make the vector, these
                  will be copied into the 'Heap'
                  - ? => during parsing, rust might create 1 - 5
                    in 'Data' and then later copy it into the 'Heap'?
            - Super common pattern
                - Stack stores metadata about a data structure
                  (in this case 'numbers')
                    - pointer to value(s) | length | capacity
                - Heap stores the actual data
                    - Actual values of the vector are stored here
                - Avoids running out of memory in the stack if the
                  data structure grows to hold a lot of data
        - Corner case
            - If a data structure owns another data structure, the
              child's metadata will be placed on the heap
            - Ex: let vec_of_num = vec![ vec![1, 2, 3, 4, 5] ];
                - Metadata for nested vector will be stored inside
                  the 'Heap'
                - Metadata for parent vector will be stored inside
                  the 'Stack'
*/

/*
    ### 74. Strings, String Refs, and String Slices ###
        - String
            - Struct (pointer to text | length of string in
              heap | capacity of string in heap) in Stack
            - Pointer points to value in 'Heap'
            - Uses memory in 'Stack' & 'Heap'
        - &String
            - Reference (read-only) to 'String' (All the work
              of String + REFERENCE) | Stored in Stack
            - Uses memory in 'Stack'
        - &str (String slice - read-only)
            - Struct (pointer to text | length of string) in
              Stack
            - Pointer points to value in 'Heap' or 'Data'
              (Heap-allocated or Data-allocated text)
            - Uses memory in 'Stack'
            - String to String slice
                let color = String::from("red");
                let c = color.as_str();
                    - In this case 'c' points to "red" in 'Heap'!
*/

/*
    ### 75. When to Use Which String ###
        - &String and &str - both provide a read-only reference to text
          data - Why two different types?
            - &str lets you refer to text in the data segment without a
              'Heap' allocation => slightly more performant
                let color = "red";
                - If we do it solely via 'String', it will be a lot more
                  work
            - &str lets you 'slice' (take a portion) of text that is
              already on the heap
              let color = String::from("blue"); => String
              let portion = &color[1..4]; => &str
              - When we do &color[1..4], behind the scenes '&str' is
                created in Stack and &color[1..4] refers to 'lue'
                portion of the same heap-allocated 'blue'
              - Again, it will be a lot more work is we do this via
                'String'

        - Usage
            - String
                let color = String::from("red");
                - Use anytime we want ownership of text
                - Use anytime we want text that can grow or shrink
            - &String
                let color = String::from("red");
                let color_ref = &color;
                - Rarely used (usually never)
                - Rust will automatically turn &String into &str for you
            - &str
                let color = String::from("red");
                let c = color.as_str();
                - Use anytime you don't want to take ownership of text
                - Use anytime you want to refer to a 'portion' of a string
                owned by something else.

        - String slices can either point at text stored in the 'Data
          segment' or text stored in the 'Heap' that belongs to a String

*/

fn string_test(a: String, b: &String, c: &str) {
    println!("{} {} {}", a, b, c)
}

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

fn string_demo_1() {
    string_test(String::from("red"), &String::from("red"), "red");

    string_test(
        "blue".to_string(),
        &String::from("blue"),
        String::from("blue").as_str(),
    );

    // Here, in case of 3rd parameter rust is automatically
    // converting &String to &str.
    // If we pass reference to a String and a String slice
    // is expected, rust will automatically do that for us.
    string_test(
        "blue".to_string(),
        &String::from("blue"),
        &String::from("blue"),
    );
}

fn read_file_via_match() {
    match fs::read_to_string("logs.txt") {
        Ok(text_that_was_read) => {
            /*
                - We could have also used '&text_that_was_read';
                  'text_that_was_read.as_str()'
                - Also, why does &String::from(text_that_was_read)
                  throws an error?
                    - extract_errors(text: &str) -> Vec<&str>
                    - results.push(line)
            */
            let error_logs = extract_errors(text_that_was_read.as_str());

            match fs::write("errors-1.txt", error_logs.join("\n")) {
                Ok(..) => println!("Wrote errors-1.txt"),
                Err(reason_write_failed) => {
                    println!("Writing of errors-1.txt failed: {}", reason_write_failed)
                }
            }
        }
        Err(reason_text_was_not_read) => {
            println!("Failed to read file: {}", reason_text_was_not_read)
        }
    }
}

fn read_file_via_expect() {
    let text = fs::read_to_string("logs.txt").expect("failed to read logs.txt");
    let error_logs = extract_errors(text.as_str());
    fs::write("errors-2.txt", error_logs.join("\n")).expect("failed to write errors-2.txt");
}

fn extract_errors(text: &str) -> Vec<String> {
    /*
        ### 77. Understanding the Issue ###
            - Python: When we split a string we copy the split
              string items into an array.
            - Rust: When we split a string we get a vector of
              String slices Vec<&str> which refer to parts/slices
              of the original string.
                - Every 'String slice' points to the first
                  character of the word in the original string?
            - 'split_text' binding => Vec<&str>; &str => refers to
              portions of the original string
            - 'results' binding => Vec<&str>; &str => refers to
              portions of the original string which start with 'ERROR'
                - In the 'for' loop we are COPYING the reference
            - Issue: When an owner goes out of scope, the value
              owned by it is dropped (cleaned up in memory)
                - We have Vec<&str>; &str points to something that
                  doesn't exist anymore!
            - Solution: results => Vec<String>; line.to_string()
    */
    let split_text = text.split("\n");

    let mut results = vec![];

    for line in split_text {
        if line.starts_with("ERROR") {
            results.push(line.to_string());
        }
    }

    results
}

fn main() {
    read_file_via_match();

    read_file_via_expect();

    println!();

    result_demo_1();

    string_demo_1();
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
