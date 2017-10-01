// This is lifting from some of my earlier code (003)
fn is_prime(num: u64) -> bool {
  for i in 2..(num / 2 + 1) {
    if num % i == 0 {
      return false;
    }
  }
  return true;
}

// Struct to hold the current Prime
struct PrimeNum {
  curr: u64,
}

// Implements the Iterator for Prime numbers
impl Iterator for PrimeNum {
  type Item = u64;
  fn next(&mut self) -> Option<u64> {
    let mut new_next = self.curr + 1;
    while !is_prime(new_next){
      new_next += 1;
    }
    
    self.curr = new_next;
    
    Some(self.curr)
  }
}

// Create fuction to serve the primes over. initializes to 1
fn primes() -> PrimeNum {
  PrimeNum { curr: 2 }
}


fn main() {
  let mut summer: u64 = 0;
  let mut counter: u64 = 0; 
  for i in primes() {
    if i < 2000000 { summer += i; counter += 1; }
    else { break; } 
  }
  println!("Sum is {} with {} primes", summer, counter)
}
