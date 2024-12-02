import init, { Game } from '../pkg/flappybird_canvas_by_rust_wasm.js';
import runing from './State/gameState.js';
import { addResumeandPauseEvent } from './components/gamePause.js';

// 图片预加载
// let imgs = [
//   "/asset/images/sky.png",
//   "/asset/images/land.png",
//   "/asset/images/pipe1.png",
//   "/asset/images/pipe2.png",
//   "/asset/images/birds.png",
// ];
// for (const img of imgs) {
//   let image = new Image();
//   image.src = img;
// }



// 主函数
async function main(): Promise<void> {
  await init();
  addResumeandPauseEvent()
  const game = new Game('canvas');
  game.game_init();
  // game.frame()
  function animation() {
    if (runing) {
      game.frame()
    }
    requestAnimationFrame(animation);
  }
  animation()
}

window.onload = () => {
  main()
}