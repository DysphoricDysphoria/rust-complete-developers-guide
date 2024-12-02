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

fn main() {}
