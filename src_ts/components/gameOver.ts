const gameOver = document.getElementById('game-over');
import { Component } from './../utils/Router';
// const gameOver = document.getElementById('game-over-title')
/**
 * 显示游戏结束
 */
// function showGameOver() {
//   if (gameOver!.style.display === 'none') {
//     gameOver!.style.display = 'initial'
//   }
// }
// /**
//  * 隐藏游戏结束
//  */
// function hiddenGameOver() {
//   if (gameOver!.style.display === 'initial') {
//     gameOver!.style.display = 'none'
//   }
// }
const initGameOverModule: Component = {
  template: gameOver!,
  mounted() {
  },
  unmounted() {
  }
}
export { initGameOverModule }