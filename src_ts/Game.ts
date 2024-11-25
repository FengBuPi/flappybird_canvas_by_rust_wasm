import init, { Game } from '../pkg/flappybird_canvas_by_rust_wasm.js';
// 图片预加载
let imgs = [
  "/asset/images/sky.png",
  "/asset/images/land.png",
  "/asset/images/pipe1.png",
  "/asset/images/pipe2.png",
  "/asset/images/birds.png",
];
for (const img of imgs) {
  let image = new Image();
  image.src = img;
}
// 主函数
async function main(): Promise<void> {
  await init();
  const game = new Game('canvas');
  game.game_init();
  // 是否运行
  let runing: boolean = true
  function animation() {
    if (runing) {
      game.frame()
    }
    requestAnimationFrame(animation);
  }
  animation()

  const resume = document.getElementById('resume')
  const pause = document.getElementById('pause')
  // 继续
  resume!.addEventListener('click', () => {
    runing = !runing
    revert()
  })
  // 暂停
  pause!.addEventListener('click', () => {
    runing = !runing
    revert()
  })
  // 反转暂停继续按钮
  function revert() {
    resume!.style.display = resume!.style.display === 'none' ? 'initial' : 'none'
    pause!.style.display = pause!.style.display === 'none' ? 'initial' : 'none'
  }
}

window.onload = () => {
  main()
}