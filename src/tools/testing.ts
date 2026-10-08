import { installPageChrome, openPage } from '../core/shell';
import { SCENARIOS, scenarioUrl } from './testScenarios';
import { applyTexts, t } from './texts';
import { LEVEL_TILES, nextFocus } from './testTiles';

installPageChrome();
applyTexts();

const list = document.getElementById('scenarios')!;
const levels = document.getElementById('levels')!;
const note = document.getElementById('note')!;

function addTile(parent: HTMLElement, titleText: string, descriptionText: string, onClick: () => void): void {
  const button = document.createElement('button');
  button.type = 'button';
  const title = document.createElement('strong');
  title.textContent = titleText;
  const description = document.createElement('span');
  description.textContent = descriptionText;
  button.append(title, description);
  button.addEventListener('click', onClick);
  parent.append(button);
}

for (const scenario of SCENARIOS) {
  addTile(list, scenario.title, scenario.description, () => {
    note.textContent = t('test.starting', { title: scenario.title });
    openPage(scenarioUrl(scenario, Date.now().toString(36))); // Uhrzeit als Kennung: jeder Start bekommt einen neuen Spielstand
  });
}
for (const tile of LEVEL_TILES) {
  addTile(levels, tile.title, tile.description, () => {
    note.textContent = t('tool.opening', { title: tile.title });
    openPage(tile.href);
  });
}

// Bedienung mit Controller: Steuerkreuz/Stick wählt, A klickt. B bleibt frei (Edge-Zurück), View + Menu übernimmt die Shell.
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

function pollPad(): void {
  const next = padKeys();
  const edge = (name: string) => next.has(name) && !held.has(name);
  const buttons = [...document.querySelectorAll<HTMLButtonElement>('main button')]; // Szenarien, dann Abschnitt „Level'
  const at = buttons.indexOf(document.activeElement as HTMLButtonElement);
  if (edge('next')) buttons[nextFocus(buttons.length, at, 'next')]?.focus();
  if (edge('prev')) buttons[nextFocus(buttons.length, at, 'prev')]?.focus();
  if (edge('a')) (document.activeElement as HTMLElement | null)?.click();
  held = next;
  requestAnimationFrame(pollPad);
}

list.querySelector('button')?.focus();
requestAnimationFrame(pollPad);
