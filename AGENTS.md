# LVM (Language Version Manager) - 项目上下文文档

## 项目概述

LVM 是一个基于 Tauri + React + TypeScript 的跨平台语言版本管理器应用程序。该项目旨在为开发者提供一个统一的管理界面,用于安装、切换和管理不同编程语言的版本(目前主要支持 Python,架构设计已预留支持 Go、Node.js、Rust 等语言的扩展能力)。

### 核心功能

- **版本管理**: 查看、安装、卸载、切换不同版本的编程语言
- **搜索过滤**: 支持按关键词搜索和过滤可用版本
- **下载管理**: 实时显示下载进度,支持批量下载任务管理
- **配置管理**: 支持自定义基础路径和下载路径设置
- **进度追踪**: 实时下载进度推送,优化前端渲染性能(1% 间隔更新)

### 核心技术栈

**前端:**
- **React 19.1.0** - UI 框架
- **TypeScript** - 类型安全的 JavaScript 超集
- **Vite 7.0.4** - 现代化的前端构建工具
- **Ant Design 6.3.0** - 企业级 UI 组件库
- **Redux Toolkit 2.11.2** - 状态管理
- **React Router DOM 7.13.0** - 路由管理
- **i18next 25.8.13** - 国际化支持(支持中英文)
- **Tauri API 2** - 前后端通信桥接

**后端:**
- **Tauri 2** - 跨平台桌面应用框架
- **Rust** - 后端核心语言
- **Tokio 1.49.0** - 异步运行时
- **Reqwest 0.13.2** - HTTP 客户端(用于下载语言版本)
- **Serde 1** - 序列化/反序列化
- **Tauri Plugin Store 2.4.2** - 持久化存储

### 项目架构

#### 前端架构 (`src/`)
```
src/
├── app/                    # 应用核心
│   ├── App.tsx            # 应用入口组件
│   ├── main.tsx           # React 渲染入口
│   └── routes/            # 路由配置
│       └── index.tsx      # 路由定义(/, /python, /settings, /downloader)
├── api/                   # API 调用层
│   └── tauri.ts           # Tauri 命令调用封装(safeInvoke)
├── core/                  # 核心配置和类型
│   ├── config/            # 配置文件
│   ├── constants/         # 常量定义(如语言枚举 LangEnum)
│   └── types/             # TypeScript 类型定义
│       └── common.ts      # ISearchPayload 等通用类型
├── features/              # 功能模块
│   ├── i18n/              # 国际化
│   │   ├── index.ts       # i18n 配置
│   │   └── locales/       # 语言包(en.json, zh.json)
│   ├── theme/             # 主题管理
│   │   ├── ThemeProvider.tsx
│   │   └── themeSlice.ts  # Redux 切片
│   └── version-manager/   # 版本管理功能
│       └── pages/         # 页面组件
│           ├── PythonManagePage/  # Python 版本管理主页
│           ├── Settings/          # 全局设置页面
│           └── DownloadCenter/    # 下载管理中心
├── hooks/                 # 自定义 React Hooks
│   └── useDownload.ts     # 下载任务管理 Hook
├── layouts/               # 布局组件
│   └── BasicLayout/       # 基础布局
│       ├── index.tsx      # 布局组件
│       └── Sider.tsx      # 侧边栏导航
├── pages/                 # 页面
│   └── error.tsx          # 错误页面
├── shared/                # 共享资源
│   ├── components/        # 共享组件
│   │   ├── IconFont.ts    # 图标字体
│   │   └── VersionTable.tsx  # 版本表格组件(带搜索)
│   └── utils/             # 工具函数
│       ├── store.ts       # Redux Store 配置
│       └── tauriStore.ts  # Tauri Store 工具
└── store/                 # Redux Store 配置
    └── index.ts           # Store 定义
```

#### 后端架构 (`src-tauri/src/`)
```
src-tauri/src/
├── main.rs                # 应用入口,调用 lib.rs::run()
├── lib.rs                 # 库入口,注册 Tauri 命令和插件
├── commands.rs            # Tauri 命令层(前后端桥接)
│   ├── list_versions      # 获取版本列表(支持搜索)
│   └── install            # 安装版本(带进度推送)
└── core/                  # 核心业务逻辑
    ├── manager.rs         # 语言管理器(统一接口)
    ├── dto.rs             # 数据传输对象(VersionInfo, PageResult)
    ├── language/          # 语言模块
    │   ├── mod.rs         # Trait 定义(LanguageInstaller)
    │   └── python.rs      # Python 实现
    ├── installers/        # 安装工具
    │   ├── mod.rs         # 安装器模块
    │   └── downloader.rs  # 下载器(带进度推送,1% 间隔优化)
    └── utils/             # 工具函数
        ├── mod.rs         # 工具模块
        ├── config.rs      # 配置管理(base_path, download_path)
        └── semver.rs      # 语义化版本处理
```

### 设计模式

- **Strategy Pattern**: 通过 `LanguageInstaller` trait 实现不同语言管理策略的可插拔设计
- **Layered Architecture**: 前端采用分层架构(Component -> Feature -> Core),后端采用三层架构(Commands -> Manager -> Installer)
- **Repository Pattern**: 隔离数据访问逻辑,便于测试和维护

---

## 构建和运行

### 前置要求

- **Node.js**: 推荐使用 Bun(项目使用 bun.lock),也可以使用 npm/pnpm
- **Rust**: 1.70 或更高版本
- **系统依赖**:
  - Windows: Microsoft Visual C++ Redistributable
  - macOS: Xcode Command Line Tools
  - Linux: libwebkit2gtk-4.0-dev, build-essential, curl, wget, file, libssl-dev

### 开发命令

```bash
# 安装依赖
bun install

# 启动开发服务器(同时启动前端和 Tauri 后端)
bun run tauri dev

# 仅启动前端开发服务器
bun run dev

# 构建生产版本
bun run build

# 预览生产构建
bun run preview

# 代码检查
bun run lint

# 自动修复代码问题
bun run lint:fix

# 代码格式化
bun run format
```

### Tauri 特定命令

```bash
# 开发模式(会自动运行 bun run dev)
bun run tauri dev

# 构建桌面应用(会自动运行 bun run build)
bun run tauri build

# 查看所有 Tauri 命令
bun run tauri --help
```

### 开发服务器配置

- **前端端口**: 1420
- **HMR 端口**: 1421
- **热模块替换**: 自动忽略 `src-tauri` 目录的变更
- **路径别名**: `@/` 指向 `src/` 目录

---

## 开发规范

### 代码风格

#### TypeScript/JavaScript
- **格式化工具**: Prettier
- **配置文件**: `.prettierrc`
- **关键规则**:
  - 打印宽度: 100 字符
  - 缩进: 2 空格
  - 引号: 单引号(JSX 属性使用双引号)
  - 分号: 必须使用
  - 尾随逗号: ES5 风格(始终添加)
  - 箭头函数参数: 单参数时省略括号
  - 行尾: CRLF (Windows)

#### Linting
- **工具**: ESLint 9
- **配置文件**: `eslint.config.js`
- **关键规则**:
  - 必须通过 TypeScript 类型检查
  - 禁止未使用的导入(`unused-imports/no-unused-imports: error`)
  - 禁止未使用的变量(警告级别,以下划线开头的变量除外)
  - 强制导入顺序: builtin → external → internal → parent → sibling → index
  - React JSX 布尔值: 省略值(如 `<Button disabled />` 而非 `<Button disabled={true} />`)
  - 自闭合标签: 尽可能使用自闭合(如 `<Component />`)

#### Rust
- **格式化工具**: cargo fmt(默认配置)
- **Linter**: clippy
- **关键约定**:
  - 使用 `async/await` 进行异步操作
  - 错误处理使用 `Result<T, String>` 类型
  - 使用 `#[allow(dead_code)]` 标记未使用的代码(保留以备未来使用)
  - 模块组织遵循功能而非文件类型

### 文件命名约定

- **组件**: PascalCase (如 `VersionTable.tsx`, `ThemeProvider.tsx`)
- **工具函数**: camelCase (如 `store.ts`, `tauriStore.ts`)
- **常量/枚举**: PascalCase (如 `LangEnum`)
- **Rust 模块**: snake_case (如 `manager.rs`, `dto.rs`)
- **Rust 结构体/枚举**: PascalCase (如 `LanguageManager`, `VersionInfo`)

### 导入顺序

```typescript
// 1. Node.js 内置模块
import { useState, useEffect } from 'react';

// 2. 外部依赖(第三方库)
import { Button, Table } from 'antd';
import { invoke } from '@tauri-apps/api/core';

// 3. 内部模块(使用 @/ 别名)
import { LangEnum } from '@/core/constants/enum';
import { store } from '@/store';

// 4. 相对导入
import { SomeComponent } from './components';
import { someUtil } from '../utils';
```

### 类型定义

- **前端 TypeScript**:
  - 所有组件必须使用接口或类型定义 Props
  - 使用 `interface` 定义对象结构
  - 使用 `type` 定义联合类型、交叉类型和工具类型
  - 启用严格模式(`strict: true`)

- **后端 Rust**:
  - 使用 `struct` 定义数据结构
  - 使用 `#[derive(Serialize, Deserialize)]` 自动实现序列化
  - 使用 `#[tauri::command]` 标记 Tauri 命令函数

### 状态管理

- **全局状态**: 使用 Redux Toolkit
  - 创建 slice: `features/*//*Slice.ts`
  - 注册到 store: `store/index.ts`
  - 使用类型安全的 hooks: `useAppDispatch`, `useAppSelector`

- **本地状态**: 使用 React Hooks (`useState`, `useReducer`)

- **持久化存储**: 使用 Tauri Store Plugin
  - 工具函数: `src/shared/utils/tauriStore.ts`

### 国际化

- **框架**: i18next + react-i18next
- **语言文件**: `src/features/i18n/locales/`
- **当前支持**: 英文(`en.json`), 中文(`zh.json`)
- **使用方式**:
  ```typescript
  import { useTranslation } from 'react-i18next';

  function MyComponent() {
    const { t } = useTranslation();
    return <h1>{t('welcome')}</h1>;
  }
  ```

### 测试

**当前状态**: 项目尚未配置测试框架。

**建议**:
- 前端单元测试: Vitest + React Testing Library
- 端到端测试: Playwright
- 后端单元测试: Rust 内置测试框架

### Git 提交规范

**当前提交历史示例**:
```
a9e5186 feat: install command & downloader
4d5b17d feat: search result filter
9e85bb7 feat: common types
acbdfe5 del: unused codes
707afc9 feat: keyword
```

**建议遵循 Conventional Commits**:
- `feat:` 新功能
- `fix:` 修复 bug
- `refactor:` 重构(不改变功能)
- `perf:` 性能优化
- `style:` 代码格式调整
- `docs:` 文档更新
- `test:` 测试相关
- `chore:` 构建/工具配置

---

## 核心功能实现

### 版本列表查询(支持搜索)

**前端调用**:
```typescript
import { safeInvoke } from '@/api/tauri';

const result = await safeInvoke<VersionResult>('list_versions', {
  language: 'python',
  page: 0,
  pageSize: 10,
  keyWord: '3.9'  // 可选搜索关键词
});
```

**后端实现**:
1. `commands.rs::list_versions` - 接收前端请求,包含搜索关键词参数
2. `manager.rs::LanguageManager::list_versions` - 根据语言类型创建对应安装器
3. `language/python.rs::PythonInstaller::list_versions` - 获取所有可用版本并过滤
4. 返回 `PageResult` 结构(包含总数和分页列表)

### 安装版本(带进度推送)

**前端调用**:
```typescript
import { safeInvoke } from '@/api/tauri';

await safeInvoke('install', {
  language: 'python',
  version: '3.9.7'
});
```

**进度监听**:
```typescript
import { listen } from '@tauri-apps/api/event';

const unlisten = listen<{
  version: string;
  current: u64;
  total: u64;
  percentage: f64;
}>('download-progress', (event) => {
  const { version, percentage } = event.payload;
  // 更新 UI 显示下载进度
});
```

**后端实现**:
1. `commands.rs::install` - 接收安装请求,注入 AppHandle 和 Window
2. `config.rs::get_download_path` - 获取下载路径(从配置或默认值)
3. `installers/downloader.rs::download_with_progress` - 下载文件并推送进度
4. 进度优化: 每 1% 变化才发送事件,减轻前端渲染压力

### 配置管理

**前端设置页面** (`src/features/version-manager/pages/Settings/index.tsx`):
```typescript
import { LazyStore } from '@tauri-apps/plugin-store';

const store = new LazyStore('.settings.json');

// 读取配置
const basePath = await store.get<string>('base_path');

// 保存配置
await store.set('base_path', 'd:\\lvm');
await store.save(); // 持久化到硬盘
```

**后端配置读取** (`src-tauri/src/core/utils/config.rs`):
```rust
pub fn get_download_path(app: &AppHandle) -> PathBuf {
    let base = get_base_path(app);
    let download_dir = base.join("download");

    // 自动创建下载目录
    if !download_dir.exists() {
        let _ = std::fs::create_dir_all(&download_dir);
    }

    download_dir
}
```

### 下载任务管理 Hook

**自定义 Hook** (`src/hooks/useDownload.ts`):
```typescript
import { useDownload } from '@/hooks/useDownload';

function DownloadCenter() {
  const { tasks, startDownload } = useDownload();

  // tasks: 下载任务列表
  // startDownload: 开始下载函数
}
```

**功能特性**:
- 自动监听 `download-progress` 事件
- 管理多个下载任务状态
- 支持任务状态追踪(downloading/success/error)
- 自动清理已完成任务

### 数据流

```
用户操作 → React 组件 → Custom Hook → Tauri Command → Rust Manager → Installer → 事件推送 → 更新 UI
```

### 路由配置

**当前路由** (`src/app/routes/index.tsx`):
- `/` - 重定向到 `/python`
- `/python` - Python 版本管理页面
- `/settings` - 全局设置页面
- `/downloader` - 下载管理中心
- 错误页面 - 自动错误边界处理

### 扩展新语言支持

要添加对新语言的支持(例如 Go),需要:

1. **后端**:
   - 在 `src-tauri/src/core/language/` 创建 `go.rs`
   - 实现 `LanguageInstaller` trait:
     ```rust
     pub struct GoInstaller;

     impl LanguageInstaller for GoInstaller {
         async fn list_versions(&self) -> Result<Vec<String>, String> {
             // 获取 Go 版本列表
         }
         async fn list_installed(&self) -> Result<Vec<String>, String> {
             // 获取已安装版本
         }
         async fn current(&self) -> Result<Option<String>, String> {
             // 获取当前版本
         }
     }
     ```
   - 在 `manager.rs` 的 `LanguageManager::new()` 中添加分支

2. **前端**:
   - 在 `src/core/constants/enum.ts` 添加语言枚举
   - 在语言包中添加翻译(`en.json`, `zh.json`)
   - 创建对应的管理页面(如 `GoManagePage`)
   - 更新路由配置和侧边栏导航

3. **API 封装**:
   - 复用 `safeInvoke` 函数调用 Tauri 命令
   - 复用 `VersionTable` 组件展示版本列表

---

## 常见问题

### Q: 开发服务器启动失败,提示端口 1420 被占用?
A: 检查是否有其他进程占用该端口,或修改 `vite.config.ts` 中的端口配置。

### Q: Tauri 命令调用失败?
A: 确保:
1. 后端命令已在 `lib.rs` 中注册
2. 前端调用时参数类型匹配
3. Rust 代码已重新编译

### Q: 国际化切换不生效?
A: 检查:
1. 语言文件是否存在于 `src/features/i18n/locales/`
2. 是否在组件中正确使用 `useTranslation` hook
3. 是否在 `main.tsx` 中初始化了 i18n

### Q: TypeScript 类型检查报错?
A: 运行 `bun run lint:fix` 尝试自动修复,或检查类型定义是否完整。

### Q: 下载进度不更新?
A: 检查:
1. 前端是否正确监听 `download-progress` 事件
2. 后端是否正确使用 `window.emit` 发送事件
3. 进度推送是否达到 1% 的变化阈值

### Q: 配置保存后不生效?
A: 检查:
1. 是否调用了 `store.save()` 持久化配置
2. 后端是否正确从 store 读取配置
3. 下载路径是否有写入权限

### Q: 搜索功能不工作?
A: 检查:
1. 前端是否正确传递 `keyWord` 参数
2. 后端是否实现了搜索过滤逻辑
3. 搜索关键词格式是否正确

---

## 项目特定配置

### TypeScript 配置 (`tsconfig.json`)
- 目标: ES2020
- 模块系统: ESNext
- 严格模式: 启用
- 路径别名: `@/` → `src/`
- JSX: react-jsx

### Vite 配置 (`vite.config.ts`)
- 插件: React
- 开发服务器: 端口 1420,严格端口模式
- HMR: WebSocket 连接到 1421
- 忽略监视: `src-tauri` 目录

### Tauri 配置 (`tauri.conf.json`)
- 产品名称: lvm
- 标识符: com.gavinhaydy.lvm
- 窗口大小: 800x600
- 用户代理: lvm
- 开发命令前: `bun run dev`
- 构建命令前: `bun run build`

### Tauri Store Plugin 配置
- **用途**: 持久化存储用户配置(如 base_path, download_path)
- **配置文件**: `.settings.json` (前端) / `.settings.dat` (后端)
- **初始化**: 在 `lib.rs` 中通过 `tauri_plugin_store::Builder::new().build()` 注册
- **使用方式**:
  - 前端: `LazyStore` 类
  - 后端: `app.get_store()` 方法

---

## 关键组件说明

### VersionTable 组件 (`src/shared/components/VersionTable.tsx`)
**功能**: 统一的版本表格展示组件
**特性**:
- 内置搜索框,支持关键词过滤
- 展示版本、安装状态、使用状态
- 支持 Install/Uninstall 和 Use 操作
- 内置分页功能

**Props 接口**:
```typescript
interface VersionTableProps {
  data: VersionResult;        // 版本数据
  loading?: boolean;          // 加载状态
  onSearch?: (value: string) => void;  // 搜索回调
  onInstallToggle?: (record: VersionItem) => void;  // 安装/卸载回调
  onUseToggle?: (record: VersionItem) => void;      // 切换版本回调
}
```

### useDownload Hook (`src/hooks/useDownload.ts`)
**功能**: 管理下载任务和进度
**特性**:
- 自动监听下载进度事件
- 管理多个并发下载任务
- 提供任务状态追踪

**返回值**:
```typescript
{
  tasks: DownloadTask[];      // 下载任务列表
  startDownload: (language: string, version: string) => void;  // 开始下载
}
```

### safeInvoke 函数 (`src/api/tauri.ts`)
**功能**: 封装 Tauri invoke 调用,提供统一的错误处理
**用途**: 简化 Tauri 命令调用,统一处理类型转换和错误

---

## 贡献指南

1. 确保所有代码通过 linting: `bun run lint`
2. 格式化代码: `bun run format`
3. 遵循既定的代码风格和架构模式
4. 提交前进行自测
5. 编写清晰的提交信息
6. 新增功能需要更新相关文档和类型定义

---

## 相关资源

- [Tauri 官方文档](https://tauri.app/)
- [Tauri Store Plugin](https://github.com/tauri-apps/plugins-workspace/tree/v2/plugins/store)
- [React 官方文档](https://react.dev/)
- [Ant Design 文档](https://ant.design/)
- [Redux Toolkit 文档](https://redux-toolkit.js.org/)
- [i18next 文档](https://www.i18next.com/)
- [Rust 官方文档](https://www.rust-lang.org/)
- [Vite 官方文档](https://vitejs.dev/)