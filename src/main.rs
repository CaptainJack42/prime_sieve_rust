use clap::{App, Arg};
use rand::Rng;

pub struct PrimeSieve {
	sieve_size: usize,
	raw_bits: Vec<bool>,
}

fn main() {
	let matches = App::new("primes")
		.version("0.1.0")
		.author("CaptainJack42 <https://github.com/CaptainJack42>")
		.about("spits out prime numbers")
		.arg(
			Arg::with_name("single")
				.short("s")
				.long("single")
				.takes_value(false)
				.help("Produces a single prime number choosen randomly"),
		)
		.arg(
			Arg::with_name("max")
				.short("m")
				.long("max")
				.takes_value(true)
				.default_value("1000000")
				.help("The maximum Number to calculate primes to"),
		)
		.get_matches();

	let max = matches
		.value_of("max")
		.unwrap_or_default()
		.parse::<usize>()
		.expect("Not a number!");
	println!("max is: {}", max);

	let max = max + 1;

	let mut sieve = PrimeSieve {
		sieve_size: max,
		raw_bits: vec![true; max],
	};

	sieve.raw_bits[0] = false;
	sieve.raw_bits[1] = false;

	let primes = calc_primes(&mut sieve);

	if matches.is_present("single"){
		print_result(true, &primes);
	} else {
		print_result(false, &primes);
	}
}

fn calc_primes(sieve: &mut PrimeSieve) -> Vec<usize> {
	let mut primes: Vec<usize> = Vec::new();
	let upper_limit: usize = (sieve.sieve_size as f64).sqrt() as usize + 1;

	primes.push(2);
	eliminate_multiples(&mut sieve.raw_bits, 2);

	for i in (3..upper_limit).step_by(2) {
		if sieve.raw_bits[i] {
			primes.push(i);
			eliminate_multiples(&mut sieve.raw_bits, i);
		}
	}

	for i in upper_limit..sieve.sieve_size {
		if sieve.raw_bits[i] {
			primes.push(i);
		}
	}

	return primes;
}

fn eliminate_multiples(bitmap: &mut Vec<bool>, step: usize) {
	for i in (step..bitmap.len()).step_by(step) {
		bitmap[i] = false;
	}
}

fn print_result(single: bool, primes: &Vec<usize>) {
	if single {
		println!("{}", primes[rand::thread_rng().gen_range(1..primes.len())])
	} else {
		println!("{:?}", primes);
		println!("Total: {} primes", primes.len());
	}
}
