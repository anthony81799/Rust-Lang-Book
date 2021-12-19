use adder;
mod common;

#[test]
fn it_adds_two() {
    common::setup();
    assert_eq!(4, adder::add_two(2));
}
// to run only integration tests
// cargo test --test integration_test
// to share data between tests make a common folder in tests
// Then put shared data in mods.rs
