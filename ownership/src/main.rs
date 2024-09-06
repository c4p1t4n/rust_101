fn main() {
   //let _s = "hello";
   let mut _s = String::from("heloo");
   _s.push_str(", world");
   println!("{}",_s);
}
//  When we dealing with primitive types they will be store in the stack example
fn _test_primitive_type(){
   let x = 5;
   let y = x;
}
//  In the function bellow we have the heap empty and the stack like
// |y|int|5|
// |x|int|5|
//  Because we don't use the heap
// but in the context bellow we have a different result
fn _test_string_type(){
   let mut s1 = String::from("hello");
   let mut s2 = s1;
   // s1.push_str(" teste");
   // println!("{},{}",s1,s2);
}
// In this case we have the heap like 
// index|value
//  0     h
//  1     e
//  2     l
//  3     l
//  4     o
//  and in the stack we have
// s1.prt = {point to the beginning of the string (index 0)}
// str.len = 5
// str.capacity = 5
// s2.ptr = point to the same piece of the memory 
// s2.len = 5
// s2.capacity = 5