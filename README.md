# Rust Database Learning

这是我的 Rust 学习仓库，用于记录学习笔记、保存独立小练习，并通过阶段项目逐步学习 Rust 系统编程与数据库开发。

每个小练习都是一个可以独立编译和运行的 Cargo 二进制项目，包含自己的 `Cargo.toml` 和 `src/main.rs`；所有练习和项目由根目录的 Cargo Workspace 统一管理。

## 学习目标

- 掌握 Rust 基础语法、所有权、借用和生命周期
- 熟悉错误处理、泛型、Trait、迭代器和智能指针
- 掌握文件 I/O、并发、异步网络和系统编程基础
- 实现内存键值存储与持久化键值存储
- 学习索引、事务、WAL、查询执行和崩溃恢复
- 最终实现一个教学级单机关系型数据库

## 仓库结构

```text
.
├── Cargo.toml                 # Cargo Workspace 配置
├── Cargo.lock                 # 工作区依赖锁定文件
├── README.md                  # 仓库说明与总体进度
├── .gitignore
│
├── notes/                     # 按知识点整理的学习笔记
│   ├── 01-basics.md
│   ├── 02-ownership.md
│   ├── 03-struct-enum.md
│   ├── 04-error-handling.md
│   └── 05-traits-lifetimes.md
│
├── exercises/                 # 独立小练习
│   ├── 01-basics/
│   │   ├── ex001-hello-world/
│   │   │   ├── Cargo.toml
│   │   │   ├── README.md
│   │   │   └── src/main.rs
│   │   ├── ex002-variables/
│   │   │   ├── Cargo.toml
│   │   │   └── src/main.rs
│   │   └── ex003-control-flow/
│   │       ├── Cargo.toml
│   │       └── src/main.rs
│   ├── 02-ownership/
│   │   ├── ex004-move/
│   │   ├── ex005-borrowing/
│   │   └── ex006-slices/
│   └── 03-struct-enum/
│       ├── ex007-struct/
│       ├── ex008-enum/
│       └── ex009-pattern-matching/
│
├── projects/                  # 阶段性项目
│   ├── guessing-game/
│   ├── minigrep/
│   ├── in-memory-kv/
│   └── persistent-kv/
│
├── journal/                   # 按月份记录学习过程
│   ├── 2026-08.md
│   └── 2026-09.md
│
└── resources/
    └── links.md               # 教材、文章和项目链接
```

目录会随学习进度逐步创建，不要求一开始全部准备完成。

## Cargo Workspace

根目录的 `Cargo.toml` 负责统一管理所有练习和项目：

```toml
[workspace]
resolver = "3"
members = [
    "exercises/*/*",
    "projects/*",
]
```

每个练习的 `Cargo.toml` 保持独立，例如：

```toml
[package]
name = "ex004-move"
version = "0.1.0"
edition = "2024"
publish = false

[dependencies]
```

## 练习命名规则

练习目录统一采用 `ex + 三位编号 + 英文主题`：

```text
ex001-hello-world
ex002-variables
ex003-control-flow
ex004-move
ex005-borrowing
ex006-slices
ex007-struct
ex008-enum
ex009-pattern-matching
ex010-error-handling
```

编号用于保留学习顺序；英文主题方便运行 Cargo 命令和跨平台使用。

## 创建和运行练习

在仓库根目录创建一个新练习：

```powershell
cargo new --bin --vcs none exercises/01-basics/ex002-variables
```

创建后，在练习的 `Cargo.toml` 中加入：

```toml
publish = false
```

运行指定练习：

```powershell
cargo run -p ex001-hello-world
```

检查指定练习：

```powershell
cargo check -p ex001-hello-world
cargo test -p ex001-hello-world
cargo clippy -p ex001-hello-world
```

检查整个工作区：

```powershell
cargo check --workspace
cargo test --workspace
cargo fmt --all
cargo clippy --workspace --all-targets
```

## 单个练习的记录方式

简单练习只需要 `Cargo.toml` 和 `src/main.rs`。重要练习可以增加一个 `README.md`，记录：

- 练习目标
- 需要解决的问题
- 自己的思考过程
- 遇到的编译错误
- 最终理解
- 对应的知识笔记

示例：

```markdown
# ex004-move

## 练习目标

理解 String 在赋值时发生的所有权移动。

## 遇到的问题

编译器提示 `borrow of moved value`。

## 我的理解

String 的堆数据不会在普通赋值时被复制，所有权会移动到新变量。
移动之后，原变量不再有效。
```

## 练习与项目的区别

放入 `exercises/`：

- 一次只验证一个知识点
- 代码量通常较少
- 每个练习可以独立运行
- 一般在一天内完成

放入 `projects/`：

- 包含多个模块或多种知识点
- 有独立的测试和外部依赖
- 需要数天或数周完成
- 例如 Minigrep、Todo、KV Store 和数据库存储引擎

## 学习进度

| 阶段 | 学习内容 | 状态 |
|---|---|---|
| 1 | 环境配置、Cargo、变量、类型和控制流 | 未开始 |
| 2 | 所有权、借用、切片和字符串 | 未开始 |
| 3 | Struct、Enum、模式匹配和错误处理 | 未开始 |
| 4 | 泛型、Trait、生命周期和迭代器 | 未开始 |
| 5 | 智能指针、并发和文件 I/O | 未开始 |
| 6 | Minigrep 与命令行项目 | 未开始 |
| 7 | 内存键值存储 | 未开始 |
| 8 | 持久化键值存储 | 未开始 |
| 9 | 数据库索引、事务、WAL 和恢复 | 未开始 |
| 10 | 教学级关系型数据库 | 未开始 |

状态统一使用：`未开始`、`学习中`、`已完成`。

## 学习流程

每个知识点尽量按照以下步骤完成：

1. 阅读教材并记录核心概念
2. 手动输入并运行示例代码
3. 完成一个对应的小练习
4. 不看资料重新实现一次
5. 为重要逻辑补充测试
6. 记录编译错误和自己的理解
7. 提交 Git，并更新学习进度

## 主要学习资料

- [Rust 语言圣经](https://course.rs/)
- [Rust 语言圣经在线练习](https://practice.course.rs/)
- [The Rust Programming Language](https://doc.rust-lang.org/stable/book/)
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/)
- [Rustlings](https://github.com/rust-lang/rustlings)
- [Comprehensive Rust 中文版](https://google.github.io/comprehensive-rust/zh-CN/)

## Git 提交信息建议

```text
notes: add ownership notes
exercise: add ex004 move practice
exercise: finish borrowing practice
project: initialize in-memory kv store
test: add kv store unit tests
fix: correct file recovery logic
docs: update learning progress
```

建议一次提交只完成一个明确的小目标。

## 当前计划

- [ ] 创建根目录 Cargo Workspace
- [ ] 创建 `notes`、`exercises`、`projects`、`journal` 和 `resources` 目录
- [ ] 确认 Rust、Cargo、Clippy 和 Rustfmt 可用
- [ ] 创建 `ex001-hello-world`
- [ ] 学习变量、数据类型、函数和控制流
- [ ] 开始 Rustlings 基础练习
- [ ] 编写第一篇学习笔记

---

坚持小步练习和小步提交。先保证理解与正确性，再考虑抽象、并发和性能。
