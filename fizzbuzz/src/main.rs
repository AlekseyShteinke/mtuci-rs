fn main() {
	for i in 1..=100{
		let mut flag1 : bool = false;
		let mut flag2 : bool = false;
		if i%3 == 0 {
			flag1 = true;
			print!("Fizz");
		}
		if i%5 == 0 {
			flag2 = true;
			print!("Buzz");
		}
		if ! flag1 && !flag2 {
			print!("{}", i)
		}
		println!()
	}
}