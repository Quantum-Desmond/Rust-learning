#[derive(Debug)]
enum Message {
}

impl Message {
    fn call(&self) {
        unimplemented!();
    };
}

fn main() {
	// calling of enum values
	let home = IpAddr::V4(127, 0, 0, 1);
	let loopback = IpAddr::V6(String::from("::1"));

}
