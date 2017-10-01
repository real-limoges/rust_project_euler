fn is_divisable(num: u64) -> bool {
  for i in 2..20 {
    if num % i != 0 { return false }
  }
  return true
}

fn main() {
    let mut done = false; 
    
    let mut counter: u64 = 21; 
    
    while !done {
      if is_divisable(counter) {
        break;
      }
      counter += 1;
    }
    
    println!("{}", counter);

}
