import type { PageEntry } from './pages';

/** Hinweis auf Kacheln und Seiten, die den Go-Server brauchen (GitHub Pages hat keinen, B-032). */
export const NO_SERVER_HINT = 'Braucht den Heimnetz-Server';

/** Kacheln dieses Abschnitts brauchen den Server; Test- und Infoseiten laufen ohne. */
export const needsServer = (page: Pick<PageEntry, 'section'>): boolean => page.section === 'play';

/** Hinweis statt Spiel in `parent` (die Shell-Leiste mit Home-Button liefert `installPageChrome()`). */
export function showNoServer(parent: HTMLElement): void {
  const box = document.createElement('div');
  box.style.cssText = 'color:#eee;font:1.5rem system-ui,sans-serif;text-align:center;padding:6rem 2rem 0;line-height:1.5';
  box.textContent = `${NO_SERVER_HINT}. Das Spiel läuft nur auf dem Go-Server (task serve), nicht auf einer statischen Seite wie GitHub Pages. Mit dem Home-Button geht es zurück.`;
  parent.append(box);
}
