import { setRuningisTrue, setRuningisFalse } from '../State/gameState.js'
import { Component } from './../utils/Router';
const buttonContainer = document.getElementById('game-container')
const resume = document.getElementById('resume')
const pause = document.getElementById('pause')

// 添加全局点击事件处理函数
function preventClick(e: MouseEvent) {
  // 如果点击的是暂停或继续按钮，则不阻止事件
  if (e.target === resume || e.target === pause) {
    return
  }
  e.stopPropagation()
  e.preventDefault()
}

function mounted() {
  // 继续
  resume!.addEventListener('click', () => {
    setRuningisTrue()
    revert()
    document.removeEventListener('click', preventClick, true) // 移除点击拦截
  })
  // 暂停
  pause!.addEventListener('click', () => {
    setRuningisFalse()
    revert()
    document.addEventListener('click', preventClick, true) // 添加点击拦截
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
  // 确保清理事件监听
  document.removeEventListener('click', preventClick, true)
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