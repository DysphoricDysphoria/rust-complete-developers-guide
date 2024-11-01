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

fn print_elements(elements: &Vec<String>) {
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

fn main() {
    let colors = vec![
        String::from("red"),
        String::from("green"),
        String::from("blue"),
    ];

    print_elements(&colors);

    println!();

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
}
