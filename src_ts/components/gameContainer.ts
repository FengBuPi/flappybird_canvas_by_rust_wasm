import init, { Game } from '../../pkg/flappybird_canvas_by_rust_wasm.js';
import runing from '../State/gameState.js';
import { Component } from './../utils/Router';
import { initGamePauseModule } from '../components/gamePause.js';

let raf: number = 0;
async function startGame() {
  await init();
  const game = new Game('canvas');
  game.game_init();
  game.frame()
  function animation() {
    if (runing.value) {
      game.frame()
    }
    raf = requestAnimationFrame(animation);
  }
  animation()
}

function mounted() {
  initGamePauseModule.mounted()
  startGame()
}

function unmounted() {
  initGamePauseModule.unmounted()
  cancelAnimationFrame(raf)
}
const initGameContainerModule: Component = {
  template: document.getElementById('game-container')!,
  mounted,
  unmounted,
}

export { initGameContainerModule }