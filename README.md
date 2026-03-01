# Tauri + React + Typescript

This template should help get you started developing with Tauri, React and Typescript in Vite.

## Recommended IDE Setup

- [VS Code](https://code.visualstudio.com/) + [Tauri](https://marketplace.visualstudio.com/items?itemName=tauri-apps.tauri-vscode) + [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer)

```text
src-tauri/
└── src/
    ├── main.rs
    ├── commands.rs
    ├── state.rs
    │
    ├── core/
    │   ├── mod.rs
    │   │
    │   ├── manager.rs          👈 统一语言管理器
    │   ├── dto.rs              👈 返回给前端的数据结构
    │   │
    │   ├── language/
    │   │   ├── mod.rs          👈 trait 定义
    │   │   ├── python.rs
    │   │   ├── go.rs
    │   │   ├── node.rs
    │   │   └── rust.rs
    │   │
    │   ├── installer/
    │   │   ├── downloader.rs   👈 下载器（带进度）
    │   │   ├── extract.rs      👈 解压
    │   │   └── fs.rs           👈 文件操作
    │   │
    │   └── utils/
    │       ├── semver.rs
    │       └── path.rs
```

### dev & build
```shell
bun run dev
cargo build -p shim
cargo tauri dev
```
