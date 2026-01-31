# OAuth前端管理系统

这是一个基于Vue 3和TypeScript的后台管理系统前端项目，为OAuth后端服务提供用户界面。

## 功能特性

- 用户登录/注册页面
- 后台管理仪表板
- 响应式设计，适配不同屏幕尺寸
- 与后端API集成的身份验证

## 技术栈

- Vue 3 (Composition API)
- TypeScript
- Vue Router 4
- Pinia (状态管理)
- Axios (HTTP请求)
- Vite (构建工具)

## 项目结构

```
src/
├── components/      # 公共组件
├── views/          # 页面视图
│   ├── Login.vue   # 登录页面
│   ├── Register.vue # 注册页面
│   └── Dashboard.vue # 仪表板页面
├── router/         # 路由配置
├── utils/          # 工具函数
├── config/         # 配置文件
└── assets/         # 静态资源
```

## 安装与运行

1. 安装依赖：
```bash
npm install
```

2. 启动开发服务器：
```bash
npm run dev
```

3. 构建生产版本：
```bash
npm run build
```

## API代理配置

开发环境中，所有 `/api` 请求都会被代理到 `http://localhost:3000`，即后端服务地址。

## 环境要求

- Node.js >= 18
- npm >= 8

## 部署

构建后的文件位于 `dist/` 目录，可以直接部署到任何静态文件服务器。