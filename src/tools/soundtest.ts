import { installPageChrome } from '../core/shell';

installPageChrome();
const hint = document.getElementById('hint');

// Entsperren per Geste (Tastatur, Klick/Touch, Controller-Taste); der Audio-Kern folgt mit SO3.2.
function unlock(): void {
  if (!hint || hint.classList.contains('on')) return;
  hint.classList.add('on');
  hint.textContent = 'Audio entsperrt';
}
addEventListener('keydown', unlock);
addEventListener('pointerdown', unlock);
addEventListener('gamepadconnected', () => {
  const poll = () => {
    const pressed = navigator.getGamepads().some((p) => p?.buttons.some((b) => b.pressed));
    if (pressed) unlock();
    else requestAnimationFrame(poll);
  };
  poll();
});
