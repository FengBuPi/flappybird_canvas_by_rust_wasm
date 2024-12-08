import { setRuningisTrue, setRuningisFalse } from '../State/gameState.js'
import { Component } from './../utils/Router';
const buttonContainer = document.getElementById('game-container')
const resume = document.getElementById('resume')
const pause = document.getElementById('pause')

function mounted() {
  // 继续
  resume!.addEventListener('click', () => {
    setRuningisTrue()
    revert()
  })
  // 暂停
  pause!.addEventListener('click', () => {
    setRuningisFalse()
    revert()
  })
}

function unmounted() {
  resume!.removeEventListener('click', () => {
    console.log('继续')
    setRuningisTrue()
    revert()
  })
  pause!.removeEventListener('click', () => {
    setRuningisFalse()
    revert()
  })
}

// 反转暂停继续按钮
function revert() {
  resume!.style.display = resume!.style.display === 'none' ? 'initial' : 'none'
  pause!.style.display = pause!.style.display === 'none' ? 'initial' : 'none'
}

const initGamePauseModule: Component = {
  template: buttonContainer!,
  mounted,
  unmounted,
}
export { initGamePauseModule }