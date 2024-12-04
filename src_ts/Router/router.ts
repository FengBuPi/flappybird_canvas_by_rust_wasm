import Router, { RouterOption } from '../utils/Router.js';
import { initGameStartModule } from '../components/gameStart.js';
import { initGameContainerModule } from '../components/gameContainer.js';
import { initGameOverModule } from '../components/gameOver.js';
import { initGamePauseModule } from '../components/gamePause.js';
console.log("router模块初始化成功")

const routers: RouterOption[] = [
  {
    name: "gameStart",
    url: "/",
    // template: document.getElementById("game-start") as HTMLTemplateElement,
    component: initGameStartModule
  },
  {
    name: "gameContainer",
    url: "/gameContainer",
    // template: document.getElementById("canvas") as HTMLTemplateElement,
    component: initGameContainerModule
  },
  {
    name: "gameOver",
    url: "/gameOver",
    // template: document.getElementById("game-over") as HTMLTemplateElement,
    component: initGameOverModule
  },
  {
    name: "buttonContainer",
    url: "/buttonContainer",
    // template: document.getElementById("button-container") as HTMLTemplateElement,
    component: initGamePauseModule
  }
]
const router = new Router(routers, document.getElementById("game-container")!)
function initRouter() {
}
export default router;
export { initRouter }