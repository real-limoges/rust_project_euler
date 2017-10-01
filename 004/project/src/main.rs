fn is_palindrome(x: u64, y: u64) -> bool {
  let mult_str: String = (x*y).to_string();

  mult_str == mult_str.chars().rev().collect::<String>()
}

fn main() {
  let mut largest: u64 = 0;

  for i in 100..999 {
    for j in 100..999 {
      if is_palindrome(i, j) && i*j > largest { largest = i * j }
    }
  }
    
  println!("{:?}", largest); 
  
}
