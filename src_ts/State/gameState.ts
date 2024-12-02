// 游戏状态

// 是否运行
let runing: boolean = true
/**
 * 开始运行游戏
 */
function setRuningisTrue() {
  runing = true;
}

/**
 * 停止运行游戏
 */
function setRuningisFalse() {
  runing = false;
}

export default runing;
export { setRuningisTrue, setRuningisFalse }