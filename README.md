# rfit

## 1. 简介

用于学习 Rust 的项目。

该项目本身实现了的简单功能：

* 对一个文本文件（目前只支持 UTF-8 编码的文本文件）进行格式化：
  * 去掉每个**文本段落**前后的**空白**（whitespace），和其他一些不可见字符（具体请查看代码）
  * 每个**文本段落**间保持**一个**空行

简单的使用方法：

```bash
# infile 是需要被格式化的文本文件
# outfile 是格式化后的结果文本文件
rfit -i <infile> -o <outfile>
```

## 2. 编码原则

* 尽量遵守 Rust 编码的 Idiomatic
* 本项目较简单，只包含了单元测试；而没有包含集成测试
* 利用 log 配合 env_logger 来输出日志
  * 设置日志级别：export RUST_LOG=info
* 利用 snafu 来创建 Error 类型，并遵循 snafu 提倡的 Error Handling philosophy
* 目前使用的 IDE 是：Visual Studio Code
