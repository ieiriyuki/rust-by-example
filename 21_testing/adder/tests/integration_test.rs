// extern crate we're testing, same as any other code would do.
extern crate adder;

// importing common module.
mod common;

#[test]
fn test_add() {
    assert_eq!(adder::add(3, 2), 5);
}
