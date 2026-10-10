import { t } from '../core/texts';
import type { PageEntry } from './pages';

/** Kacheln dieses Abschnitts brauchen den Server (GitHub Pages hat keinen, B-032); Test- und Infoseiten laufen ohne. */
export const needsServer = (page: Pick<PageEntry, 'section'>): boolean => page.section === 'play';

/** Hinweis statt Spiel in `parent` (die Shell-Leiste mit Home-Button liefert `installPageChrome()`). */
export function showNoServer(parent: HTMLElement): void {
  const box = document.createElement('div');
  box.style.cssText = 'color:#eee;font:1.5rem system-ui,sans-serif;text-align:center;padding:6rem 2rem 0;line-height:1.5';
  box.textContent = t('landing.noServerText');
  parent.append(box);
}
