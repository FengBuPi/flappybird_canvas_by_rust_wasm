import { setRuningisTrue, setRuningisFalse } from '../State/gameState'
const resume = document.getElementById('resume')
const pause = document.getElementById('pause')

function addResumeandPauseEvent() {
  // 继续
  resume!.addEventListener('click', () => {
    console.log('继续')
    setRuningisTrue()
    revert()
  })
  // 暂停
  pause!.addEventListener('click', () => {
    setRuningisFalse()
    revert()
  })
}
// 反转暂停继续按钮
function revert() {
  resume!.style.display = resume!.style.display === 'none' ? 'initial' : 'none'
  pause!.style.display = pause!.style.display === 'none' ? 'initial' : 'none'
}

export { addResumeandPauseEvent }