

fn is_prime(num: u64) -> bool {
  let upper_f = num as f64;
  let square_root = upper_f.sqrt() as u64 + 1; 
  
  for i in 2..square_root {
    if num % i == 0 { return false }
  }
  return true
}

fn main() {
  assert!( is_prime(3), true );

}
