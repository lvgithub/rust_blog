fn main() {
    let a1 = 5;
    let a2: i32 = 5;
    // let a3: u32 = 5;

    //类型不一样，值一样也是不相等的
    println!("a1 == a2: {}", a1 == a2);
    assert_eq!(a1, a2);
    // assert_eq!(a1, a3); // throw error

    // 解构
    let (a,  b): (bool, bool) = (true, false);
    println!("a={},b={}", a, b);

    // char
    let c = 'f';
    println!("char:{}", c);

    // float
    let float: f64 = 1.23e+2;
    println!("float:{}", float);

    // zero
    let zero = (float-123.00).abs();
    println!("zero:{}", zero);

    // 二进制
    let bin = 0b110;
    println!("bin:{}", bin);

    // 八进制
    let oct = 0o11;
    println!("oct:{}", oct);

    // 十六进制
    let hex = 0xA;
    println!("hex:{}", hex);

    // b"..."	字节字符串字面值; 构造一个 [u8] 类型而非字符串
    let byte = b"hello byte";
    let strq = String::from_utf8(byte.to_vec()).unwrap();
    println!("byte:{}",strq);

    let heart_eyed_cat = "😻";
    println!("cat:{}",heart_eyed_cat);
}
