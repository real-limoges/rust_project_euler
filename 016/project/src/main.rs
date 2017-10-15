extern crate num;

use num::bigint::{BigUint, ToBigUint};



fn main() {
  let big_two: BigUint = 2.to_biguint().unwrap();
  let big_num: BigUint = num::pow(big_two, 1000);
  let big_string: String = big_num.to_str_radix(10);
  let big_s_vec: Vec<&str> = big_string.split("").collect();

  let mut big_i_vec = Vec::new();
/*
  let big_i_vec: Vec<&u32> = big_s_vec.into_iter()
                                      .map(|x| x.parse::<u32>().unwrap())
                                      .rev().collect();
*/

  for v in big_s_vec {
    let foo: u32 = v.parse::<u32>().unwrap_or_default();
    big_i_vec.push(foo);
  }
  let summer: u32 = big_i_vec.iter().sum();
  println!("{:?}", summer);
} 

