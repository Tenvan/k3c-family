/**
 * Seiten-Rahmen ("Shell", B-768): Das Spiel (`index.html`) ist die einzige oberste Seite und navigiert nie, damit Vollbild erhalten
 * bleibt. Werkzeug-Seiten öffnen als Panel (iframe) im Spiel-Menü (`panelHost.ts`).
 *
 * REGEL: Jede Werkzeug-Seite ruft `installPageChrome()` auf, das Spiel selbst `installPageChrome({ home: false })`. Das liefert:
 *   - sichtbaren Home-Button (oben mittig), nicht im Spiel,
 *   - View + Menu gemeinsam halten (Controller) bzw. Pos1 (Tastatur) => zurück ins Menü (schließt das Panel), nicht im Spiel,
 *   - Zurück-Falle: Browser-Zurück (B auf der Xbox) schließt die Seite nicht versehentlich.
 * Läuft eine Werkzeug-Seite ausnahmsweise allein (direkt aufgerufen), führt Home per Link zum Spiel.
 */

import { installPadNav } from './padNav';
import { t } from './texts';

export const SHELL_MESSAGE = {
  home: 'k3c:home',
  fullscreen: 'k3c:fullscreen',
  fullscreenResult: 'k3c:fullscreen-result',
  open: 'k3c:open',
} as const;

export type ShellMessage =
  | { type: typeof SHELL_MESSAGE.home }
  | { type: typeof SHELL_MESSAGE.fullscreen; id: number }
  | { type: typeof SHELL_MESSAGE.fullscreenResult; id: number; error: string | null }
  | { type: typeof SHELL_MESSAGE.open; href: string };

/** true, wenn die Seite in der Landingpage-Shell läuft */
export const isEmbedded = typeof window !== 'undefined' && window.parent !== window;

export function postToShell(message: ShellMessage): void {
  window.parent.postMessage(message, location.origin);
}

/**
 * Darf die Shell diese Seite öffnen? Nur eine Seite dieses Ordners: `name.html` mit beliebiger Abfrage oder Hash, aber ohne Pfad, Schema,
 * `//` oder `..` und nicht die Landingpage selbst. Der Hash der Landingpage ist Nutzereingabe, `open` kommt von einer Seite im iframe.
 */
export function isOpenable(href: string): boolean {
  const path = href.split(/[?#]/, 1)[0] ?? '';
  return /^[\w.-]+\.html$/.test(path) && !path.includes('..') && path !== 'index.html';
}

/** Eine andere Seite öffnen. Seiten wechseln nie per Link oder `location` untereinander, sondern über die Shell (Regel in CLAUDE.md). */
export function openPage(href: string): void {
  if (!isOpenable(href)) return;
  if (isEmbedded) postToShell({ type: SHELL_MESSAGE.open, href });
  else location.assign(href);
}

/** Solange der Wächter `true` meldet, löst die Home-Kombi dieses Dokuments nichts aus (ein Werkzeug-Panel im Menü bedient sie selbst, B-768) */
let homeGuard: () => boolean = () => false;
export function setHomeGuard(guard: () => boolean): void {
  homeGuard = guard;
}

export function goHome(): void {
  if (isEmbedded) postToShell({ type: SHELL_MESSAGE.home });
  else location.assign('./');
}

const VIEW = 8;
const MENU = 9;
const HOLD_MS = 400;

export interface PageChromeOptions {
  /** Wird aufgerufen, wenn der Browser "Zurück" auslöst (auf der Xbox vermutlich die B-Taste). Die Seite bleibt offen. */
  onBack?: () => void;
  /** Controller bewegt den Fokus und klickt (B-767, `padNav.ts`); `false` für Seiten mit eigener Pad-Bedienung oder Phaser-Spiel. Standard: an. */
  padNav?: boolean;
  /** Home-Button, Home-Kombi (View + Menu) und Pos1; `false` für das Spiel selbst, das die oberste Seite ist (BD11.6). Standard: an. */
  home?: boolean;
}

/** Home-Button, Home-Kombi und Zurück-Falle installieren. Auf jeder Unterseite genau einmal aufrufen. */
export function installPageChrome(options: PageChromeOptions = {}): void {
  const home = options.home !== false;
  if (home) addHomeButton();
  installBackTrap(options.onBack);
  if (options.padNav !== false) installPadNav();

  let heldSince = 0;
  const poll = () => {
    const pads = typeof navigator.getGamepads === 'function' ? navigator.getGamepads() : [];
    if (pads.some((p) => p?.buttons.some((b) => b.pressed))) rearmOnFirstInteraction();
    const combo = home && pads.some((p) => p?.buttons[VIEW]?.pressed && p.buttons[MENU]?.pressed); // das Spiel (home: false) ist oben: kein „Home“
    if (!combo) heldSince = 0;
    else if (heldSince === 0) heldSince = performance.now();
    else if (performance.now() - heldSince > HOLD_MS) {
      heldSince = Infinity; // nur einmal auslösen, bis losgelassen
      if (!homeGuard()) goHome();
    }
    requestAnimationFrame(poll);
  };
  requestAnimationFrame(poll);

  addEventListener('keydown', (e) => {
    if (home && e.key === 'Home' && !homeGuard()) goHome();
  });
}

// ---------- Zurück-Falle ----------
// B ist auf dem Controller die typische "Abbrechen"-Taste. Löst sie in Edge ein Browser-Zurück aus, soll das nicht
// versehentlich das Spiel schließen. Ein eigener Verlaufseintrag fängt das ab: Zurück landet in popstate, die Seite bleibt.
let interacted = false;

function armBackTrap(): void {
  history.pushState({ k3cTrap: Date.now() }, '');
}

function rearmOnFirstInteraction(): void {
  if (interacted) return;
  interacted = true;
  // Chrome überspringt beim Zurückgehen teils Einträge, die ohne Nutzer-Interaktion angelegt wurden -> nachlegen.
  armBackTrap();
}

function installBackTrap(onBack?: () => void): void {
  armBackTrap();
  addEventListener('popstate', () => {
    onBack?.();
    armBackTrap();
  });
  addEventListener('pointerdown', rearmOnFirstInteraction);
  addEventListener('keydown', rearmOnFirstInteraction);
}

// ---------- Home-Button ----------
function addHomeButton(): void {
  const style = document.createElement('style');
  style.textContent = `
    .k3c-home {
      position: fixed; top: 0.6667em; left: 50%; translate: -50% 0; z-index: 2147483000;
      display: flex; align-items: center; gap: 0.5556em; padding: 0.4444em 0.8889em 0.4444em 0.6667em;
      font: 600 18px/1 "Segoe UI", system-ui, sans-serif; font-size: clamp(12px, 1.125vw, 18px); color: #f1f3f9; text-decoration: none;
      background: rgba(12, 16, 36, 0.72); border: 2px solid rgba(255, 255, 255, 0.18); border-radius: 999px;
      backdrop-filter: blur(6px); opacity: 0.8; transition: opacity 150ms ease, border-color 150ms ease;
    }
    @media (orientation: portrait) { .k3c-home { font-size: 14px; } }
    .k3c-home:hover, .k3c-home:focus-visible { opacity: 1; border-color: #ffd166; outline: none; }
    .k3c-home svg { width: 1.2222em; height: 1.2222em; fill: none; stroke: currentColor; stroke-width: 2.2; stroke-linejoin: round; }
    .k3c-home small { font-size: 0.7222em; font-weight: 600; color: #a9b3cf; }
  `;
  const button = document.createElement('a');
  button.className = 'k3c-home';
  button.href = './';
  button.title = t('shell.homeTitle');
  button.innerHTML = `<svg viewBox="0 0 24 24" aria-hidden="true"><path d="M3 11 12 3l9 8M5 9.5V21h5v-6h4v6h5V9.5"/></svg>
    <span>${t('shell.home')}</span><small>View+Menu</small>`;
  button.addEventListener('click', (e) => {
    e.preventDefault();
    goHome();
  });
  document.head.append(style);
  document.body.append(button);
}
