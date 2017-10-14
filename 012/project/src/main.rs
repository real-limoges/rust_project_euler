fn is_prime(num: u64) -> bool {
  for i in 2..(num / 2 + 1) {
    if num % i == 0 {
      return false;
    }
  }
  return true;
}

struct Prime {
  curr: u64,
}

impl Iterator for Prime {
  type Item = u64;
  fn next(&mut self) -> Option<u64> {
    let mut new_next = self.curr + 1;
    while !is_prime(new_next) {
      new_next += 1;
    }
    
    self.curr = new_next;
    Some(self.curr)
  }
}

fn primes() -> Prime {
  Prime { curr: 1 }
}

fn main() {

  let mut number = 0u64;
  let first_100_p: Vec<u64> = primes().take(100).collect();

  for i in 1..1_000_000u64 {
    // Maps i to a mutable variable to keep loop invariant
    number += i;
   
    // Mutatable variable incrimented by number of divisors found 
    let mut number_div = 1u64;

    // This variable is the number being repeatedly divided
    let mut number_temp = number; 
    
    for &n in &first_100_p {
      let p = n;
      if number % p == 0 {
        // This allows us to express all numbers without looping through
        // each one by looking at exponents and prime factorization
        let mut exp = 1u64;
        
        while number_temp % p == 0 {
          exp += 1;
          number_temp /= p;
        }
        
        number_div *= exp;
      }
      if number_temp == 1 { break; }
    }
    
    if number_div > 500 {
      println!("Number is {}", number);
      return;
    }
  }
}
