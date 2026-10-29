// @vitest-environment happy-dom
import { Theme } from '@radix-ui/themes';
import { act, useState } from 'react';
import { createRoot, type Root } from 'react-dom/client';
import { afterEach, describe, expect, it } from 'vitest';
import { BalancingLayout, type BalancingView } from './BalancingLayout';
import { BalancingPage } from './BalancingPage';

// B-659/AC-01 und AC-02 (WZ32.2): Werkzeugleiste, Liste und Detail sind getrennte Bereiche mit eigenen Scroll-Containern;
// in der schmalen Ansicht wechselt ein Klick in der Liste auf das Detail, „Liste“ zurück, die Auswahl bleibt.
// Die Breiten (1.100 und 760 px) entscheidet die Container Query in styles/balancing.css (BalancingCss.test.ts), hier
// steht der Zustand `data-view`, der sie speist.

(globalThis as { IS_REACT_ACT_ENVIRONMENT?: boolean }).IS_REACT_ACT_ENVIRONMENT = true;

let root: Root | null = null;
const wait = (ms: number) => act(async () => new Promise((r) => setTimeout(r, ms)));
async function flush() {
  for (let i = 0; i < 5; i++) await wait(0);
}

async function mount(node: React.ReactNode): Promise<HTMLElement> {
  const host = document.createElement('div');
  document.body.appendChild(host);
  root = createRoot(host);
  act(() => root!.render(<Theme>{node}</Theme>));
  await flush();
  return host;
}

const byLabel = (host: ParentNode, label: string) => host.querySelector<HTMLElement>(`[aria-label="${label}"]`)!;
const button = (host: ParentNode, text: string) => [...host.querySelectorAll('button')].find((b) => b.textContent?.trim() === text)!;
const body = (host: ParentNode) => host.querySelector<HTMLElement>('.bal-body')!;
async function click(el: HTMLElement) {
  await act(async () => el.click());
  await flush();
}

afterEach(() => {
  act(() => root?.unmount());
  root = null;
  document.body.innerHTML = '';
});

/** Rahmen mit eigenem Ansichtszustand, wie ihn die Seite hält. */
function Harness() {
  const [view, setView] = useState<BalancingView>('list');
  return (
    <BalancingLayout
      toolbar={<span>Werkzeuge</span>} notice={<p role="alert">Hinweis</p>}
      list={<button type="button" onClick={() => setView('detail')}>Eintrag</button>}
      detail={<span>Inhalt</span>} view={view} onBack={() => setView('list')}
    />
  );
}

describe('Rahmen der Balancing-Seite', () => {
  it('trennt Werkzeugleiste, Liste und Detail; Liste und Detail haben je einen eigenen Scroll-Container', async () => {
    const host = await mount(<Harness />);
    const toolbar = byLabel(host, 'Werkzeugleiste');
    const list = byLabel(host, 'Läufe und Berichte');
    const detail = byLabel(host, 'Detail');
    expect(toolbar.getAttribute('role')).toBe('toolbar');
    expect(toolbar.textContent).toBe('Werkzeuge');
    const scrolls = [...host.querySelectorAll<HTMLElement>('.bal-scroll')];
    expect(scrolls.map((s) => s.dataset.pane)).toEqual(['list', 'detail']);
    expect(list.contains(scrolls[0])).toBe(true);
    expect(detail.contains(scrolls[1])).toBe(true);
    expect(scrolls[0].textContent).toContain('Eintrag');
    expect(scrolls[1].textContent).toBe('Inhalt');
    // Die Werkzeugleiste und der Hinweis stehen außerhalb der Scroll-Container, Seite und Rahmen scrollen nicht.
    expect(scrolls.some((s) => s.contains(toolbar) || s.contains(host.querySelector('[role="alert"]')))).toBe(false);
    expect(host.querySelector('.bal-wrap')!.classList.contains('bal-scroll')).toBe(false);
    expect(host.querySelector('.bal-page')!.classList.contains('bal-scroll')).toBe(false);
    expect(host.querySelector('[role="separator"]')).not.toBeNull();
    expect(body(host).style.getPropertyValue('--bal-split')).toBe('30%');
  });

  it('wechselt mit der Auswahl auf das Detail und mit „Liste“ zurück', async () => {
    const host = await mount(<Harness />);
    expect(body(host).dataset.view).toBe('list');
    await click(button(host, 'Eintrag'));
    expect(body(host).dataset.view).toBe('detail');
    await click(button(host, 'Liste'));
    expect(body(host).dataset.view).toBe('list');
  });
});

describe('Balancing-Seite im Rahmen', () => {
  const row = (host: HTMLElement, name: string) => [...byLabel(host, 'Läufe und Berichte').querySelectorAll<HTMLElement>('.bal-item')].find((r) => r.textContent?.includes(name))!;

  it('zeigt ohne Auswahl den Hinweis mit „Neuer Lauf“ und in der Werkzeugleiste die Warteschlange', async () => {
    const host = await mount(<BalancingPage />);
    expect(body(host).dataset.view).toBe('list');
    const toolbar = byLabel(host, 'Werkzeugleiste');
    expect(button(toolbar, 'Neuer Lauf')).toBeDefined();
    expect(byLabel(toolbar, 'Warteschlange').textContent).toBe('0 läuft0 wartet');
    const detail = byLabel(host, 'Detail');
    expect(detail.textContent).toContain('Nichts gewählt');
    expect(byLabel(host, 'Läufe und Berichte').querySelectorAll('.bal-item').length).toBeGreaterThan(0);
    expect(byLabel(host, 'Konfiguration')).toBeNull();
    await click(button(detail, 'Neuer Lauf'));
    expect(byLabel(byLabel(host, 'Detail'), 'Konfiguration')).not.toBeNull();
    expect(body(host).dataset.view).toBe('detail');
  });

  it('öffnet einen Bericht im Detail, „Liste“ führt zurück und die Auswahl bleibt', async () => {
    const host = await mount(<BalancingPage />);
    await click(row(host, 'balance-1.json').querySelector<HTMLElement>('.bal-item-name')!);
    expect(body(host).dataset.view).toBe('detail');
    expect(byLabel(byLabel(host, 'Detail'), 'Bericht')).not.toBeNull();
    await click(button(byLabel(host, 'Detail'), 'Liste'));
    expect(body(host).dataset.view).toBe('list');
    expect(row(host, 'balance-1.json').dataset.on).toBe('true');
    expect(byLabel(byLabel(host, 'Detail'), 'Bericht')).not.toBeNull(); // der Bericht bleibt im Detail, nur verborgen
  });

  it('„Neuer Lauf“ ersetzt den Bericht durch das Formular, ein Bericht in der Liste bringt ihn zurück', async () => {
    const host = await mount(<BalancingPage />);
    await click(row(host, 'balance-1.json').querySelector<HTMLElement>('.bal-item-name')!);
    await click(button(byLabel(host, 'Werkzeugleiste'), 'Neuer Lauf'));
    expect(byLabel(byLabel(host, 'Detail'), 'Konfiguration')).not.toBeNull();
    expect(byLabel(host, 'Bericht')).toBeNull();
    expect(row(host, 'balance-1.json').dataset.on).toBe('true'); // die Auswahl bleibt
    await click(row(host, 'balance-1.json').querySelector<HTMLElement>('.bal-item-name')!);
    expect(byLabel(host, 'Konfiguration')).toBeNull();
    expect(byLabel(byLabel(host, 'Detail'), 'Bericht')).not.toBeNull();
  });

  it('„Lauf starten“ springt in der schmalen Ansicht zur Liste mit dem Fortschritt', async () => {
    const host = await mount(<BalancingPage />);
    await click(button(byLabel(host, 'Werkzeugleiste'), 'Neuer Lauf'));
    expect(body(host).dataset.view).toBe('detail');
    await click(button(byLabel(host, 'Konfiguration'), 'Lauf starten'));
    expect(body(host).dataset.view).toBe('list');
    expect(byLabel(host, 'Läufe und Berichte').querySelectorAll('.bal-item[data-id^="run:"]')).toHaveLength(1);
    expect(byLabel(byLabel(host, 'Werkzeugleiste'), 'Warteschlange').textContent).toMatch(/[01] läuft[01] wartet/);
  });
});
