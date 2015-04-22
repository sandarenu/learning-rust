// Learning How to write macros

// Simple macro. `say_hello` is the name
macro_rules! say_hello {
	// `()` tell that macro don't take any arguments
	() => (
			println!("Hello, I'm a Macro");
		)
}	

// Macro taking expression as a parameter
macro_rules! hello_name {
	($name:expr) => (
			println!("Hello {:?} = {:?}", stringify!($name), $name);
		)
}

// Macro with repeating arguments
macro_rules! more_names {
	//Base case
	($x:expr) => (
			println!("Hello {:?}", $x);
		);

	// `+` is used to indicate repeating arguments.
	// `$x` followed by atleast one `$y`
	($x:expr, $($y:expr) ,+) => (
			println!("Hello {:?}", $x);
			more_names!($($y),+);
		)
}

// Macro with different separator template.
macro_rules! separator_tpl {
	// Arguments need not be separated by a comma `,`. It can be any template
	// Here I've use `=>` a separator
	($left:expr => $right:expr) => (
			println!("{:?} => {:?}", stringify!($left), stringify!($right));
		)
}


fn main() {
    say_hello!();

    hello_name!("Sandarenu");
    hello_name!(1 * 2);

    more_names!("Lakruwan", "Sandarenu", "Siheli", "Hiruki");

    separator_tpl!( 1 * 1 => "one" );
}
