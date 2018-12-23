fn main() {
    let mut s = String::from("hello");

    let r1 = &mut s;
    let r2 = &mut s;
    println!("s:{},{}",r1,r2);//second mutable borrow occurs here
}
