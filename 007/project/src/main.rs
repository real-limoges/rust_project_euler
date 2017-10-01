
// Simple naive function to determine if num is prime
fn is_prime(num: u64) -> bool {
  let upper_f = num as f64;
  let square_root = upper_f.sqrt() as u64 + 1; 
  
  for i in 2..square_root {
    if num % i == 0 { return false; }
  }
  return true;
}

// Simple struct of a prime number - created mostly for the Iterator
struct PrimeNum {
  curr: u64,
}

// Implements an Iterator for the PrimeNum holder
impl Iterator for PrimeNum {
  
  // Boilerplate to implement Iterator
  type Item = u64;
  
  // Creates a next function that returns an Option<u64> 
  fn next(&mut self) -> Option<u64> {
    
    let mut new_next = self.curr + 1;
    
    // Iterates until it finds the next prime number
    while !is_prime(new_next) {
      new_next += 1;
    }

    // Overwrites the PrimeNum's curr u64
    self.curr = new_next;

    // Returns the curr as a Some()
    Some(self.curr)
  }
}
 
// Function creates the primes and returns them
// Initialized at 1
fn primes() -> PrimeNum {
  PrimeNum { curr: 1 }
}


fn main() {
  let mut summer: u64 = 0;
  let mut counter: u64 = 0;

  for num in primes() {
    counter += 1;
    summer += num;

    if counter == 3 {
      break;
    }
  }
  println!("{:?}", summer);
  

}
