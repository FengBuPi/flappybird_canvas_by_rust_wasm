# 编译
监听文件变化触发编译
~~~bash
cargo watch -s 'wasm-pack build --dev --target web && tsc'
~~~
# git 提交规范
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