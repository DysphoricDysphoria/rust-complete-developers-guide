/*
    ### Iterator ###
        - Used to iterate over any kind of data structure
        - We've already been using them - they are used
          behind the scenes when you write a for loop
        - Follow all the same rules of ownership,
          borrowing, lifetimes
        - Use the Option enum
*/

fn main() {
    let colors = vec![
        String::from("red"),
        String::from("green"),
        String::from("blue"),
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

    println!("{:#?}", colors_iter.next());
    println!("{:#?}", colors_iter.next());
    println!("{:#?}", colors_iter.next());
    println!("{:#?}", colors_iter.next());
}
