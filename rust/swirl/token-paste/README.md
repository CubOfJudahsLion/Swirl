This is a minimal library providing the single macro `token_paste!`.

**Usage**: invoke the macro with valid *ident* tokens, separated by spaces.

    use token_paste::token_paste;
    let some_variable = "Hello, token pasters!";
    println!("{}", token_paste!( some_  variable ));

At least one *ident* token is expected. While it seems a waste to use a token-
paster with a single token, this facilitates inclusion in variadic `macro_rules!`
constructions.


--6 Alex Feterman (<10951848+CubOfJudahsLion@users.noreply.github.com>)
