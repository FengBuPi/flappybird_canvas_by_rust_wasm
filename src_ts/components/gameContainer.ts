import init, { Game } from "../../pkg/flappybird_canvas_by_rust_wasm.js";
import runing from "../State/gameState.js";
import { Component } from "./../utils/Router";
import { initGamePauseModule } from "../components/gamePause.js";

// 添加图片预加载函数
async function preloadImages() {
  const images = [
    "./asset/images/sky.png",
    "./asset/images/land.png",
    "./asset/images/pipe1.png",
    "./asset/images/pipe2.png",
    "./asset/images/birds.png",
  ];

  const loadImage = (src: string): Promise<void> => {
    return new Promise((resolve) => {
      const img = new Image();
      img.onload = () => resolve();
      img.src = src;
    });
  };

  await Promise.all(images.map(loadImage));
}

let raf: number = 0;
async function startGame() {
  await Promise.all([init(), preloadImages()]);
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
