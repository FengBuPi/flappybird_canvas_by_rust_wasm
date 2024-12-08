import router from '../Router/router.js';
import { Component } from './../utils/Router';
const gameStart = document.getElementsByClassName('game-start')[0] as HTMLElement;
const buttonPlay = document.getElementById('button-play');
const buttonScore = document.getElementById('button-score');

function mounted() {
  buttonPlay?.addEventListener('click', () => {
    router.push('/gameContainer',)
  })
}

function unmounted() {
  buttonPlay?.removeEventListener('click', () => { })
}

const initGameStartModule: Component = {
  template: gameStart,
  mounted,
  unmounted,
}
export { initGameStartModule };