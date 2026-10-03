/** Reine Teile der Lade-Szene: Texte und Balkenbreite, ohne Phaser. */

const clamp01 = (v: number) => Math.min(1, Math.max(0, Number.isFinite(v) ? v : 0));

/** Fortschritt von Phaser (0..1) als Prozenttext, außerhalb des Bereichs begrenzt. */
export function progressText(value: number): string {
  return `Lade Grafiken … ${Math.round(clamp01(value) * 100)} %`;
}

/** Breite des gefüllten Balkens in Pixeln. */
export function barFill(value: number, width: number): number {
  return Math.round(clamp01(value) * width);
}

/** Meldung bei Ladefehlern: Dateiname der fehlgeschlagenen Assets (ohne Doppelte). */
export function errorText(files: readonly string[]): string {
  const unique = [...new Set(files)];
  return `Laden fehlgeschlagen:\n${unique.join('\n')}\n\nSeite neu laden (F5) oder Server prüfen.`;
}
