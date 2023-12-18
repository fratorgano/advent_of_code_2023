pub fn print_matrix<T: std::fmt::Debug>(matrix: &Vec<Vec<T>>) {
  for line in matrix {
    for elem in line {
      print!("{:?}",elem)
    }
    println!();
  }
  println!();
}