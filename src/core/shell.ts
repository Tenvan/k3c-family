/**
 * Seiten-Rahmen ("Shell"): Die Landingpage (index.html) bleibt dauerhaft geöffnet und zeigt alle anderen Seiten
 * in einem Vollflächen-iframe. So bleibt Vollbild über Seitenwechsel erhalten.
 *
 * REGEL: Jede Seite außer der Landingpage ruft `installPageChrome()` auf. Das liefert:
 *   - sichtbaren Home-Button (oben mittig),
 *   - View + Menu gemeinsam halten (Controller) bzw. Pos1 (Tastatur) => zurück zur Landingpage,
 *   - Zurück-Falle: Browser-Zurück (B auf der Xbox) schließt die Seite nicht versehentlich.
 * Läuft die Seite ausnahmsweise ohne Shell (direkt aufgerufen), führt Home per Link zur Landingpage.
 */

export const SHELL_MESSAGE = {
  home: 'k3c:home',
  fullscreen: 'k3c:fullscreen',
  fullscreenResult: 'k3c:fullscreen-result',
} as const;

export type ShellMessage =
  | { type: typeof SHELL_MESSAGE.home }
  | { type: typeof SHELL_MESSAGE.fullscreen; id: number }
  | { type: typeof SHELL_MESSAGE.fullscreenResult; id: number; error: string | null };

/** true, wenn die Seite in der Landingpage-Shell läuft */
export const isEmbedded = window.parent !== window;

export function postToShell(message: ShellMessage): void {
  window.parent.postMessage(message, location.origin);
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
}

/** Home-Button, Home-Kombi und Zurück-Falle installieren. Auf jeder Unterseite genau einmal aufrufen. */
export function installPageChrome(options: PageChromeOptions = {}): void {
  addHomeButton();
  installBackTrap(options.onBack);

  let heldSince = 0;
  const poll = () => {
    const pads = typeof navigator.getGamepads === 'function' ? navigator.getGamepads() : [];
    if (pads.some((p) => p?.buttons.some((b) => b.pressed))) rearmOnFirstInteraction();
    const combo = pads.some((p) => p?.buttons[VIEW]?.pressed && p.buttons[MENU]?.pressed);
    if (!combo) heldSince = 0;
    else if (heldSince === 0) heldSince = performance.now();
    else if (performance.now() - heldSince > HOLD_MS) {
      heldSince = Infinity; // nur einmal auslösen, bis losgelassen
      goHome();
    }
    requestAnimationFrame(poll);
  };
  requestAnimationFrame(poll);

  addEventListener('keydown', (e) => {
    if (e.key === 'Home') goHome();
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
      position: fixed; top: 12px; left: 50%; translate: -50% 0; z-index: 2147483000;
      display: flex; align-items: center; gap: 10px; padding: 8px 16px 8px 12px;
      font: 600 18px/1 "Segoe UI", system-ui, sans-serif; color: #f1f3f9; text-decoration: none;
      background: rgba(12, 16, 36, 0.72); border: 2px solid rgba(255, 255, 255, 0.18); border-radius: 999px;
      backdrop-filter: blur(6px); opacity: 0.8; transition: opacity 150ms ease, border-color 150ms ease;
    }
    .k3c-home:hover, .k3c-home:focus-visible { opacity: 1; border-color: #ffd166; outline: none; }
    .k3c-home svg { width: 22px; height: 22px; fill: none; stroke: currentColor; stroke-width: 2.2; stroke-linejoin: round; }
    .k3c-home small { font-size: 13px; font-weight: 600; color: #a9b3cf; }
  `;
  const button = document.createElement('a');
  button.className = 'k3c-home';
  button.href = './';
  button.title = 'Zurück zur Startseite (View + Menu, Pos1)';
  button.innerHTML = `<svg viewBox="0 0 24 24" aria-hidden="true"><path d="M3 11 12 3l9 8M5 9.5V21h5v-6h4v6h5V9.5"/></svg>
    <span>Start</span><small>View+Menu</small>`;
  button.addEventListener('click', (e) => {
    e.preventDefault();
    goHome();
  });
  document.head.append(style);
  document.body.append(button);
}
