# Vetra

<p align="center">
  <img src="public/logo.png" alt="Flux Logo" width="160" />
</p>

一个极简的 Windows 桌面应用模板，基于 Vue 3、Tauri 2、Naive UI 和 UnoCSS，默认采用透明窗口、深色界面与 Windows 11 Mica 风格。

## 截图

![截图 1](./docs/screenshots/1-0429.png)
![截图 2](./docs/screenshots/2-0429.png)
![截图 3](./docs/screenshots/3-0429.png)

## 技术栈

- Vue 3 + TypeScript + Vite
- Tauri 2 + Rust
- Naive UI
- UnoCSS

## 开发

```bash
# 安装依赖
pnpm i
# 启动
pnpm run windows:dev
# 打包
pnpm run windows:build
```

## 页面路由元数据

```vue
<route lang="json5">
{
  name: 'Welcome', // 路由名称
  meta: {
    layout: 'main', // 使用的布局
    isTab: true, // 是否显示在顶部标签栏
    tabsName: '欢迎', // 标签栏显示文字
    tabOrder: 1, // 标签栏排序
  },
}
</route>
```

## 致谢

[**Linux.Do 社区**](https://linux.do) (真诚 、友善 、团结 、专业)
