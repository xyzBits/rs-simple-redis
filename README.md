# 一个简单的 redis server 实现。

## From trait 
- 实现了 From trait 的类型，会自动实现 Into trait 

## TryFrom trait 
- 定义了一个 try_from 方法，用于将一个类型转换为另一个类型
- 与 From 不同，转换过程可能会失败，返回一个 Result 类型，包含转换结果或错误信息
- 当转换可能失败时，使用 TryFrom 可以提供更好的错误处理
- 
