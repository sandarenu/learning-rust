
pub mod my_mod;

mod tests {
	mod my_mod;
}

pub fn add_two(a: i32) -> i32 {
    a + 2
}


/// Both unit test and funtion are in the same file. 
///
/// Even though this is the commong way of writting unit tests, it is not the 
/// best **idiomatic** way of writting unit tests.
///
#[test]
fn test_add_two() {
    assert_eq!(4, add_two(2));
}
