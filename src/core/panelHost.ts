/**
 * Panel-Host (B-768, BD11.5): Werkzeug-Seiten öffnen sich im Spiel-Dokument selbst, vollflächig unter einer Leiste mit „Zurück“,
 * ohne dass die Seite navigiert. Das Panel ist ein iframe; die Seite darin ruft wie bisher `installPageChrome()` auf und meldet dem
 * Host per Nachricht (`home` = Panel schließen, `open` = andere Seite, `fullscreen` = Vollbild). Eine Seite, die ein Spiel starten
 * will (`game.html?…`), schließt das Panel und gibt die Konfiguration weiter (kein Spiel im Spiel). Die Nachrichten-Logik ist rein
 * (`panelAction`), das DOM dünn.
 */
import { toggleFullscreen } from './fullscreen';
import { runParamsOf, type RunParams } from './runConfig';
import { SHELL_MESSAGE, isOpenable, setHomeGuard, type ShellMessage } from './shell';
import { t } from './texts';

/** Was eine Nachricht der Seite im Panel auslöst */
export type PanelAction =
  | { kind: 'close' }
  | { kind: 'open'; href: string }
  | { kind: 'run'; params: RunParams }
  | { kind: 'fullscreen'; id: number };

/** Reine Auswertung einer Nachricht der Seite im Panel; `null` = ignorieren (unbekannt, ungültig, nicht öffnbar) */
export function panelAction(message: unknown): PanelAction | null {
  const m = message as Partial<ShellMessage> | null;
  if (!m || typeof m !== 'object') return null;
  if (m.type === SHELL_MESSAGE.home) return { kind: 'close' };
  if (m.type === SHELL_MESSAGE.fullscreen && typeof m.id === 'number') return { kind: 'fullscreen', id: m.id };
  if (m.type === SHELL_MESSAGE.open && typeof m.href === 'string' && isOpenable(m.href)) {
    const params = runParamsOf(m.href);
    return params ? { kind: 'run', params } : { kind: 'open', href: m.href };
  }
  return null;
}

/** Was das Spiel beim Öffnen und Schließen des Panels tun muss (Schleife anhalten, Szenario starten) */
export interface PanelHooks {
  onOpen(): void;
  onClose(): void;
  onRun(params: RunParams): void;
}

interface Panel {
  root: HTMLElement;
  frame: HTMLIFrameElement;
}

let panel: Panel | null = null;
let hooks: PanelHooks | null = null;

export const panelOpen = (): boolean => panel !== null;

const STYLE = `
  .k3c-panel { position: fixed; inset: 0; z-index: 2147482000; display: flex; flex-direction: column; background: #0b1026; }
  .k3c-panel-bar { display: flex; align-items: center; gap: 1em; padding: 0.5em 1em; background: #1b2550; color: #f1f3f9;
    font: 600 clamp(14px, 1.4vw, 22px) "Segoe UI", system-ui, sans-serif; border-bottom: 2px solid #3b3a6b; }
  .k3c-panel-bar button { font: inherit; color: #2a1d2e; background: #f3ead4; border: 0; border-radius: 0.4em; padding: 0.3em 0.9em; cursor: pointer; }
  .k3c-panel-bar button:focus-visible { outline: 3px solid #ffd166; outline-offset: 2px; }
  .k3c-panel-bar small { margin-left: auto; color: #a9b3cf; font-weight: 400; }
  .k3c-panel iframe { flex: 1; width: 100%; border: 0; background: #0b1026; }
`;

/** Verbindet den Host mit dem Spiel; einmal beim Start aufrufen */
export function installPanelHost(h: PanelHooks): void {
  hooks = h;
  setHomeGuard(panelOpen);
  addEventListener('message', (e: MessageEvent) => {
    if (!panel || e.origin !== location.origin || e.source !== panel.frame.contentWindow) return;
    const action = panelAction(e.data);
    if (!action) return;
    if (action.kind === 'close') closePanel();
    else if (action.kind === 'open') panel.frame.src = action.href;
    else if (action.kind === 'run') {
      closePanel();
      hooks?.onRun(action.params);
    } else {
      const target = panel.frame.contentWindow;
      void toggleFullscreen().then((error) => target?.postMessage({ type: SHELL_MESSAGE.fullscreenResult, id: action.id, error } satisfies ShellMessage, location.origin));
    }
  });
}

/** Öffnet eine Seite des Projekts im Panel (`name.html`, mit Abfrage); andere Adressen werden abgelehnt */
export function openPanel(href: string, title: string): void {
  if (!isOpenable(href) || panel) return;
  if (!document.getElementById('k3c-panel-style')) {
    const style = document.createElement('style');
    style.id = 'k3c-panel-style';
    style.textContent = STYLE;
    document.head.append(style);
  }
  const root = document.createElement('div');
  root.className = 'k3c-panel';
  const bar = document.createElement('div');
  bar.className = 'k3c-panel-bar';
  const back = document.createElement('button');
  back.type = 'button';
  back.textContent = t('panel.back');
  back.addEventListener('click', closePanel);
  const name = document.createElement('span');
  name.textContent = title;
  const hint = document.createElement('small');
  hint.textContent = t('panel.hint');
  bar.append(back, name, hint);
  const frame = document.createElement('iframe');
  frame.title = title;
  frame.allow = 'fullscreen; gamepad; autoplay';
  frame.addEventListener('load', () => {
    frame.contentWindow?.focus();
    frame.contentWindow?.addEventListener('keydown', (e) => e.key === 'Escape' && !e.defaultPrevented && closePanel());
  });
  frame.src = href;
  root.append(bar, frame);
  document.body.append(root);
  panel = { root, frame };
  document.querySelector<HTMLElement>('.k3c-home')?.style.setProperty('display', 'none'); // die Home-Taste der Seite im Panel genügt
  hooks?.onOpen();
}

export function closePanel(): void {
  if (!panel) return;
  const { root } = panel;
  root.remove();
  panel = null;
  document.querySelector<HTMLElement>('.k3c-home')?.style.removeProperty('display');
  window.focus();
  hooks?.onClose();
}
