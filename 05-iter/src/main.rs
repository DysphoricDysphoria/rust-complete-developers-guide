/*
    ### Iterator ###
        - Used to iterate over any kind of data structure
        - We've already been using them - they are used
        behind the scenes when you write a for loop
        - Follow all the same rules of ownership,
        borrowing, lifetimes
        - Use the Option enum
*/

/*
    ### We usually don't call 'next' on an iterator
        manually. Instead... ###
        - Option 1: Use a for loop. Automatically creates
        an iterator and calls 'next' on it
        - Option 2: Use iterator adaptors and consumers
        like 'for_each', 'collect', 'map', etc.
*/

// TODO: Clippy with rust-analyzer?

/*
    ### Vector slice - &[<Type>] ###
        - Vec<String> => &[String]
        - It is a struct
            - Pointer to data
            - Length
        - Similar to 'String slice' and avoids the same
        set of problems
        - Why use 'print_elements' with Vector slice?
            - Single function that can work with either
            a full vector or just a portion of a vector
*/

fn print_elements(elements: &[String]) {
    /*
        ### for loop will ... ###
            - Automatically create an iterator for the
            vector
            - Call 'next' on the vector and unwrap the
            Option that comes back
            - Break once 'next' returns a None
    */
    let use_for_loop = false;

    if use_for_loop {
        for element in elements {
            println!("{}", element);
        }
    } else {
        /*
            ### Iterator consumers ###
                - Iterators are 'lazy'. Nothing happens until...
                    - We call 'next'
                    - We use a function that calls 'next' automatically
                        - These functions are called consumers (they
                        are (kinda?) consuming values out of the
                        iterator). Ex: for_each()
                        - for_each() is an iterator 'consumer'
                        - It will repeatedly call 'next()' on the
                        iterator until it gets 'None'
        */
        let use_iter_adaptor = true;

        if use_iter_adaptor {
            /*
                ### Iterator adaptors ###
                    - .map() is an iterator adaptor
                    - Adaptors create a step in a processing pipeline,
                    but don't actually cause any iteration
                    - .map() transforms each item (a copy?) of 'element'
                    and forwards it to 'for_each'
            */
            elements
                .iter()
                .map(|el| format!("{} {}", el, el))
                .for_each(|el| println!("{}", el)); // '|el| func body...' is a closure function
        } else {
            elements.iter().for_each(|el| println!("{}", el));
        }
    }
}

fn shorten_string(elements: &mut Vec<String>) {
    /*
        - iter()
            - This will give us a read-only reference
            to each element
        - iter_mut()
            - This will give us a mutable reference to
            each element
        - into_iter()
            - This will give us ownership of each elem,
            unless called on a mutable ref to a vector
    */
    elements.iter_mut().for_each(|el| el.truncate(1));
}

fn main() {
    let mut colors = vec![
        String::from("blue"),
        String::from("green"),
        String::from("red"),
    ];

    /*
        - colors.iter() creates a new data structure
          Iter<String>
            - Iter is separate from the Vector etc.
              we are iterating over
            - Iter is a struct which has a couple
              of fields inside it
                - Pointer to data (Vec<String>, in this
                  case)
                - Pointer to current position ("red")
                - Pointer to end (pointing outside the
                  bounds of vector)
            - next() => Some() or None
    */
    let mut colors_iter = colors.iter();
    // We need to use 'mut' above because we are changing
    // something (pointer to current position) within the
    // iterator every time we use 'next()'

    println!("colors_iter.next() {:#?}", colors_iter.next());
    println!("colors_iter.next() {:#?}", colors_iter.next());
    println!("colors_iter.next() {:#?}", colors_iter.next());
    println!("colors_iter.next() {:#?}", colors_iter.next());

    println!();

    // With 'print_elements(elements: &Vec<String>)'
    // we are only allowed to pass in the full Vector.
    // If we modify it to 'print_elements(elements:
    // &[String])', we can pass in a portion of the
    // Vector as well
    print_elements(&colors);

    println!();

    print_elements(&colors[1..3]);

    println!();

    shorten_string(&mut colors);

    println!("{:#?}", colors);
}
