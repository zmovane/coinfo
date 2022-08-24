use adder_lib::add;
#[test]
fn it_works_with_assert() {
    let result = add::add(2, 2);
    assert_eq!(result, 4);
}

#[test]
fn it_works_with_result() -> Result<(), String> {
    if 2 + 2 == 4 {
        Ok(())
    } else {
        Err(String::from("two plus two does not equal four"))
    }
}
