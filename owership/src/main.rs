fn main() {
    let s = String::from("hello");  // s 进入作用域

    run_move(s);                    // s 的值移动到函数里,s失效
                                    // 因此到这里，s不再有效
    /* 
    将会报错：因为s已经被move
    报错信息：will error value borrowed here after move
    println!("s:{}",s);            
    */
    let x = 5;                      // x 进入作用域

    run_copy(x);                    // x 应该移动函数里
    println!("x:{}",x);             // 因为 x 是 栈变量，因为不会被 move 使失效

    let s2 = String::from("world"); // s2 进入作用域
    run_not_move(&s2);              // 传递说s2的引用作为参数
    //&s2.push_str("push"); //cannot borrow as mutable 
    println!("s2:{}",s2);           // 正常打印
} 
//  x 移出了作用域，
//  s 移出了作用域但,因为 s 的值已被移走，所以不会有特殊操作
//  s2 移除了作用域，调用drop释放内存

fn run_move(some_string: String) { // some_string 进入作用域
    println!("run_move:{}", some_string);
} // 这里，some_string 移出作用域并调用 `drop` 方法。占用的内存被释放

fn run_copy(some_integer: i32) { // some_integer 进入作用域
    println!("run_copy:{}", some_integer);
} // some_integer 移出作用域

fn run_not_move(some_string_ref: &String){ //some_string_ref 是指向说s2的引用，不会触发move
    println!("run_not_move:{}", some_string_ref);
    // some_string_ref.push_str(",okok"); //will error
}