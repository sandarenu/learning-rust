struct Dog {
    name: &'static str,
    age: i32
}


impl Dog {

	//Methods are basically functions that take a first argument named self
	fn say_hello(&self) {
		println!("Hello everyone, I'm {:?}", self.name);
	}

	//Functions are like static methods in Java
	fn scientific_name() {
		println!("Dog's scientific name is 'Canis lupus familiaris'");
	}

	//Function as a constructor
	fn new(name: &'static str, age: i32) -> Dog {
		Dog { name: name, age: age}
	}
}

fn main() {
    let a = Dog { name: "Brown", age: 10 };
    println!("{:?} is {:?} years old", a.name, a.age );
    a.say_hello();

    Dog::scientific_name();
    // a.scientific_name(); you can't call this

    Dog::new("Rosy", 5).say_hello();

}
