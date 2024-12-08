// 游戏状态

// 是否运行
const runing: { value: boolean } = { value: true }
/**
 * 开始运行游戏
 */
function setRuningisTrue() {
  runing.value = true;
}

/**
 * 停止运行游戏
 */
function setRuningisFalse() {
  runing.value = false;
  console.log('停止运行游戏', runing.value)
}

export default runing;
export { setRuningisTrue, setRuningisFalse }