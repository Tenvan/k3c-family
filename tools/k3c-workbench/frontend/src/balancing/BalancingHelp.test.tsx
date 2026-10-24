// @vitest-environment happy-dom
import { Theme } from '@radix-ui/themes';
import { act } from 'react';
import { createRoot, type Root } from 'react-dom/client';
import { afterEach, describe, expect, it } from 'vitest';
import { pickValue } from '@webkit/testing/select';
import { backend } from '../api';
import { BalancingPage } from './BalancingPage';
import { appendLine, EXAMPLES, FIELD_HELP, formFromExample, rangeAround, resizeMix, SELECT_HELP, type HelpKey } from './help';
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
  return host;
}

const byLabel = (host: ParentNode, label: string) => host.querySelector<HTMLElement>(`[aria-label="${label}"]`)!;
const buttonText = (host: ParentNode, text: string) => [...host.querySelectorAll('button')].find((b) => b.textContent?.trim() === text)!;
const field = (host: ParentNode, label: string) => host.querySelector<HTMLInputElement | HTMLTextAreaElement>(`input[aria-label="${label}"], textarea[aria-label="${label}"]`)!;

async function until(ok: () => boolean, ms = 5000) {
  for (let t = 0; t < ms && !ok(); t += 50) await wait(50);
  expect(ok()).toBe(true);
}

/** Beispiel übernehmen: Popup „Beispiele“ öffnen, Knopf des Beispiels klicken. */
async function example(host: HTMLElement, name: string) {
  await click(buttonText(host, 'Beispiele …'));
  await click(byLabel(document, `Beispiel übernehmen: ${name}`));
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
  it('jedes Feld hat Hilfe-Knopf, freie Felder dazu einen Hilfetext; der Knopf öffnet das Popup', async () => {
    const host = await mount();
    await example(host, 'Mauerkosten abstimmen'); // Abstimmen an: alle Felder sichtbar
    for (const [key, h] of Object.entries(FIELD_HELP)) {
      const box = host.querySelector(`[data-help="${key}"]`);
      expect(box, key).not.toBeNull();
      if (SELECT_HELP.includes(key as HelpKey)) expect(box!.textContent, key).not.toContain(h.hint);
      else expect(box!.textContent, key).toContain(h.hint);
      expect(box!.querySelector(`[aria-label="Hilfe: ${h.label}"]`), key).not.toBeNull();
    }
    await click(byLabel(host, 'Hilfe: Varianten'));
    const dialog = document.querySelector('[role="dialog"]')!;
    expect(dialog.textContent).toContain('economy.json:purse.startGold=120');
  });

  it('Beispiele übernehmen die Konfiguration und ergeben gültige Läufe', async () => {
    for (const e of EXAMPLES) expect(specFromForm(formFromExample(e, EMPTY_FORM)).error, e.name).toBeUndefined();
    const host = await mount();
    await example(host, 'Startgold variieren');
    expect(field(host, 'Seeds').value).toBe('20');
    expect(field(host, 'Varianten').value).toBe('economy.json:purse.startGold=120');
    await example(host, 'Mauerkosten abstimmen');
    expect(field(host, 'Varianten').value).toBe('');
    expect(field(host, 'Parameter').value).toBe('buildings.json:wall.cost.gold=2..6');
  });

});

describe('Konfigurationsdateien und Bot je Spieler', () => {
  it('listet die Konfigurationsdateien, zeigt die Beschreibung und übernimmt Werte', async () => {
    const host = await mount();
    await click(buttonText(host, 'Konfigurationsdateien …'));
    await until(() => document.querySelector('[role="dialog"] [aria-label="Konfigurationsdateien"]') !== null);
    const list = byLabel(document, 'Konfigurationsdateien');
    await until(() => list.querySelectorAll('.bal-file').length === 17);
    expect(list.textContent).not.toContain('Beschreibung fehlt');
    expect(list.querySelectorAll('[data-file] button[aria-label^="Hilfe:"]')).toHaveLength(17);
    await click(byLabel(list, 'Hilfe: economy.json'));
    await until(() => document.querySelectorAll('[role="dialog"]').length === 2 && ([...document.querySelectorAll('[role="dialog"]')].at(-1)?.textContent ?? '').includes('Aufbau'));
    const docDialog = [...document.querySelectorAll<HTMLElement>('[role="dialog"]')].at(-1)!;
    await act(async () => docDialog.dispatchEvent(new KeyboardEvent('keydown', { key: 'Escape', bubbles: true })));
    await flush();
    const row = list.querySelector('[data-file="economy.json"]') as HTMLElement;
    expect(buttonText(list.querySelector('[data-file="guards.json"]')!, 'Werte')).toBeUndefined();
    await click(buttonText(row, 'Werte'));
    await until(() => !!buttonText(row, 'Als Variante'));
    await click(buttonText(row, 'Als Variante'));
    await click(buttonText(row, 'Als Variante')); // keine doppelte Zeile
    expect(field(host, 'Varianten').value).toMatch(/^economy\.json:[\w.]+=-?[\d.]+$/);
    expect(row.querySelector('[role="status"]')?.textContent).toMatch(/^Übernommen: economy\.json:/);
    await click(buttonText(row, 'Als Parameter'));
    expect(field(host, 'Parameter').value).toMatch(/^economy\.json:[\w.]+=-?[\d.]+\.\.-?[\d.]+$/);
  });

  // WZ28.12/AC-13 (B-611): Bot je Spieler; die Mischung folgt der Spielerzahl.
  it('wählt je Spieler einen Bot und erzeugt die Mischung', async () => {
    const host = await mount();
    await pickValue(host, 'Bot', 'je-spieler');
    await pickValue(host, 'Bot Spieler 2', 'passive');
    await pickValue(host, 'Spieler', '3');
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
