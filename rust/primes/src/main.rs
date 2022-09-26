use std::time::Instant;

fn main() {
    let now = Instant::now();

    let mut primes = vec![];

    for num in 2..100_001 {
        let mut prime = true;
        for p in primes.iter() {
            if num % p == 0 {
                prime = false;
                break;
            }
        }
        if prime {
            primes.push(num)
        }
    }

    let elapsed_time = now.elapsed();

    println!("Found {} prime numbers", primes.len());
    println!("Should be 9592");
    println!("Took {} seconds", elapsed_time.subsec_millis() as f32 / 1000.0);
}
