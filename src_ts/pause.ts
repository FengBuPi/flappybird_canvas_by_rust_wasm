const resume = document.getElementById('resume')
const pause = document.getElementById('pause')

// 是否运行
let runing: boolean = true
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



export { runing }