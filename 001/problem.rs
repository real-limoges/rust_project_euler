

fn is_mult(n: u32) -> bool {
  n % 3 == 0 || n % 5 == 0
}

fn main() {
  let add_mults: u32 = 
      (0..1000).filter(|&n| is_mult(n))
               .fold(0, |sum, i| sum + i);
  println!("Add Mults: {:?}", add_mults);
}
