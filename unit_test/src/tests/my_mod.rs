use my_mod;

/*
  #[test] implies #[cfg(test)]. So this method is only compiled
  during unit tests(cargo test).
  Even if there are any compilation errors cargo build will succeed
  without any issues.
*/
#[test]
fn test_add_three() {
    assert_eq!(5, my_mod::add_three(2));
}


/*
This test will be ignored when running cargo test
*/
#[test]
#[cfg_attr(test, ignore)]
fn ignored_test() {
    assert_eq!(5, my_mod::add_three(2));
}

/*
 This method will get compiled during build(cargo build)
*/
fn other_fn() {
    my_mod::add_three(2);
}