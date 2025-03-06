import { initRouter } from "./Router/router.js";

// 主函数
async function main(): Promise<void> {
  initRouter();
}

window.onload = () => {
  main();
};
