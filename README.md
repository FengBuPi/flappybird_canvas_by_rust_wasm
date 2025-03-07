# Flappy Bird (Rust + WebAssembly + Ts)

## 项目特点

1. 使用 Rust 编写核心游戏逻辑，通过 WebAssembly 在浏览器中运行
2. TypeScript 处理 UI 和用户交互
3. 采用 Canvas 进行游戏渲染
4. 模块化设计，代码结构清晰

## 技术栈

- Rust
- WebAssembly
- TypeScript
- Canvas API
- Web APIs

## 项目结构

```
flappybird_canvas_by_rust_wasm/
├── src/                # Rust 源代码
│   ├── bird.rs        # 小鸟模块
│   ├── pipe.rs        # 管道模块
│   ├── sky.rs         # 天空背景
│   ├── land.rs        # 地面模块
│   ├── game.rs        # 游戏主逻辑
│   └── lib.rs         # 库入口
├── src_ts/            # TypeScript 源代码
│   ├── Router/        # 路由管理
│   ├── State/         # 状态管理
│   ├── components/    # 游戏组件
│   └── main.ts        # 主入口
└── asset/             # 静态资源
└── images/        # 游戏图片资源
```

## 编开发模式（监听文件变化自动编译）

监听文件变化触发编译

```bash
cargo watch -s 'wasm-pack build --dev --target web && tsc'
```

然后使用 live-server 启动服务

## Git 提交规范

提交信息格式： <type>: <description>

```bash
feat: 新功能
fix: 修复bug
refactor: 重构
perf: 性能优化
docs: 文档
test: 测试
style: 代码格式修改,不涉及业务逻辑
ci: 持续集成
chore: 其他
revert: 回滚
```

## 游戏功能

- 小鸟飞行物理效果
- 管道随机生成
- 碰撞检测
- 游戏暂停/继续
- 游戏结束判定
- 重新开始功能

## 许可证

MIT
