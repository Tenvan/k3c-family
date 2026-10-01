import { installPageChrome, openPage } from '../core/shell';
import { SCENARIOS, scenarioUrl } from './testScenarios';

installPageChrome();

const list = document.getElementById('scenarios')!;
const note = document.getElementById('note')!;

for (const scenario of SCENARIOS) {
  const button = document.createElement('button');
  button.type = 'button';
  const title = document.createElement('strong');
  title.textContent = scenario.title;
  const description = document.createElement('span');
  description.textContent = scenario.description;
  button.append(title, description);
  button.addEventListener('click', () => {
    note.textContent = `Starte „${scenario.title}“ …`;
    openPage(scenarioUrl(scenario, Date.now().toString(36))); // Uhrzeit als Kennung: jeder Start bekommt einen neuen Spielstand
  });
  list.append(button);
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
  const buttons = [...list.querySelectorAll('button')];
  const at = Math.max(0, buttons.indexOf(document.activeElement as HTMLButtonElement));
  if (edge('next')) buttons[Math.min(buttons.length - 1, at + 1)]?.focus();
  if (edge('prev')) buttons[Math.max(0, at - 1)]?.focus();
  if (edge('a')) (document.activeElement as HTMLElement | null)?.click();
  held = next;
  requestAnimationFrame(pollPad);
}

list.querySelector('button')?.focus();
requestAnimationFrame(pollPad);
