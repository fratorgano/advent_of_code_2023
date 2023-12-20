pub fn gcd(n1:u64,n2:u64) -> u64 {
  if n2==0 {return n1};
  return gcd(n2, n1%n2)
}

pub fn lcm(nums:Vec<u64>) -> u64 {
  let mut res = nums[0];
  for i in 1..nums.len()  {
    res = nums[i]*res / gcd(nums[i], res)
  }
  res
}

#[cfg(test)]
mod tests {
  use super::*;
  #[test]
  fn test_gcd() {
    let result = gcd(4093, 3889);
    assert_eq!(1,result)
  }

  #[test]
  fn test_lcm() {
    let result = lcm(vec![4093, 3889, 3821, 3739]);
    assert_eq!(227411378431763,result)
  }
}