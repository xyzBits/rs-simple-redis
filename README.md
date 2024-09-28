# 一个简单的 redis server 实现。

## From trait 
- 实现了 From trait 的类型，会自动实现 Into trait 

## TryFrom trait 
- 定义了一个 try_from 方法，用于将一个类型转换为另一个类型
- 与 From 不同，转换过程可能会失败，返回一个 Result 类型，包含转换结果或错误信息
- 当转换可能失败时，使用 TryFrom 可以提供更好的错误处理

## 命令解析的过程
- set hello world 为例
- 这个命令前面带有 *，将其识别为 RespFrameArray
- 解析成一个 RespFrameBulkString的数组
- 再拿出 数组中的每一个 BulkString，判断其是 Set命令
- 将其解析成 Set 指令
