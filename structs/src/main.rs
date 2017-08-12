fn main() {
	let user1 = User {
		email: String::from("someone@example.com"),
		username: String::from("someusername123"),
		active: true,
		sign_in_count: 1,
	};

	// .. notation here means take the rest of the members from
	// user1; saves code
	let user2 = User {
		email: String::from("another@example.com"),
		username: String::from("anotherusername567"),
		..user1
	};

	// standard dot notation is uses to access members of a struct,
	// and can change values if the struct is mutable

	let origin = Point_3D(0, 0, 0);

	println!("{} {} {}", origin.0, origin.1, origin.2);

	// struct tuples are structs without names; means even if they
	// have the same data stored, its naming means that you can't
	// mix 2 tuple structs up. eg. colour in RGB, or a point in 3D
}

fn build_user(email: String, username: String) -> User {
	// don't need to explicitly define key val pairs for 
	// email and username if the variables have the same name
	// as the member names. Makes defs more concise
    User {
    	email,
    	username,
    	active: true,
    	sign_in_count: 1,
    }
}

struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

struct Point_3D(i32, i32, i32);