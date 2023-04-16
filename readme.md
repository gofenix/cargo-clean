# cargo-clean

递归清除给定目录下的所有 rust 项目缓存，瞬间释放你的磁盘。

特别声明：

- 它是安全的，不会 rm -rf

# 背景

- 诶，我电脑磁盘怎么不够用了？
- 噢，这个项目怎么用了我 64 个 G？
- 好家伙，rust 项目太恐怖了！
- 我去，一个一个删太慢了！
- 要不写个小工具，自动运行 cargo clean！

说干就干，其实关键代码就是扫描给定目录下面的所有 rust 项目，然后执行 cargo clean 即可。

# 安装

## 从 cargo 安装

```
cargo install cargo-clean
```

## 直接从 git 安装

```
cargo install --locked --force --git https://github.com/zhenfeng-zhu/cargo-clean.git
```

# 用法

```
## 移除当前目录下的所有rust构建缓存，一键释放你的磁盘空间。
cargo-clean .
```
