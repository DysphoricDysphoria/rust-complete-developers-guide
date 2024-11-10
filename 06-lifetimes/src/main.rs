/*
    ### Lifetime annotations ###
        - Used with functions, structs, enums, and more
        - Help the compiler make sure refs won't outlive
        the value they refer to
        - This will seem like something the compiler
        should do on its own
*/

/*
    ### Notes ###
        - A lifetime is how long a binding can be used
*/

/*
    ### What Lifetime Annotations Are All About? ###
        - If we have a function that takes in two or more
        refs and returns a ref, Rust will make a huge
        assumption => Rust assumes that the return ref
        will point at data referred to by one of the
        arguments
        - Rust will not analyze the body of our function
        to figure out whether the return ref is pointing
        at the first or second arg
            - Rust still wants to know whether the
            returned reference points to the first or
            second argument
            - Rust expects the user to use lifetime
            annotation so that it knows whether the
            returned reference points to the first or
            second argument

    ### Common Questions Around Lifetimes ###
        - To clarify which ref the return ref is pointing
        at, we have to add lifetime annotations
        - QUESTION: Why does it matter whether the return
        ref points at the first or second arg?
            - More or less the same code can work or break
            depending upon what we are returning from the
            function.
        - QUESTION: Why doesn't Rust analyze the function
        body to figure out if the returned ref points at
        the first or second arg?
            - The documentation of Rust also includes
            lifetime annotations. If we relied on Rust to
            figure out the lifetimes, we wouldn't know if
            the returned ref uses the first or second arg.
            In this case, we won't know which code would
            work and which not.
            - WITH lifetime annotation:
                fn split<'a>(s: &'a str, pattern: &str) ->
                &'a str {}
            - WITHOUT lifetime annotation:
                fn split(s: &str, pattern: &str) ->
                &str

    ### Lifetime Elision ###
        - If we have a function that takes in one ref
        and returns a ref, Rust will make the assumption
        that the returned ref will point at data referred
        to by the argument. Adding lifetime annotations
        is optional in this case.

        - We have to think about annotations anytime a
        function receives a ref and returns a ref
        - We can omit annotations in two scenarios.
        (There are more explicit rules for this, these
        two are the most common)
            - Function that takes one ref + any number
            of values + returns a ref
            - Method that takes &self and any number of
            other refs + returns a ref. Rust assumes
            the returned ref will point at &self
                - In this case, if the Rust's assumption
                is incorrect, we need to add lifetime
                annotation.

        - Omitting lifetime annotations is referred to
        as elision
            - I removed/elided the lifetime annotations.
            - We can remove/elide the annotations.
            - Think about removal/elision of the
            annotations.
            - Pronunciation => eeh-lah-eed (elide)

        - Adding lifetime annotations doesn't change how
        our code runs at all. It doesn't prolong a
        reference, it doesn't make it live longer or
        anything like that. => It is communicating the
        relationship b/w the returned reference and the
        argument reference(s).

*/

/*
    <'a> - There is a type of ref called 'a'
    &'a  - This first ref is of type 'a'
    &'a  - This returned ref is also of type 'a'
*/
fn next_language<'a>(languages: &'a [String], current: &str) -> &'a str {
    let mut found = false;

    for lang in languages {
        if found {
            return lang;
        }

        if lang == current {
            found = true;
        }
    }

    languages.last().unwrap()
}

// We are omitting lifetime annotation here.
// Why? => 'Function that takes one ref + any number
// of values + returns a ref'
fn last_language(languages: &[String]) -> &str {
    languages.last().unwrap()
}

fn longest_language<'a>(lang_a: &'a str, lang_b: &'a str) -> &'a str {
    if lang_a.len() >= lang_b.len() {
        lang_a
    } else {
        lang_b
    }
}

fn main() {
    let languages = vec![
        String::from("Rust"),
        String::from("Golang"),
        String::from("TypeScript"),
    ];

    let result_1 = next_language(&languages, "Rust");

    println!("{}", result_1);

    let result_2 = last_language(&languages);

    println!("{}", result_2);

    let result_3 = longest_language("Erlang", "Golang");

    println!("{}", result_3);
}
