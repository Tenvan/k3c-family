import { fullscreenSupported, isFullscreen, onFullscreenChange, toggleLocal } from '../core/fullscreen';
import { SHELL_MESSAGE, isOpenable, type ShellMessage } from '../core/shell';
import { PAGES, SECTIONS, type PageEntry } from './pages';
import { CLIENT, fetchServerBuild, versionLine, versionMismatch } from '../core/version';
import { NO_SERVER_HINT, needsServer } from './serverCheck';

/**
 * Landingpage = dauerhaft offene Shell.
 * - Kacheln aus pages.ts, Navigation per Gamepad (D-Pad/Stick + A), Tastatur (Pfeile + Enter) und Maus.
 *   Die Richtungsnavigation ist räumlich (nächste Kachel in Blickrichtung), funktioniert also für jedes Layout.
 * - Seiten öffnen sich in einem Vollflächen-iframe (#frame). Vollbild bleibt dadurch über Seitenwechsel erhalten.
 * - Zurück: Home-Button/View+Menu der Unterseite (postMessage). Browser-Zurück (B) wird abgefangen.
 */

const A = 0;
const Y = 3;
const VIEW = 8;
const MENU = 9;
const DPAD = { up: 12, down: 13, left: 14, right: 15 } as const;
const STICK_THRESHOLD = 0.5;
const REPEAT_DELAY_MS = 380;
const REPEAT_RATE_MS = 150;
const HOME_HOLD_MS = 400;
const FOCUS_KEY = 'k3c.landing.focus';

type Dir = 'up' | 'down' | 'left' | 'right';

const menu = document.getElementById('menu')!;
/** Alles, was per D-Pad auswählbar ist: Kacheln + Vollbild-Knopf. */
const cards: HTMLElement[] = [];

// ---------- Kacheln ----------
function render(): void {
  for (const section of Object.keys(SECTIONS) as PageEntry['section'][]) {
    const pages = PAGES.filter((p) => p.section === section);
    if (pages.length === 0) continue;
    const h2 = document.createElement('h2');
    h2.textContent = SECTIONS[section];
    const grid = document.createElement('div');
    grid.className = 'grid';
    for (const page of pages) {
      const a = document.createElement('a');
      a.className = `card${page.primary ? ' primary' : ''}${page.small ? ' small' : ''}`;
      a.href = typeof page.href === 'string' ? page.href : '#';
      a.dataset.title = page.title;
      if (needsServer(page)) a.dataset.needsServer = '';
      a.innerHTML = `<span class="icon">${page.icon}</span><span><p class="title"></p><p class="desc"></p></span>`;
      a.querySelector('.title')!.textContent = page.title;
      a.querySelector('.desc')!.textContent = page.description;
      a.addEventListener('click', (e) => {
        e.preventDefault();
        launch(a, page);
      });
      grid.append(a);
      register(a);
    }
    menu.append(h2, grid);
  }
}

/** Macht ein Element per Controller/Tastatur/Maus auswählbar. */
function register(el: HTMLElement): void {
  // Nur echte Mausbewegung wählt aus. Ein still stehender Cursor (z.B. Edge-Cursor auf der Xbox)
  // soll die Controller-Auswahl nicht überschreiben, wenn die Seite scrollt oder sichtbar wird.
  el.addEventListener('pointermove', (e) => {
    if (e.movementX !== 0 || e.movementY !== 0) select(el);
  });
  el.addEventListener('focus', () => select(el)); // Tab-Taste
  cards.push(el);
}

/**
 * Wählt eine Kachel aus. Eigene Klasse statt nur :focus, denn ohne Fensterfokus (z.B. wenn der
 * Controller-Cursor auf der Xbox woanders ist) greift weder :focus noch das focus-Event.
 */
function select(card: HTMLElement): void {
  if (card.classList.contains('focus')) return;
  cards.forEach((c) => c.classList.toggle('focus', c === card));
  card.focus({ preventScroll: true });
  card.scrollIntoView({ block: 'nearest', behavior: 'smooth' });
  try {
    sessionStorage.setItem(FOCUS_KEY, card.dataset.title!);
  } catch {
    /* Speicher nicht verfügbar – egal */
  }
}

/** Ohne Go-Server (z. B. GitHub Pages) sind die Spiel-Kacheln deaktiviert und erklärt; Testseiten bleiben offen (B-032). */
function markNoServer(): void {
  for (const card of cards.filter((c) => c.dataset.needsServer !== undefined)) {
    card.classList.add('disabled');
    card.querySelector('.desc')!.textContent = NO_SERVER_HINT;
  }
}

function restoreFocus(): void {
  let title: string | null = null;
  try {
    title = sessionStorage.getItem(FOCUS_KEY);
  } catch {
    /* ignorieren */
  }
  const card = cards.find((c) => c.dataset.title === title) ?? cards[0];
  if (card) select(card);
}

function current(): HTMLElement {
  return (cards.find((c) => c.classList.contains('focus')) ?? cards[0])!;
}

/** Nächste Kachel in Richtung `dir`: Abstand in Blickrichtung zählt einfach, seitlicher Versatz doppelt. */
function move(dir: Dir): void {
  const from = current().getBoundingClientRect();
  const fx = from.left + from.width / 2;
  const fy = from.top + from.height / 2;
  let best: HTMLElement | null = null;
  let bestScore = Infinity;

  for (const card of cards) {
    if (card === current()) continue;
    const r = card.getBoundingClientRect();
    const dx = r.left + r.width / 2 - fx;
    const dy = r.top + r.height / 2 - fy;
    const [main, side] = dir === 'left' ? [-dx, dy] : dir === 'right' ? [dx, dy] : dir === 'up' ? [-dy, dx] : [dy, dx];
    if (main <= 1) continue;
    const score = main + Math.abs(side) * 2;
    if (score < bestScore) {
      bestScore = score;
      best = card;
    }
  }

  if (best) select(best);
}

// ---------- Seiten im iframe ----------
// Das iframe wird bei jedem Öffnen neu erzeugt und beim Schließen entfernt. Würde man nur `src` wechseln,
// legte der Browser Verlaufseinträge an, und "Zurück" könnte alte Seiten unsichtbar im Hintergrund laden.
let frame: HTMLIFrameElement | null = null;

/** Nur Seiten dieses Ordners (*.html) zulassen – der Hash in der URL und `open` von Seiten sind Eingaben von außen. */
function safePageUrl(href: string): string | null {
  return isOpenable(href) ? href : null;
}

function launch(card: HTMLElement, page: PageEntry): void {
  if (card.classList.contains('disabled')) return;
  card.classList.add('launch');
  const href = typeof page.href === 'function' ? page.href() : page.href;
  setTimeout(() => {
    card.classList.remove('launch');
    openPage(href);
  }, 120);
}

function openPage(href: string): void {
  const safe = safePageUrl(href);
  if (!safe) return;
  frame?.remove();
  frame = document.createElement('iframe');
  frame.id = 'frame';
  frame.title = 'Seite';
  frame.allow = 'fullscreen; gamepad; autoplay';
  frame.src = safe;
  frame.addEventListener('load', () => frame?.contentWindow?.focus(), { once: true });
  document.body.append(frame);
  document.body.classList.add('page-open');
  // Hash nur ersetzen (kein neuer Verlaufseintrag): Neuladen öffnet dieselbe Seite wieder.
  history.replaceState(history.state, '', `#${safe}`);
}

function closePage(): void {
  if (!frame) return;
  frame.remove();
  frame = null;
  document.body.classList.remove('page-open');
  history.replaceState(history.state, '', location.pathname + location.search);
  window.focus();
}

function isPageOpen(): boolean {
  return frame !== null;
}

// Zurück-Falle auch für die Landingpage: B soll Edge nicht verlassen.
history.pushState({ k3cShell: true }, '');
addEventListener('popstate', () => history.pushState({ k3cShell: true }, ''));

addEventListener('message', (e: MessageEvent<ShellMessage>) => {
  if (e.origin !== location.origin || !frame || e.source !== frame.contentWindow) return;
  if (e.data?.type === SHELL_MESSAGE.home) closePage();
  if (e.data?.type === SHELL_MESSAGE.open) openPage(e.data.href);
  if (e.data?.type === SHELL_MESSAGE.fullscreen) {
    const id = e.data.id;
    const target = frame.contentWindow;
    void toggleLocal().then((error) => {
      target?.postMessage({ type: SHELL_MESSAGE.fullscreenResult, id, error } satisfies ShellMessage, location.origin);
    });
  }
});

// ---------- Vollbild ----------
const fsButton = document.getElementById('fullscreen') as HTMLButtonElement;
const fsLabel = document.getElementById('fullscreen-label')!;
const toast = document.getElementById('toast')!;
let toastTimer = 0;

function showToast(message: string): void {
  toast.textContent = message;
  toast.classList.add('show');
  clearTimeout(toastTimer);
  toastTimer = window.setTimeout(() => toast.classList.remove('show'), 5000);
}

async function fullscreen(via: string): Promise<void> {
  const error = await toggleLocal();
  if (!error) return;
  showToast(
    via === 'Klick'
      ? 'Der Browser erlaubt hier kein Vollbild. Auf der Xbox alternativ das Vollbild aus dem Edge-Menü nutzen.'
      : `Vollbild per ${via} blockiert. Bitte mit dem Cursor auf „Vollbild“ klicken.`,
  );
}

function setupFullscreen(): void {
  if (!fullscreenSupported()) return void fsButton.remove();
  fsButton.dataset.title = '__fullscreen';
  fsButton.addEventListener('click', () => void fullscreen('Klick'));
  const update = (active: boolean) => {
    fsLabel.textContent = active ? 'Vollbild beenden' : 'Vollbild';
    fsButton.classList.toggle('active', active);
  };
  onFullscreenChange(update);
  update(isFullscreen());
  register(fsButton);
}

// ---------- Tastatur ----------
addEventListener('keydown', (e) => {
  if (isPageOpen()) {
    if (e.key === 'Home') closePage();
    return;
  }
  const dir = ({ ArrowUp: 'up', ArrowDown: 'down', ArrowLeft: 'left', ArrowRight: 'right' } as Record<string, Dir>)[e.key];
  if (dir) {
    e.preventDefault();
    move(dir);
  } else if (e.key === 'f' || e.key === 'F') {
    void fullscreen('Taste F');
  }
});

// ---------- Gamepad ----------
const statusEl = document.getElementById('pad-status')!;
const statusText = document.getElementById('pad-status-text')!;
const heldSince = new Map<string, number>();
const lastRepeat = new Map<string, number>();
// true: beim Zurückkommen noch gehaltene Tasten lösen nichts aus
let aWasDown = true;
let yWasDown = true;
let comboSince = 0;

function pollGamepads(now: number): void {
  const pads = (typeof navigator.getGamepads === 'function' ? navigator.getGamepads() : []).filter((p): p is Gamepad => !!p);

  if (isPageOpen()) {
    // Die Unterseite hat die Kontrolle. Nur die Home-Kombi beobachten, falls der Browser die Daten (auch) hierher liefert.
    const combo = pads.some((p) => p.buttons[VIEW]?.pressed && p.buttons[MENU]?.pressed);
    if (!combo) comboSince = 0;
    else if (comboSince === 0) comboSince = now;
    else if (now - comboSince > HOME_HOLD_MS) {
      comboSince = Infinity;
      closePage();
    }
    aWasDown = yWasDown = true;
    heldSince.clear();
    return void requestAnimationFrame(pollGamepads);
  }

  statusEl.classList.toggle('on', pads.length > 0);
  statusText.textContent = pads.length === 0 ? 'Controller: Taste drücken' : `${pads.length} Controller verbunden`;

  const dirs = new Set<Dir>();
  let aDown = false;
  let yDown = false;
  for (const pad of pads) {
    const b = (i: number) => !!pad.buttons[i]?.pressed;
    const [x = 0, y = 0] = pad.axes;
    if (b(DPAD.up) || y < -STICK_THRESHOLD) dirs.add('up');
    if (b(DPAD.down) || y > STICK_THRESHOLD) dirs.add('down');
    if (b(DPAD.left) || x < -STICK_THRESHOLD) dirs.add('left');
    if (b(DPAD.right) || x > STICK_THRESHOLD) dirs.add('right');
    if (b(A)) aDown = true;
    if (b(Y)) yDown = true;
  }

  // Richtung: sofort einmal, nach kurzer Pause wiederholen solange gehalten.
  for (const dir of ['up', 'down', 'left', 'right'] as Dir[]) {
    if (!dirs.has(dir)) {
      heldSince.delete(dir);
      continue;
    }
    const since = heldSince.get(dir);
    if (since === undefined) {
      heldSince.set(dir, now);
      lastRepeat.set(dir, now);
      move(dir);
    } else if (now - since > REPEAT_DELAY_MS && now - lastRepeat.get(dir)! > REPEAT_RATE_MS) {
      lastRepeat.set(dir, now);
      move(dir);
    }
  }

  if (aDown && !aWasDown) current().click();
  aWasDown = aDown;
  if (yDown && !yWasDown) void fullscreen('Controller (Y)');
  yWasDown = yDown;

  requestAnimationFrame(pollGamepads);
}

render();
setupFullscreen();
restoreFocus();
// Ohne Go-Server: Spiel-Kacheln aus; die Kopfzeile zeigt Client- und Server-Version (Tag und Buildzeit).
void fetchServerBuild().then((server) => {
  if (!server) markNoServer();
  const el = document.getElementById('version')!;
  el.textContent = versionLine(CLIENT, server);
  el.classList.toggle('mismatch', versionMismatch(CLIENT, server));
});
// Direkt-Link / Neuladen mit geöffneter Seite (index.html#game.html?seed=abc)
if (location.hash.length > 1) openPage(decodeURIComponent(location.hash.slice(1)));
requestAnimationFrame(pollGamepads);
