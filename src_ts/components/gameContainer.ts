import init, { Game } from "../../pkg/flappybird_canvas_by_rust_wasm.js";
import runing from "../State/gameState.js";
import { Component } from "./../utils/Router";
import { initGamePauseModule } from "../components/gamePause.js";

const sleep = (ms: number) => new Promise((resolve) => setTimeout(resolve, ms));

let raf: number = 0;
async function startGame() {
  await init();
  const game = new Game("canvas");
  game.game_init();
  game.frame();

  function animation() {
    // 如果游戏暂停，只继续动画循环但不更新游戏状态
    if (!runing.value) {
      raf = requestAnimationFrame(animation);
      return;
    }
    // 更新游戏状态,但如果游戏帧返回 true（表示游戏结束）
    if (game.frame()) {
      runing.value = false;
      cancelAnimationFrame(raf);
      return;
    }
    // 继续下一帧动画
    raf = requestAnimationFrame(animation);
  }

  animation();
}

function mounted() {
  initGamePauseModule.mounted();
  startGame();
}

function unmounted() {
  initGamePauseModule.unmounted();
  cancelAnimationFrame(raf);
}
const initGameContainerModule: Component = {
  template: document.getElementById("game-container")!,
  mounted,
  unmounted,
};

export { initGameContainerModule };
