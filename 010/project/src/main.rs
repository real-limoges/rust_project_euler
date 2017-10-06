fn main() {
  
  let mut summer = 0;
  const SIZE: usize = 2000000;

  // fixed array of size SIZE initialized to true
  let mut slots: [bool; SIZE] = [true; SIZE];
 
  // initially mark 0 and 1 as not prime 
  slots[0] = false;
  slots[1] = false;

  // for each number, 
  for jump in 2..(SIZE/2) {
    // Current starting position. Mark off every multiple as false
    let mut pos = jump;
    while pos < (SIZE - jump) {
      pos += jump;
      slots[pos] = false;
    }
  }

  for (index, prime) in slots.into_iter().enumerate(){
    if *prime {
      summer += index;
    }
  }

  println!("The Sum is {:?}", summer);
}
