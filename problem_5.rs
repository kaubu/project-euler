// Find the smallest positive number that is evenly divisible by all of the numbers from 1 to 20
// https://projecteuler.net/problem=5
// Time: 1.4443334s

const MIN: i32 = 1;
const MAX: i32 = 20;

fn main() {
	let mut counter = MIN;

	loop {
		for n in MIN..(MAX+1) {
			if (counter % n) == 0 {
				if n == MAX {
					println!("{}", counter);
					std::process::exit(0);
				};
			} else {
				break;
			}
		}	

		counter += 1;
	}
}
