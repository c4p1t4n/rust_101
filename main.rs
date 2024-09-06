fn main() {
  let mut a = vec![1, 2, 3];
  let b = a;  //  &mut borrow of `a` starts here
  // some code
    b[1] = 123;

  println!("{:?}", a); // trying to access `a` as a shared borrow, so giving an error
}                  //  &mut borrow of `a` ends here