/*
    ### 73. The Stack and Heap ###
        - Stack
            - Fast, but limited size (2 - 8 MB)
        - Heap
            - Slow, but can grow to store a lot of data
            (Gigabytes worth of data)
        - Data/Data segment/Read-only-data segment/Static
        segment
            - Stores literal values that we write directly
            into our source code
                - Ex: let num = 45; let color = "red"; =>
                45 and "red" are stored into 'Data'
        - Ex:
            - let numbers = vec![1, 2, 3, 4, 5];
                - The initial values 1, 2, 3, 4, 5 will be
                stored into 'Data' and when we make the
                vector, these will be copied into the
                'Heap'
                    - During parsing, Rust might create
                    1 - 5 in 'Data' and then later copy
                    it into the 'Heap'?
            - Super common pattern
                - Stack stores metadata about a data
                structure (in this case 'numbers')
                    - pointer to value(s) | length |
                    capacity
                - Heap stores the actual data
                    - Actual values of the vector are
                    stored here
                - Avoids running out of memory in the
                stack if the data structure grows to hold
                a lot of data
        - Corner case
            - If a data structure owns another data
            structure, the child's metadata will be placed
            on the heap
            - Ex: let vec_of_num = vec![ vec![1, 2, 3, 4,
            5] ];
                - Metadata for nested vector will be stored
                inside the 'Heap'
                - Metadata for parent vector will be stored
                inside the 'Stack'
*/

/*
    ### 74. Strings, String Refs, and String Slices ###
        - String
            - Struct (pointer to text | length of string in
            heap | capacity of string in heap) in Stack
            - Pointer points to value in 'Heap'
            - Uses memory in 'Stack' & 'Heap'
        - &String (read-only)
            - Reference to 'String' (All the work of
            String + REFERENCE) | Stored in Stack
            - Uses memory in 'Stack'
        - &str (String slice - read-only)
            - Struct (pointer to text | length of string)
            in Stack
            - Pointer points to value in 'Heap' or 'Data'
            (Heap-allocated or Data-allocated text)
            - Uses memory in 'Stack'
            - String to String slice
                let color = String::from("red");
                let c = color.as_str();
                    - In this case 'c' points to "red" in
                    'Heap'!
*/

/*
    ### 75. When to Use Which String ###
        - &String and &str - both provide a read-only
        reference to text data - Why two different types?
            - &str lets you refer to text in the data
            segment without a 'Heap' allocation =>
            slightly more performant
                - If we do it solely via 'String', it will
                be a lot more work
            - &str lets you 'slice' (take a portion) of
            text that is already on the heap
                let color = String::from("blue"); => String
                let portion = &color[1..4]; => &str
                - When we do &color[1..4], behind the
                scenes '&str' is created in Stack and
                &color[1..4] refers to 'lue' portion of
                the same heap-allocated 'blue'
                - Again, it will be a lot more work if we
                do this via 'String'

        - Usage
            - String
                let color = String::from("red");
                - Use anytime we want ownership of text
                - Use anytime we want text that can
                grow or shrink
            - &String
                let color = String::from("red");
                let color_ref = &color;
                - Rarely used (usually never)
                - Rust will automatically turn &String
                into &str for you
            - &str
                let color = String::from("red");
                let c = color.as_str();
                - Use anytime you don't want to take
                ownership of text
                - Use anytime you want to refer to a
                'portion' of a string owned by something
                else.

        - String slice (&str) can either point at text
        stored in the 'Data segment' or text stored in
        the 'Heap' that belongs to a String
*/

fn string_test(a: String, b: &String, c: &str) {
    println!("{} {} {}", a, b, c);
}

/*
    ### Result enum ###
        - Result enum is used when we need to know if
        something worked or failed
            - Ok() variant is used when something went
            well
            - Err() variant is used when something bad
            happened

    ### Option enum ###
        - Option enum is used when we need to know if a
        value is present or not
            - Some() variant is used when we have a value
            - None variant is used when there is no value
*/

fn main() {}
