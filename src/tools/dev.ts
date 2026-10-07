import { installPageChrome, openPage } from '../core/shell';
import { DEV_SECTIONS, type DevTile } from './devTiles';
import { type FocusKey, nextFocus } from './testTiles';

installPageChrome();

const root = document.getElementById('sections')!;
const note = document.getElementById('note')!;

/** Öffnet eine Kachel: Shell-Seiten über `openPage()`, Seiten außerhalb der Shell (`external`) in einem neuen Fenster. */
function launch(tile: DevTile): void {
  if (!tile.external) {
    note.textContent = `Öffne „${tile.title}“ …`;
    openPage(tile.href);
    return;
  }
  const url = new URL(tile.href, location.href).href;
  // Ohne `noopener`: nur so meldet `window.open` eine Pop-up-Sperre mit `null`. Die Seite stammt aus diesem Ordner.
  const win = window.open(url, '_blank');
  note.textContent = win ? `„${tile.title}“ öffnet in einem neuen Fenster.` : `Neues Fenster gesperrt: ${url} bitte von Hand öffnen.`;
}

function tileButton(tile: DevTile): HTMLButtonElement {
  const button = document.createElement('button');
  button.type = 'button';
  const title = document.createElement('strong');
  title.textContent = `${tile.icon} ${tile.title}`;
  const description = document.createElement('span');
  description.textContent = tile.description;
  button.append(title, description);
  button.addEventListener('click', () => launch(tile));
  return button;
}

for (const section of DEV_SECTIONS) {
  const box = document.createElement('section');
  const heading = document.createElement('h2');
  heading.textContent = section.title;
  box.append(heading);
  if (section.tiles.length === 0) {
    const empty = document.createElement('p');
    empty.className = 'empty';
    empty.textContent = section.empty;
    box.append(empty);
  } else {
    const grid = document.createElement('div');
    grid.className = 'grid';
    grid.append(...section.tiles.map(tileButton));
    box.append(grid);
  }
  root.append(box);
}

// Bedienung mit Controller wie auf der Testseite: Steuerkreuz/Stick wählt, A klickt. B bleibt frei (Edge-Zurück), View + Menu übernimmt die Shell.
const PAD = { A: 0, UP: 12, DOWN: 13, LEFT: 14, RIGHT: 15 } as const;
const STICK = 0.5;
let held = new Set<string>();

/** Gerade gehaltene Tasten aller Pads: 'a', 'prev' (links/oben), 'next' (rechts/unten) */
function padKeys(): Set<string> {
  const keys = new Set<string>();
  for (const pad of navigator.getGamepads?.() ?? []) {
    if (!pad) continue;
    const on = (i: number) => pad.buttons[i]?.pressed;
    const [x = 0, y = 0] = pad.axes;
    if (on(PAD.A)) keys.add('a');
    if (on(PAD.LEFT) || on(PAD.UP) || Math.min(x, y) < -STICK) keys.add('prev');
    if (on(PAD.RIGHT) || on(PAD.DOWN) || Math.max(x, y) > STICK) keys.add('next');
  }
  return keys;
}

/** Fokus eine Kachel weiter oder zurück (Controller und Pfeiltasten). */
function moveFocus(key: FocusKey): void {
  const buttons = [...document.querySelectorAll<HTMLButtonElement>('main button')];
  const at = buttons.indexOf(document.activeElement as HTMLButtonElement);
  buttons[nextFocus(buttons.length, at, key)]?.focus();
}

function pollPad(): void {
  const next = padKeys();
  const edge = (name: string) => next.has(name) && !held.has(name);
  if (edge('next')) moveFocus('next');
  if (edge('prev')) moveFocus('prev');
  if (edge('a')) (document.activeElement as HTMLElement | null)?.click();
  held = next;
  requestAnimationFrame(pollPad);
}

// Tastatur wie auf der Landingpage: Pfeile wählen, Enter öffnet (Enter klickt den fokussierten Knopf von selbst).
const ARROWS: Record<string, FocusKey> = { ArrowLeft: 'prev', ArrowUp: 'prev', ArrowRight: 'next', ArrowDown: 'next' };
document.addEventListener('keydown', (e) => {
  const key = ARROWS[e.key];
  if (!key) return;
  e.preventDefault();
  moveFocus(key);
});

root.querySelector('button')?.focus();
requestAnimationFrame(pollPad);
