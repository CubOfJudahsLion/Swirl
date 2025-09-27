use token_paste::token_paste;

#[test]
fn test() {
    let some_var = 10u8;
    assert_eq!(token_paste!(some_ var), some_var);
}
