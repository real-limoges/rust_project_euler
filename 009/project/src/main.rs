fn is_triplet(x: u64, y: u64, z: u64) -> bool {
  if x.pow(2) + y.pow(2) == z.pow(2) { return true; }
  return false;
}
  

fn main() {
  
  for i in 1..1000 {
    for j in i..1000 {
      for k in j..1000 {
         if is_triplet(i, j, k) && i + j + k == 1000 {
            println!("Triplet is {}, {}, {}", i, j, k);
            println!("Product is {}", i*j*k);
            return;
        }
      }
    }
  }
}
