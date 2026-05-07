# Agent 协作指南

当前仓库的代码代理协作说明。

## 概览

- 技术栈：Vue 3 + TypeScript + Vite + Vue Router
- 桌面端：Tauri 2 + Rust
- UI：Naive UI
- 样式：UnoCSS
- 平台：Windows 桌面，默认保持透明窗口与 Mica 视觉基线

## 必须遵守

- 保证 `pnpm run windows:dev` 可继续正常工作。
- 非明确需求下，不改变现有路由和布局结构。
- 优先做小而聚焦的改动，避免无关重构。
- 保持文档与项目真实状态一致。

## 路由与布局

- 路由统一定义在 `src/router/index.ts`。
- 顶部 Tabs 由 `src/components/Tabs.vue` 根据路由元信息生成。
- 如果页面需要显示在顶部 Tabs，必须同时设置：
  - `meta.isTab: true`
  - `meta.tabsName: '...'`
- 路由 `name` 要保持稳定，Tabs 会用它作为菜单 key。
- 主页面放在 `DefaultLayout` 下。
- 设置页放在 `SettingsLayout` 下，除非任务明确要求调整。

## UI 与样式

- 先遵循现有模式，再考虑新增抽象。
- 常规样式优先使用 UnoCSS。
- 组件局部过渡、深层覆盖或不适合工具类的样式再用 scoped CSS。
- 保证前景内容在透明 / Mica 背景上清晰可读。
- 非明确需求下，不要把透明窗口基线改成纯不透明整窗背景。

## 工具链与校验

- 包管理和脚本统一使用 `pnpm`。
- 非必要不要改动 Vite 与 Tauri 的 dev URL 对齐关系。
- 除非确有需要，不要破坏 `vite.config.ts` 中现有的自动导入和自动注册配置。
- Rust 侧改动保持在 `src-tauri/` 内。

按改动范围执行对应命令：

```bash
pnpm run build
pnpm run windows:dev
```

如果环境阻止命令执行，需要明确说明，并至少对改动文件做人工校验。

## 文档

- 当行为或约定变化时，同步更新中英文文档。
- 示例要与当前路由名、脚本名和目录结构保持一致。
- 文档保持简洁、可执行、准确。
