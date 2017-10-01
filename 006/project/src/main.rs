fn main() {
  let upper: u64 = 101;
  
  let squares: u64 = (1..upper).fold(0, |tot, val: u64| tot + val*val);
  let squared: u64 = (1..upper).fold(0, |tot, val: u64| tot + val).pow(2);

  println!("{:?}", squared - squares);

}
