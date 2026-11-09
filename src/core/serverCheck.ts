import { t } from './texts';

/** Hinweis statt Spiel in `parent`, wenn kein Go-Server antwortet (z. B. GitHub Pages hat keinen, B-032). */
export function showNoServer(parent: HTMLElement): void {
  const box = document.createElement('div');
  box.style.cssText = 'color:#eee;font:1.5rem system-ui,sans-serif;text-align:center;padding:6rem 2rem 0;line-height:1.5';
  box.textContent = t('landing.noServerText');
  parent.append(box);
}
