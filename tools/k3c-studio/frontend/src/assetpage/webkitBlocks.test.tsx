// @vitest-environment happy-dom
import { FocusBar, type FocusOption } from '@webkit/molecules';
import { FacetRail, TransportBar, type Facet } from '@webkit/organisms';
import { Theme } from '@webkit/templates';
import { act } from 'react';
import { createRoot, type Root } from 'react-dom/client';
import { afterEach, describe, expect, it, vi } from 'vitest';

// B-677 (AC-02, AC-07): die neuen Bausteine der Assets-Seite aus tools/webkit im DOM.
(globalThis as { IS_REACT_ACT_ENVIRONMENT?: boolean }).IS_REACT_ACT_ENVIRONMENT = true;

let root: Root | null = null;
let host: HTMLElement | null = null;

async function mount(node: React.ReactNode): Promise<HTMLElement> {
  host = document.createElement('div');
  document.body.append(host);
  root = createRoot(host);
  await act(async () => root!.render(<Theme>{node}</Theme>));
  return host;
}

const click = (el: Element | null) => act(async () => { (el as HTMLElement).click(); });
const byText = (scope: ParentNode, selector: string, text: string) => [...scope.querySelectorAll(selector)].find((e) => e.textContent?.includes(text)) ?? null;

afterEach(() => {
  act(() => root?.unmount());
  host?.remove();
  document.body.innerHTML = '';
  root = null;
  host = null;
});

describe('FocusBar (B-677)', () => {
  const options: FocusOption[] = [
    { id: 'alles', label: 'Alles', count: 12 },
    { id: 'luecken', label: 'Lücken', count: 3 },
    { id: 'eigen:test', label: 'Mein Chip', count: 0, custom: true },
  ];

  it('zeigt die Chips mit Zähler und markiert den gewählten', async () => {
    const h = await mount(<FocusBar label="Fokus" options={options} value="luecken" onChange={() => undefined} />);
    expect(h.querySelector('[role="group"]')?.getAttribute('aria-label')).toBe('Fokus');
    const lue = byText(h, 'button', 'Lücken')!;
    expect(lue.textContent).toContain('3');
    expect(lue.getAttribute('aria-pressed')).toBe('true');
    expect(byText(h, 'button', 'Alles')!.getAttribute('aria-pressed')).toBe('false');
    expect(byText(h, 'button', 'Mein Chip')!.getAttribute('aria-pressed')).toBe('false');
  });

  it('markiert keinen Chip, wenn der Wert zu keinem passt', async () => {
    const h = await mount(<FocusBar label="Fokus" options={options} value="" onChange={() => undefined} />);
    expect([...h.querySelectorAll('[aria-pressed="true"]')]).toHaveLength(0);
  });

  it('meldet den Klick auf einen Chip mit dessen ID', async () => {
    const onChange = vi.fn();
    const h = await mount(<FocusBar label="Fokus" options={options} value="alles" onChange={onChange} />);
    await click(byText(h, 'button', 'Lücken'));
    expect(onChange).toHaveBeenCalledExactlyOnceWith('luecken');
  });

  it('zeigt das ✕ nur bei eigenen Chips und meldet das Entfernen, ohne den Chip zu wählen', async () => {
    const onChange = vi.fn();
    const onRemove = vi.fn();
    const h = await mount(<FocusBar label="Fokus" options={options} value="alles" onChange={onChange} onRemove={onRemove} />);
    expect(h.querySelectorAll('[aria-label$="entfernen"]')).toHaveLength(1);
    await click(h.querySelector('[aria-label="Mein Chip entfernen"]'));
    expect(onRemove).toHaveBeenCalledExactlyOnceWith('eigen:test');
    expect(onChange).not.toHaveBeenCalled();
  });

  it('zeigt ohne onRemove kein ✕, auch nicht bei eigenen Chips', async () => {
    const h = await mount(<FocusBar label="Fokus" options={options} value="alles" onChange={() => undefined} />);
    expect(h.querySelectorAll('[aria-label$="entfernen"]')).toHaveLength(0);
  });

  it('zeigt „Ansicht merken“ nur mit Handler und ruft ihn auf', async () => {
    const onSave = vi.fn();
    const h = await mount(<FocusBar label="Fokus" options={options} value="alles" onChange={() => undefined} onSave={onSave} />);
    await click(byText(h, 'button', 'Ansicht merken'));
    expect(onSave).toHaveBeenCalledOnce();
    await act(async () => root!.render(<Theme><FocusBar label="Fokus" options={options} value="alles" onChange={() => undefined} /></Theme>));
    expect(byText(h, 'button', 'Ansicht merken')).toBeNull();
  });

  it('nimmt eine eigene Beschriftung für den Merken-Knopf', async () => {
    const h = await mount(<FocusBar label="Fokus" options={options} value="" onChange={() => undefined} onSave={() => undefined} saveLabel="Merken" />);
    expect(byText(h, 'button', 'Merken')).not.toBeNull();
  });
});

describe('FacetRail (B-677)', () => {
  const facets: Facet[] = [
    { id: 'art', title: 'Art', options: [{ value: 'grafik', label: 'Grafik', count: 5 }, { value: 'sound', label: 'Sound', count: 2 }] },
    { id: 'status', title: 'Status', options: [{ value: 'luecke', label: 'Lücke' }] },
  ];

  it('zeigt je Gruppe ihre Optionen, den Zähler im Label und markiert die Auswahl', async () => {
    const h = await mount(<FacetRail facets={facets} selected={{ art: ['sound'] }} onToggle={() => undefined} onClear={() => undefined} />);
    expect([...h.querySelectorAll('[role="group"]')].map((g) => g.getAttribute('aria-label'))).toEqual(['Art', 'Status']);
    const labels = [...h.querySelectorAll('label')].map((l) => l.textContent);
    expect(labels).toEqual(['Grafik (5)', 'Sound (2)', 'Lücke']);
    const boxes = [...h.querySelectorAll('[role="checkbox"]')];
    expect(boxes.map((c) => c.getAttribute('aria-checked'))).toEqual(['false', 'true', 'false']);
  });

  it('meldet das Umschalten mit Gruppe, Wert und neuem Zustand', async () => {
    const onToggle = vi.fn();
    const h = await mount(<FacetRail facets={facets} selected={{ art: ['sound'] }} onToggle={onToggle} onClear={() => undefined} />);
    const boxes = [...h.querySelectorAll('[role="checkbox"]')];
    await click(boxes[0]!);
    expect(onToggle).toHaveBeenLastCalledWith('art', 'grafik', true);
    await click(boxes[1]!);
    expect(onToggle).toHaveBeenLastCalledWith('art', 'sound', false);
  });

  it('zeigt „Facetten zurücksetzen“ nur bei einer Auswahl und ruft onClear auf', async () => {
    const onClear = vi.fn();
    const h = await mount(<FacetRail facets={facets} selected={{}} onToggle={() => undefined} onClear={onClear} />);
    expect(byText(h, 'button', 'Facetten zurücksetzen')).toBeNull();
    await act(async () => root!.render(<Theme><FacetRail facets={facets} selected={{ status: ['luecke'] }} onToggle={() => undefined} onClear={onClear} /></Theme>));
    await click(byText(h, 'button', 'Facetten zurücksetzen'));
    expect(onClear).toHaveBeenCalledOnce();
  });

  it('zeigt „Facetten zurücksetzen“ nicht bei leeren Listen der Gruppen', async () => {
    const h = await mount(<FacetRail facets={facets} selected={{ art: [], status: [] }} onToggle={() => undefined} onClear={() => undefined} />);
    expect(byText(h, 'button', 'Facetten zurücksetzen')).toBeNull();
  });
});

describe('TransportBar (B-677/AC-07)', () => {
  const handlers = () => ({ onStop: vi.fn(), onFade: vi.fn(), onVolume: vi.fn() });

  it('ruft Stopp, Überblenden, Leiser und Lauter mit den richtigen Argumenten auf', async () => {
    const hs = handlers();
    const h = await mount(<TransportBar playing="" volume={50} {...hs} />);
    await click(byText(h, 'button', 'Stopp'));
    await click(byText(h, 'button', 'Überblenden'));
    await click(h.querySelector('[aria-label="Leiser"]'));
    await click(h.querySelector('[aria-label="Lauter"]'));
    expect(hs.onStop).toHaveBeenCalledOnce();
    expect(hs.onFade).toHaveBeenCalledOnce();
    expect(hs.onVolume.mock.calls).toEqual([[-1], [1]]);
  });

  it('zeigt Stille und die Lautstärke, wenn nichts spielt', async () => {
    const h = await mount(<TransportBar playing="" volume={70} {...handlers()} />);
    expect(h.querySelector('[role="status"]')?.textContent).toBe('Stille · Lautstärke 70 %');
  });

  it('zeigt, was gerade spielt', async () => {
    const h = await mount(<TransportBar playing="music:tag · probe-a.ogg" volume={30} {...handlers()} />);
    expect(h.querySelector('[role="status"]')?.textContent).toBe('spielt: music:tag · probe-a.ogg · Lautstärke 30 %');
  });

  it('zeigt eine Meldung nur, wenn es eine gibt', async () => {
    const h = await mount(<TransportBar playing="" volume={50} status="Datei fehlt" {...handlers()} />);
    expect(h.textContent).toContain('Datei fehlt');
    await act(async () => root!.render(<Theme><TransportBar playing="" volume={50} {...handlers()} /></Theme>));
    expect(h.textContent).not.toContain('Datei fehlt');
  });
});
