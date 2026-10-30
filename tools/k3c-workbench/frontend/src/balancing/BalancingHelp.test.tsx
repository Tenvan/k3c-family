// @vitest-environment happy-dom
import { Theme } from '@radix-ui/themes';
import { act } from 'react';
import { createRoot, type Root } from 'react-dom/client';
import { afterEach, describe, expect, it } from 'vitest';
import { pickValue } from '@webkit/testing/select';
import { backend } from '../api';
import { BalancingPage } from './BalancingPage';
import { appendLine, EXAMPLES, FIELD_HELP, formFromExample, rangeAround, resizeMix, type HelpKey } from './help';
import { EMPTY_FORM, specFromForm } from './spec';

// WZ28.10/AC-10 (B-609): jedes Feld hat Hilfetext und Hilfe-Knopf, Beispiele lassen sich übernehmen.
// WZ28.10/AC-11 (B-610): Liste der Konfigurationsdateien mit Popup und Übernahme von Werten (Mock-Backend).

(globalThis as { IS_REACT_ACT_ENVIRONMENT?: boolean }).IS_REACT_ACT_ENVIRONMENT = true;

let root: Root | null = null;
const wait = (ms: number) => act(async () => new Promise((r) => setTimeout(r, ms)));
async function flush() {
  for (let i = 0; i < 5; i++) await wait(0);
}

async function mount(): Promise<HTMLElement> {
  const host = document.createElement('div');
  document.body.appendChild(host);
  root = createRoot(host);
  act(() => root!.render(<Theme><BalancingPage /></Theme>));
  await flush();
  // B-659: Das Formular steht im Detail und öffnet mit „Neuer Lauf“ in der Werkzeugleiste.
  await act(async () => [...host.querySelectorAll('button')].find((b) => b.textContent?.trim() === 'Neuer Lauf')!.click());
  await flush();
  return host;
}

const byLabel = (host: ParentNode, label: string) => host.querySelector<HTMLElement>(`[aria-label="${label}"]`)!;
const buttonText = (host: ParentNode, text: string) => [...host.querySelectorAll('button')].find((b) => b.textContent?.trim() === text)!;
const field = (host: ParentNode, label: string) => host.querySelector<HTMLInputElement | HTMLTextAreaElement>(`input[aria-label="${label}"], textarea[aria-label="${label}"]`)!;

async function until(ok: () => boolean, ms = 5000) {
  for (let t = 0; t < ms && !ok(); t += 50) await wait(50);
  expect(ok()).toBe(true);
}

/** Beispiel übernehmen: Ausgangspunkt „Beispiel“ wählen, dann den Eintrag. */
async function example(host: HTMLElement, name: string) {
  await click(host.querySelector<HTMLElement>('[aria-label="Ausgangspunkt"] [data-value="beispiel"]')!);
  await pickValue(host, 'Eintrag des Ausgangspunkts', name);
}

async function click(el: HTMLElement) {
  await act(async () => el.click());
  await flush();
}

afterEach(() => {
  act(() => root?.unmount());
  root = null;
  document.body.innerHTML = '';
});

describe('Balancing-Hilfen', () => {
  it('jedes Feld hat einen Hilfe-Knopf ohne Hilfetext darunter; der Knopf öffnet das Popup', async () => {
    const host = await mount();
    await click(host.querySelector<HTMLElement>('[aria-label="Welt"] [data-value="standard"]')!);
    await click(host.querySelector<HTMLElement>('[aria-label="Seeds"] [data-value="liste"]')!);
    await example(host, 'Mauerkosten abstimmen'); // Abstimmen an: Modus und Budget sichtbar
    for (const key of ['world', 'grade', 'days', 'players', 'bot', 'seeds', 'baseline', 'label', 'mode', 'budget'] as HelpKey[]) {
      const box = host.querySelector(`[data-help="${key}"]`);
      expect(box, key).not.toBeNull();
      expect(box!.textContent, key).not.toContain(FIELD_HELP[key].hint);
      expect(box!.querySelector(`[aria-label="Hilfe: ${FIELD_HELP[key].label}"]`), key).not.toBeNull();
    }
    await click(byLabel(host, 'Hilfe: Varianten'));
    const dialog = document.querySelector('[role="dialog"]')!;
    expect(dialog.textContent).toContain('economy.json:purse.startGold=120');
  });

  it('Beispiele ergeben gültige Läufe und lassen sich übernehmen', async () => {
    for (const e of EXAMPLES) expect(specFromForm(formFromExample(e, EMPTY_FORM)).error, e.name).toBeUndefined();
    const host = await mount();
    await example(host, 'Startgold variieren');
    expect(host.querySelector('[aria-label="Seeds"] [data-value="20"]')!.getAttribute('data-state')).toBe('on');
    expect(field(host, 'Neuer Wert für purse.startGold').value).toBe('120');
    await example(host, 'Mauerkosten abstimmen'.replace('kk', 'k'));
    expect(host.querySelector('table[aria-label="Varianten"] tbody tr')?.textContent).toContain('wall.cost.gold');
    expect(field(host, 'Von für wall.cost.gold').value).toBe('2');
    expect(field(host, 'Bis für wall.cost.gold').value).toBe('6');
    expect(field(host, 'Budget').value).toBe('20');
  });
});

describe('Konfigurationsdateien und Bot je Spieler', () => {
  it('listet die Konfigurationsdateien und zeigt die Beschreibung', async () => {
    const host = await mount();
    await click(buttonText(host, 'Konfigurationsdateien …'));
    await until(() => document.querySelector('[role="dialog"] [aria-label="Konfigurationsdateien"]') !== null);
    const list = byLabel(document, 'Konfigurationsdateien');
    await until(() => list.querySelectorAll('.bal-file').length === 17);
    expect(list.textContent).not.toContain('Beschreibung fehlt');
    expect(list.querySelectorAll('[data-file] button[aria-label^="Hilfe:"]')).toHaveLength(17);
    expect(buttonText(list, 'Werte')).toBeUndefined();
    await click(byLabel(list, 'Hilfe: economy.json'));
    await until(() => document.querySelectorAll('[role="dialog"]').length === 2 && ([...document.querySelectorAll('[role="dialog"]')].at(-1)?.textContent ?? '').includes('Aufbau'));
  });

  // WZ28.12/AC-13 (B-611): Bot je Spieler; die Mischung folgt der Spielerzahl.
  it('wählt je Spieler einen Bot und erzeugt die Mischung', async () => {
    const host = await mount();
    await click(host.querySelector<HTMLElement>('[aria-label="Bot-Zuordnung"] [data-value="je"]')!);
    await pickValue(host, 'Bot Spieler 2', 'passive');
    await click(byLabel(host, 'Ein Spieler mehr'));
    await pickValue(host, 'Bot Spieler 3', 'economy');
    expect(host.querySelectorAll('[role="combobox"][aria-label^="Bot Spieler"]')).toHaveLength(3);
    await click(buttonText(host, 'Lauf starten'));
    const runs = await backend.balanceRuns();
    expect(runs.at(-1)?.spec).toMatchObject({ players: 3, bot: 'saver+passive+economy' });
    expect(resizeMix('saver+passive+economy', 2)).toBe('saver+passive');
    expect(resizeMix('walls', 3)).toBe('walls+saver+saver');
  });

  it('Hilfsfunktionen: Zeile anhängen, Bereich um einen Wert', () => {
    expect(appendLine('a\n\n b ', 'c')).toBe('a\nb\nc');
    expect(appendLine('a', 'a')).toBe('a');
    expect(rangeAround(4)).toBe('3..5');
    expect(rangeAround(0.5)).toBe('0.375..0.625');
    expect(rangeAround(-8)).toBe('-10..-6');
    expect(rangeAround(1)).toBe('1..2');
    expect(rangeAround(0)).toBe('0..1');
  });
});
