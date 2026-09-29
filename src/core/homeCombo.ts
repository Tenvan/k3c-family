/**
 * Auf jeder Unterseite einbinden: View + Menu gleichzeitig (auf irgendeinem Controller) => zurück zur Landingpage.
 * B ist tabu (Edge auf der Xbox nutzt B als "Zurück"), daher diese Kombination.
 * Tastatur: Pos1/Home.
 */
const VIEW = 8;
const MENU = 9;
const HOLD_MS = 400;

export function installHomeCombo(homeUrl = './'): void {
  let heldSince = 0;

  const poll = () => {
    const pads = typeof navigator.getGamepads === 'function' ? navigator.getGamepads() : [];
    const combo = pads.some((p) => p?.buttons[VIEW]?.pressed && p.buttons[MENU]?.pressed);
    if (!combo) heldSince = 0;
    else if (heldSince === 0) heldSince = performance.now();
    else if (performance.now() - heldSince > HOLD_MS) return void location.assign(homeUrl);
    requestAnimationFrame(poll);
  };
  requestAnimationFrame(poll);

  addEventListener('keydown', (e) => {
    if (e.key === 'Home') location.assign(homeUrl);
  });
}
