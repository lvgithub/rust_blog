
控制流
* 本笔记，只记录rust控制流的特殊点，如需全面请参考官网文档哦。
* 简单的if else 等语句其他语言一样，就不列举了

### 在let语句中使用if表达式
```rust
## 重点：5，6背后是不能有分号的哦（有分号就成为了语句）
fn main() {
    let condition = true;
    let _number = if condition {
        5
    } else {
        6
    };
}
## 也可以这样写，number == 7
fn main() {
    let condition = true;
    let _number = if condition {
        5;
        7
    } else {
        6
    };
}
```

### 从循环返回
``` rust
fn main() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;

        /* 还可以这样写
            let ret = counter * 2; 
            break ret;
        */

        /* 但这样是错误的写法
            counter * 2;
            break;
        */
        }
    };

    assert_eq!(result, 20);
}
```

### for 循环
``` rust
fn main() {
    let a = [10, 20, 30, 40, 50];

    for element in a.iter() {
        println!("the value is: {}", element);
    }
}
```