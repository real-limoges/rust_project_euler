struct Collatz {
  curr: u64,
}

impl Iterator for Collatz {
  type Item = u64;
  fn next(&mut self) -> Option<u64> {
    if self.curr % 2 == 0 { self.curr = self.curr / 2; }
    else { self.curr =  3u64 * self.curr + 1u64; }
    Some(self.curr)
  }
}

fn collatz(start: u64) -> Collatz {
  Collatz { curr: start }
}

fn main() {
  let mut max_coll_len: usize = 0;
  let mut max_coll_i: u64 = 0; 
  
  for i in 2u64..1_000_000u64 { 
    let ith_collatz: Vec<u64> = collatz(i)
                                  .take_while(|x| x > &1u64)
                                  .collect();
    // Add 2 for 0th and last elements
    let coll_len: usize = ith_collatz.len() + 2;

    if coll_len > max_coll_len {
      max_coll_len = coll_len;
      max_coll_i = i;
    }
  }
  println!("Max i is {:?}", max_coll_i);
  println!("Max len is {:?}", max_coll_len);
}
