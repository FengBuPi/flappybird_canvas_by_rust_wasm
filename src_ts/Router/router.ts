import Router, { RouterOption } from '../utils/Router.js';
import { initGameStartModule } from '../components/gameStart.js';
import { initGameContainerModule } from '../components/gameContainer.js';
import { initGameOverModule } from '../components/gameOver.js';
console.log("router模块初始化成功")

const routers: RouterOption[] = [
  {
    name: "gameStart",
    url: "/",
    component: initGameStartModule
  },
  {
    name: "gameContainer",
    url: "/gameContainer",
    component: initGameContainerModule
  },
  {
    name: "gameOver",
    url: "/gameOver",
    component: initGameOverModule
  },
]

const router = new Router(routers, document.getElementById("app")!)
function initRouter() {
}
export default router;
export { initRouter }