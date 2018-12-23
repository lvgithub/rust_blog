fn main() {
    let a = [11,31,41,51,61];

    // 遍历key,value
    for (k,v) in a.iter().enumerate()   {
        println!("the key is:{},the value is:{}",k,v);
    }

    // 只遍历value
    for element in a.iter() {
        println!("the value is: {}", element);
    }
}