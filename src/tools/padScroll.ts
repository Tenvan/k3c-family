/** Seite mit Stick oder Steuerkreuz scrollen (B bleibt frei). */
export function installPadScroll(): void {
  const tick = () => {
    const pads = typeof navigator.getGamepads === 'function' ? navigator.getGamepads() : [];
    let dy = 0;
    for (const p of pads) {
      if (!p) continue;
      const y = p.axes[1] ?? 0;
      if (Math.abs(y) > 0.2) dy += y * 24;
      if (p.buttons[12]?.pressed) dy -= 16;
      if (p.buttons[13]?.pressed) dy += 16;
    }
    if (dy) scrollBy(0, dy);
    requestAnimationFrame(tick);
  };
  requestAnimationFrame(tick);
}
