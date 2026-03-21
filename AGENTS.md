# AGENTS.md

本文档为 Qoder (qoder.com) 提供本仓库的开发指南。

## 项目概述

LVM 是一个基于 Tauri + React + TypeScript 的跨平台语言版本管理器桌面应用，提供统一的界面来安装、切换和管理不同编程语言的版本（当前支持 Python 和 Go）。

## 常用命令

### 开发命令

```bash
# 安装依赖
bun install

# 启动开发服务器（同时启动前端和 Tauri 后端）
bun run tauri dev

# 仅启动前端开发服务器（Mock 模式，用于前端独立开发）
bun run dev:mock

# 构建生产版本
bun run build
```

### 代码质量

```bash
# 运行代码检查（提交前必须通过）
bun run lint

# 自动修复代码问题
bun run lint:fix

# 格式化代码
bun run format
```

### Rust 后端

```bash
# 格式化 Rust 代码
cargo fmt

# 运行 Clippy 检查
cargo clippy

# 构建 shim crate（运行 Tauri 前需要）
cargo build -p shim
```

## 架构概述

### 前后端通信

前端通过 `safeInvoke()` 函数（位于 `src/api/tauri.ts`）调用 Tauri 命令：

```typescript
import { safeInvoke } from '@/api/tauri';

const result = await safeInvoke<VersionResult>('list_versions', {
  language: 'python',
  page: 0,
  pageSize: 10,
  keyWord: '3.9'
});
```

后端命令实现在 `src-tauri/src/commands.rs`，通过 `LanguageManager`（位于 `src-tauri/src/core/manager.rs`）分派到具体的语言安装器。

### 语言扩展架构

新增语言需要实现 `LanguageInstaller` trait（定义在 `src-tauri/src/core/language/mod.rs`）：

```rust
#[async_trait]
pub trait LanguageInstaller {
    async fn list_versions(&self) -> Result<Vec<String>, String>;
    async fn list_installed(&self) -> Result<Vec<String>, String>;
    async fn current(&self) -> Result<Option<String>, String>;
    async fn install(&self, window: tauri::Window<Wry>, version: &str, save_path: &str) -> Result<(), String>;
    async fn activate(&self, version: &str) -> Result<(), String>;
    async fn deactivate(&self, version: &str) -> Result<(), String>;
    async fn uninstall(&self, version: &str) -> Result<(), String>;
    fn get_download_url(&self, version: &str) -> Result<String, String>;
}
```

已有实现位于 `src-tauri/src/core/language/python.rs` 和 `go.rs`。Manager 模式根据语言类型将命令路由到对应的安装器。

### 下载进度系统

下载过程中 Rust 向前端发送进度事件：
- `download-progress` - 下载进度（前端 200ms 节流）
- `download-complete` - 安装成功完成
- `download-error` - 安装失败

`useDownload` Hook（位于 `src/hooks/useDownload.ts`）管理任务状态。

### Mock 模式

设置 `VITE_API_MODE=mock` 可使用 Mock 数据代替 Tauri 后端。Mock 处理器定义在 `src/mock/handlers.ts`。用于不启动 Rust 后端时的前端开发。

## 代码规范

### TypeScript/React

- **导入顺序**：builtin → external → internal (@/) → parent → sibling → index（ESLint 强制）
- **未使用导入**：禁止（error 级别）
- **Prettier 配置**：100 字符宽度、2 空格缩进、单引号（JSX 双引号）、尾随逗号（ES5 风格）
- **翻译 key**：使用 snake_case，位于 `src/features/i18n/locales/`

### Rust

- 异步操作使用 `async/await`
- 错误处理：返回 `Result<T, String>`
- 故意未使用的代码标记 `#[allow(dead_code)]`

## 关键文件参考

| 用途 | 路径 |
|------|------|
| Tauri 命令 | `src-tauri/src/commands.rs` |
| 语言 Trait | `src-tauri/src/core/language/mod.rs` |
| 语言管理器 | `src-tauri/src/core/manager.rs` |
| API 客户端 | `src/api/tauri.ts` |
| 下载 Hook | `src/hooks/useDownload.ts` |
| 共享表格组件 | `src/shared/components/VersionTable.tsx` |
| 国际化配置 | `src/features/i18n/` |
| Mock 处理器 | `src/mock/handlers.ts` |

## 添加新语言步骤

1. **后端**：在 `src-tauri/src/core/language/{lang}.rs` 实现 `LanguageInstaller`
2. **后端**：在 `manager.rs` 的 `LanguageManager::new()` 中添加语言分支
3. **前端**：在 `src/core/constants/enum.ts` 的 `LangEnum` 中添加语言
4. **前端**：在 `en.json` 和 `zh.json` 中添加翻译
5. **前端**：创建页面组件并在 `src/app/routes/index.tsx` 添加路由
