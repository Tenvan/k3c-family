import { fullscreenSupported, isFullscreen, onFullscreenChange, toggleFullscreen } from '../core/fullscreen';
import { PAGES, SECTIONS, type PageEntry } from './pages';

/**
 * Landingpage: Kacheln aus pages.ts, Navigation per Gamepad (D-Pad/Stick + A), Tastatur (Pfeile + Enter) und Maus.
 * Die Richtungsnavigation ist räumlich (nächste Kachel in Blickrichtung), funktioniert also für jedes Grid-Layout.
 */

const A = 0;
const Y = 3;
const DPAD = { up: 12, down: 13, left: 14, right: 15 } as const;
const STICK_THRESHOLD = 0.5;
const REPEAT_DELAY_MS = 380;
const REPEAT_RATE_MS = 150;
const FOCUS_KEY = 'k3c.landing.focus';

type Dir = 'up' | 'down' | 'left' | 'right';

const menu = document.getElementById('menu')!;
/** Alles, was per D-Pad auswählbar ist: Kacheln + Vollbild-Knopf. */
const cards: HTMLElement[] = [];

function resolveHref(page: PageEntry): string {
  return typeof page.href === 'function' ? page.href() : page.href;
}

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
      a.className = `card${page.primary ? ' primary' : ''}`;
      a.href = typeof page.href === 'string' ? page.href : '#';
      a.dataset.title = page.title;
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
  const error = await toggleFullscreen();
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
  onFullscreenChange((active) => {
    fsLabel.textContent = active ? 'Vollbild beenden' : 'Vollbild';
    fsButton.classList.toggle('active', active);
  });
  fsLabel.textContent = isFullscreen() ? 'Vollbild beenden' : 'Vollbild';
  register(fsButton);
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
  remember(card.dataset.title!);
}

function remember(title: string): void {
  try {
    sessionStorage.setItem(FOCUS_KEY, title);
  } catch {
    /* Speicher nicht verfügbar – egal */
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

function launch(card: HTMLAnchorElement, page: PageEntry): void {
  card.classList.add('launch');
  const href = resolveHref(page);
  setTimeout(() => location.assign(href), 120);
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

// ---------- Tastatur ----------
addEventListener('keydown', (e) => {
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
// true: ein beim Laden noch gehaltenes A/Y (von der vorherigen Seite) löst nichts aus
let aWasDown = true;
let yWasDown = true;

function pollGamepads(now: number): void {
  const pads = (typeof navigator.getGamepads === 'function' ? navigator.getGamepads() : []).filter((p): p is Gamepad => !!p);

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
// Zurück-Navigation (bfcache): Kachel-Animation zurücksetzen.
addEventListener('pageshow', () => cards.forEach((c) => c.classList.remove('launch')));
requestAnimationFrame(pollGamepads);
