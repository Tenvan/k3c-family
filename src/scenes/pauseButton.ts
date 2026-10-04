import { t } from '../core/texts';

/**
 * Touch-Schaltfläche „Optionen“ oben links (Home-Button oben mittig und Vorräte oben rechts bleiben frei).
 * Ein Knopf pro Seite, auch über Szenen-Neustarts hinweg; `take()` liefert einmal pro Druck `true`.
 */
export class PauseButton {
  private pressed = false;
  private readonly el: HTMLButtonElement;

  constructor(parent: HTMLElement = document.body) {
    this.el = document.createElement('button');
    this.el.type = 'button';
    this.relabel();
    this.el.style.cssText =
      'position:fixed;left:12px;top:64px;z-index:9;min-width:64px;min-height:64px;padding:8px 16px;font:bold 22px sans-serif;' +
      'color:#fff;background:rgba(0,0,0,.6);border:2px solid #fff;border-radius:12px;touch-action:manipulation';
    this.el.addEventListener('pointerdown', () => (this.pressed = true));
    parent.appendChild(this.el);
  }

  /** Beschriftung in der gewählten Sprache; nur schreiben, wenn sie sich geändert hat (einmal pro Frame aufgerufen). */
  private relabel(): void {
    const text = `☰ ${t('opt.openButton')}`;
    if (this.el.textContent === text) return;
    this.el.textContent = text;
    this.el.setAttribute('aria-label', t('opt.openButton'));
  }

  take(): boolean {
    this.relabel();
    const was = this.pressed;
    this.pressed = false;
    return was;
  }

  show(visible: boolean): void {
    this.el.hidden = !visible;
    this.relabel();
  }
}

let shared: PauseButton | undefined;
/** Ein Knopf pro Seite */
export function pauseButton(): PauseButton {
  return (shared ??= new PauseButton());
}
