fn main() {
    let mut s1 = String::from("hello");

    let len = calculate_length(&mut s1);

    println!("The length of {} is {}", s1, len);
}

fn calculate_length(s: &mut String) -> usize { // s is a ref to String
	s.push_str(", world!");
    s.len()
} // Here, s goes out of scope but some_string is fine
