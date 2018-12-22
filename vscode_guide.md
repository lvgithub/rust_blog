
## 前提
 * 你已经知道如何搭建rust环境、或已成功安装rust环境
 * 只适配vscode编辑器

## 第一步:安装rustup
```
## 安装好rust记得配置环境变量
curl https://sh.rustup.rs -sSf | sh
```

## 第二步:安装依赖
```
rustup component add rust-src       ##
rustup component add rustfmt        ## 代码格式化
rustup component add clippy         ## 一个代码小工具集用于捕获常见错误，以及提高Rust代码质量
rustup component add rust-analysis  ## 处理rls数据
rustup component add rls            ## Rust语言服务器（将IDE的一些编程语言相关的部分由单独的服务器来实现）
```

## 第三步: 安装vscode插件

## 
* [rls(支持代码提示、格式化、代码片段等)](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust) 

* [CodeLLDB(调试工具)](https://github.com/vadimcn/vscode-lldb/blob/master/MANUAL.md) 




