
### 默认安装路径
```
cargo: ~/.cargo/bin             # rust执行文件目录就在这里（环境变量地址）  
rustup: ~/.rustup/toolchains    # 工具执行文件地址
```

### 版本
```
# Rust 提供了三个版本渠道：nightly，beta，还有stable。不稳定特性只在 Nightly 
# 切换版本: rustup default [nightly|beta|stable]
```

### 编译
```
cargo build               # 编译
cargo build --release     # 编译优化
cargo run                 # 编译后同时执行
cargo check               # 检查
```
### [安装vscode环境](./vscode_guide.md)
### 基础语法
```
# let foo = 5; // 不可变
# let mut bar = 5; // 可变
# 数字可以加上前缀 0x、0o、0b 分别表示十六进制数、八进制数、二进制数
# 1_000 等同于 1000， 0.000_001 等同于 0.000001。
# Rust 的 char 类型代表了一个 Unicode 标量值
```

## [cargo-watch(watch改动，自动编译)](https://github.com/passcod/cargo-watch)

## 模块
* mod 关键字创建一个新模块
* use 关键字允许您使用模块（在要使用这些定义的范围内公开它们）
* pub 关键字将模块的元素设置为公有（否则它们是私有的）

## 类型
* 布尔类型：有两个值true和false。
* 字符类型：表示单个Unicode字符，存储为4个字节。
* 数值类型：分为有符号整数 (i8, i16, i32, i64, isize)、 无符号整数 (u8, u16, u32, u64, usize) 以及浮点数 (f32, f64)。
* 字符串类型：最底层的是不定长类型str，更常用的是字符串切片&str和堆分配字符串String， 其中字符串切片是静态分配的，有固定的大小，并且不可变，而堆分配字符串是可变的。
* 数组：具有固定大小，并且元素都是同种类型，可表示为[T; N]。
* 切片：引用一个数组的部分数据并且不需要拷贝，可表示为&[T]。
* 元组：具有固定大小的有序列表，每个元素都有自己的类型，通过解构或者索引来获得每个元素的值。
* 指针：最底层的是裸指针*const T和*mut T，但解引用它们是不安全的，必须放到unsafe块里。
* 函数：具有函数类型的变量实质上是一个函数指针。
* 元类型：即()，其唯一的值也是()