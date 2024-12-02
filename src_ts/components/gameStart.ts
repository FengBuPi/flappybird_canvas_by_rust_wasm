const gameStart = document.getElementsByClassName('game-start')[0] as HTMLElement;
const buttonPlay = document.getElementById('button-play');
const buttonScore = document.getElementById('button-score');

function showGameStart() {
  if (gameStart!.style.display === 'initial') return
  gameStart!.style.display = 'initial';
}

function hiddenGameStart() {
  if (gameStart!.style.display === 'none') return
  gameStart!.style.display = 'none';
}

buttonPlay?.addEventListener('click', () => {
  hiddenGameStart();
})


export { showGameStart };