struct Fibonacci {
  curr: u64,
  next: u64,
}

impl Iterator for Fibonacci {
  type Item = u64;

  fn next(&mut self) -> Option<u64> {
    let new_next = self.curr + self.next;

    self.curr = self.next;
    self.next = new_next;

    Some(self.curr)
  }
}

fn fibonacci() -> Fibonacci {
  Fibonacci { curr: 1, next: 1 }
}

fn main() {
  let mut sum: u64 = 0;
  for i in fibonacci().take_while(|&x| x < 4_000_000)
                      .filter(|&x| x % 2 == 0) { sum += i }
  println!("sum: {:?}", sum);
}

