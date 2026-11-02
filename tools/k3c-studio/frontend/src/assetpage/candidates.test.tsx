// @vitest-environment happy-dom
import { Theme } from '@webkit/templates';
import { act } from 'react';
import { createRoot, type Root } from 'react-dom/client';
import { afterEach, beforeEach, describe, expect, it, vi } from 'vitest';
import { backend, STALE, type AssetCandidate, type AssetSlot } from '../api';
import { resetMockSources, setMockSourcesOffline } from '../api/mockSources';
import { canAssign, confirmText, hitId, markTone, needsConfirm, originTip, previewKind, rowError, sourceLine, stageText, stageTone } from './candidateLogic';
import { Inspector } from './Inspector';
import { playFile, playOn } from './transport';

// B-677 (AC-06, Inspektor): Kopf, Reiter Belegung und Kandidaten, Details zum Aufklappen im DOM mit dem Mock; Abspielen läuft über transport.ts (hier ersetzt).
vi.mock('./transport', () => ({ playOn: vi.fn(async () => true), playFile: vi.fn(async () => true) }));
(globalThis as { IS_REACT_ACT_ENVIRONMENT?: boolean }).IS_REACT_ACT_ENVIRONMENT = true;

let root: Root | null = null;
let host: HTMLElement | null = null;
const wait = (ms: number) => act(async () => { await new Promise((r) => setTimeout(r, ms)); });
const flush = async () => { await wait(0); await wait(0); await wait(0); };
const click = (el: Element | null | undefined) => act(async () => { (el as HTMLElement).click(); });
const byText = (selector: string, text: string) => [...(host?.querySelectorAll(selector) ?? [])].find((e) => e.textContent?.includes(text)) ?? null;
const row = (id: string) => host!.querySelector<HTMLElement>(`[data-cand="${id}"]`)!;
const rowButton = (id: string, text: string) => [...row(id).querySelectorAll('button')].find((b) => b.textContent === text) ?? null;

/** Tippt in das Suchfeld und wartet die Entprellung (300 ms) und die Antwort ab. */
async function typeQuery(value: string): Promise<void> {
  const input = host!.querySelector<HTMLInputElement>('input[aria-label="Katalog durchsuchen"]')!;
  await act(async () => {
    Object.getOwnPropertyDescriptor(HTMLInputElement.prototype, 'value')!.set!.call(input, value);
    input.dispatchEvent(new Event('input', { bubbles: true }));
  });
  await wait(350);
  await flush();
}

async function slotOf(id: string): Promise<AssetSlot> {
  const found = (await backend.assetsList()).slots.find((s) => s.id === id);
  if (!found) throw new Error(`Slot ${id} fehlt im Mock`);
  return found;
}

/** Inspektor eines Slots, optional gleich auf dem Reiter Kandidaten. */
async function mount(id: string, tab = 'belegung', props: Partial<Parameters<typeof Inspector>[0]> = {}): Promise<{ onReload: ReturnType<typeof vi.fn> }> {
  localStorage.setItem('k3c-studio.assets.inspector', tab);
  const onReload = vi.fn();
  const slot = await slotOf(id);
  host = document.createElement('div');
  document.body.append(host);
  root = createRoot(host);
  await act(async () => root!.render(
    <Theme><Inspector slot={slot} used={new Set()} onSave={props.onSave ?? (async () => true)} onReload={onReload} /></Theme>,
  ));
  await flush();
  return { onReload };
}

beforeEach(() => { resetMockSources(); localStorage.clear(); vi.mocked(playOn).mockClear(); vi.mocked(playFile).mockClear(); });
afterEach(() => {
  act(() => root?.unmount());
  host?.remove();
  document.body.innerHTML = '';
  vi.restoreAllMocks();
  root = null;
  host = null;
});

describe('candidateLogic', () => {
  const base: AssetCandidate = {
    id: 'sug:q05', stufe: 'netz', praef: 0, titel: 'Taverne', datei: 'q05.png', quelle: 'Kenney', urheber: 'Kenney', lizenz: 'CC0 1.0', marke: 'frei', groesse: 212_000,
    vorschau: '', herkunft: ['vorschlag', 'recherche'], zuordenbar: true,
  };

  it('Stufe: Präf. n vor Spiel, Vorrat und Netz, mit Ton', () => {
    expect(stageText({ stufe: 'spiel', praef: 2 })).toBe('Präf. 2');
    expect([stageText(base), stageText({ stufe: 'vorrat', praef: 0 }), stageText({ stufe: 'spiel', praef: 0 })]).toEqual(['Netz', 'Vorrat', 'Spiel']);
    expect(new Set([stageTone({ stufe: 'spiel', praef: 1 }), stageTone(base), stageTone({ stufe: 'vorrat', praef: 0 }), stageTone({ stufe: 'spiel', praef: 0 })]).size).toBe(4);
  });

  it('Lizenz-Marke: frei ok, SA und prüfen warn, gesperrt error', () => {
    expect(['frei', 'sa', 'pruefen', 'gesperrt'].map((m) => markTone(m as AssetCandidate['marke']))).toEqual(['ok', 'warn', 'warn', 'error']);
  });

  it('Bestätigung nur bei Netz, Text mit Größe, Lizenz und Ziel', () => {
    expect(needsConfirm(base)).toBe(true);
    expect(needsConfirm({ stufe: 'vorrat' })).toBe(false);
    expect(confirmText(base, 'tavern')).toBe('212 kB · CC0 1.0 · in tavern');
  });

  it('Zuordnen nur für zuordenbare, nicht zugeordnete Kandidaten', () => {
    expect(canAssign(base)).toBe(true);
    expect(canAssign({ ...base, praef: 1 })).toBe(false);
    expect(canAssign({ ...base, zuordenbar: false })).toBe(false);
  });

  it('Vorschau: Endung vor Art, ohne Adresse keine', () => {
    expect(previewKind({ vorschau: '' }, 'grafik')).toBe('keine');
    expect(previewKind({ vorschau: 'a/b.ogg' }, 'grafik')).toBe('ton');
    expect(previewKind({ vorschau: 'a/b.png?x=1' }, 'sound')).toBe('bild');
    expect(previewKind({ vorschau: 'a/b' }, 'sound')).toBe('ton');
  });

  it('Texte: Herkünfte im Tooltip, Quelle · Urheber, Treffer-ID ohne Präfix', () => {
    expect(originTip(base)).toContain('Vorschlag + Recherche');
    expect(sourceLine(base)).toBe('Kenney · Kenney');
    expect(sourceLine({ quelle: 'OGA' })).toBe('OGA');
    expect(hitId('hit:abc')).toBe('abc');
  });

  it('Fehler: Stale bekommt den Hinweis Register geändert', () => {
    expect(rowError(new Error(STALE))).toMatchObject({ stale: true });
    expect(rowError(new Error(STALE)).text).toContain('Register geändert');
    expect(rowError('Download scheitert')).toEqual({ text: 'Download scheitert', stale: false });
  });
});

describe('Inspektor', () => {
  it('hat nur die Reiter Belegung und Kandidaten und merkt den gewählten', async () => {
    await mount('farm');
    const tabs = [...host!.querySelectorAll('[role="tab"]')].map((t) => t.textContent?.replace(/^(.+)\1$/, '$1')); // Radix doppelt den Text (fette Breite)
    expect(tabs).toEqual(['Belegung', 'Kandidaten']);
    const pick = (name: string) => act(async () => { byText('[role="tab"]', name)!.dispatchEvent(new MouseEvent('mousedown', { bubbles: true, button: 0 })); }); // Radix wählt bei mousedown
    await pick('Kandidaten');
    await flush();
    expect(localStorage.getItem('k3c-studio.assets.inspector')).toContain('kandidaten');
    expect(host!.querySelector('.ap-cands')).not.toBeNull();
  });

  it('ein gemerkter alter Reiter Details fällt auf Belegung zurück', async () => {
    await mount('farm', 'details');
    expect(host!.querySelector('[role="tab"][aria-selected="true"]')?.textContent).toMatch(/^Belegung/);
  });

});

describe('Inspektor: Kopf und Details', () => {
  it('Kopf: Slot-Name, Status und „Datei zuordnen …“ öffnet Kandidaten mit Fokus in der Suche', async () => {
    await mount('farm');
    const head = host!.querySelector('.ap-head-slot')!;
    expect(head.textContent).toContain('farm');
    expect(head.querySelector('.wk-status')).not.toBeNull();
    await click([...head.querySelectorAll('button')].find((b) => b.textContent?.startsWith('Datei zuordnen')));
    await flush();
    expect(host!.querySelector('[role="tab"][aria-selected="true"]')?.textContent).toMatch(/^Kandidaten/);
    expect(document.activeElement?.getAttribute('aria-label')).toBe('Katalog durchsuchen');
  });

  it('Details klappt auf und zu und merkt sich den Zustand', async () => {
    await mount('mount:horse');
    const details = host!.querySelector<HTMLDetailsElement>('details.ap-details')!;
    expect(details.open).toBe(false);
    expect(host!.querySelector('.as-info')).toBeNull();
    await act(async () => { details.open = true; details.dispatchEvent(new Event('toggle')); });
    await flush();
    expect(host!.querySelector('.as-info')).not.toBeNull();
    expect(host!.textContent).toContain('Pferd');
    expect(localStorage.getItem('k3c-studio.assets.details')).toContain('1');
    await act(async () => { details.open = false; details.dispatchEvent(new Event('toggle')); });
    expect(host!.querySelector('.as-info')).toBeNull();
    expect(localStorage.getItem('k3c-studio.assets.details')).toContain('0');
  });
});

describe('Inspektor: Belegung', () => {
  it('Belegung: Präferenzen, Hinweis, bei Grafik Ausschnitt (Mini-Szene steht auf der Bühne), kein ▶', async () => {
    await mount('farm');
    expect(host!.textContent).toContain('Präferenz 1 ist im Spiel aktiv');
    expect(host!.querySelectorAll('.ap-choice').length).toBe(2);
    expect(host!.querySelector('.as-cut')).not.toBeNull();
    expect(host!.querySelector('.as-scene')).toBeNull();
    expect(host!.querySelector('[aria-label="Abspielen"]')).toBeNull();
  });

  it('Belegung Sound: ▶ je Zeile, ▶ fern nur bei Atlas-Ton, beides ruft playOn', async () => {
    await mount('music:kampf');
    const plays = [...host!.querySelectorAll('[aria-label="Abspielen"]')];
    expect(plays.length).toBe(2);
    expect(host!.querySelector('[aria-label="Abspielen fern"]')).toBeNull();
    await click(plays[1]);
    expect(playOn).toHaveBeenCalledWith(expect.objectContaining({ id: 'music:kampf' }), { datei: 'audio/probe-a.ogg' });
    document.body.innerHTML = '';
    act(() => root?.unmount());
    await mount('sfx:coin');
    await click(host!.querySelector('[aria-label="Abspielen fern"]'));
    expect(playOn).toHaveBeenLastCalledWith(expect.objectContaining({ id: 'sfx:coin' }), expect.objectContaining({ sprite: 'coin' }), true);
  });

  it('Belegung: Lücke nennt den Weg zu den Kandidaten', async () => {
    await mount('music:abend');
    expect(host!.textContent).toContain('Lücke');
  });

});

describe('Kandidaten: Liste und Zuordnen', () => {
  it('eine Liste mit Stufe, Lizenz-Marke und Größe; Zuordnungen tragen nur die Präf.-Marke', async () => {
    await mount('farm', 'kandidaten');
    const first = row('wahl:grafik/gothicvania-town/props-einzeln/well.png');
    expect(first.textContent).toContain('Präf. 1');
    expect([...first.querySelectorAll('button')].some((b) => b.textContent === 'Zuordnen')).toBe(false);
    await typeQuery('crate'); // Spiel-Kandidat aus dem Katalog: Stufe Spiel, Lizenz-Marke CC0/BY, sofort zuordenbar
    const cat = host!.querySelector('[data-cand^="cat:"]')!;
    expect(cat.textContent).toContain('Spiel');
    expect(cat.textContent).toContain('CC0/BY');
    expect([...cat.querySelectorAll('button')].some((b) => b.textContent === 'Zuordnen' && !(b as HTMLButtonElement).disabled)).toBe(true);
  });

  it('Spiel-Kandidat wird ohne Bestätigung zugeordnet', async () => {
    const { onReload } = await mount('farm', 'kandidaten');
    await typeQuery('crate');
    const assign = vi.spyOn(backend, 'assetCandidateAssign');
    await click(rowButton('cat:grafik/gothicvania-town/props-einzeln/crate.png', 'Zuordnen'));
    await flush();
    expect(assign).toHaveBeenCalledWith('farm', 'cat:grafik/gothicvania-town/props-einzeln/crate.png', expect.any(String), 0);
    expect(onReload).toHaveBeenCalled();
  });

  it('Netz: Zuordnen bestätigt in der Zeile (Größe, Lizenz, Ziel), erst Bestätigen ordnet zu und lädt neu', async () => {
    const { onReload } = await mount('tavern', 'kandidaten');
    const assign = vi.spyOn(backend, 'assetCandidateAssign');
    await click(rowButton('sug:q05', 'Zuordnen'));
    expect(assign).not.toHaveBeenCalled();
    expect(row('sug:q05').textContent).toMatch(/25 kB · CC0 1\.0 · in tavern/);
    expect(host!.querySelector('[role="dialog"]')).toBeNull();
    await click(rowButton('sug:q05', 'Bestätigen'));
    await wait(20);
    await flush();
    expect(assign).toHaveBeenCalledWith('tavern', 'sug:q05', expect.any(String), 24_500);
    expect(onReload).toHaveBeenCalled();
  });

  it('Abbrechen schließt die Bestätigung ohne Aufruf', async () => {
    await mount('tavern', 'kandidaten');
    const assign = vi.spyOn(backend, 'assetCandidateAssign');
    await click(rowButton('sug:q05', 'Zuordnen'));
    await click(rowButton('sug:q05', 'Abbrechen'));
    expect(assign).not.toHaveBeenCalled();
    expect(row('sug:q05').textContent).not.toContain('in tavern');
  });

});

describe('Kandidaten: Fehler und Sperren', () => {
  it('Fehler steht in der Zeile, Erneut versuchen wiederholt dieselbe Zuordnung', async () => {
    await mount('tavern', 'kandidaten');
    const assign = vi.spyOn(backend, 'assetCandidateAssign').mockRejectedValueOnce(new Error('Download fehlgeschlagen'));
    await click(rowButton('sug:q05', 'Zuordnen'));
    await click(rowButton('sug:q05', 'Bestätigen'));
    await flush();
    expect(row('sug:q05').querySelector('[role="alert"]')?.textContent).toContain('Download fehlgeschlagen');
    await click(rowButton('sug:q05', 'Erneut versuchen'));
    await wait(20);
    await flush();
    expect(assign).toHaveBeenCalledTimes(2);
  });

  it('Stale: Hinweis Register geändert in der Zeile, die Liste lädt neu', async () => {
    await mount('tavern', 'kandidaten');
    vi.spyOn(backend, 'assetCandidateAssign').mockRejectedValueOnce(new Error(STALE));
    const list = vi.spyOn(backend, 'assetCandidates');
    await click(rowButton('sug:q05', 'Zuordnen'));
    await click(rowButton('sug:q05', 'Bestätigen'));
    await flush();
    expect(row('sug:q05').querySelector('[role="alert"]')?.textContent).toContain('Register geändert');
    expect(list).toHaveBeenCalled();
    expect(rowButton('sug:q05', 'Erneut versuchen')).not.toBeNull();
  });

  it('Lizenz gesperrt: Zuordnen deaktiviert', async () => {
    await mount('portal', 'kandidaten');
    expect(row('sug:q07').textContent).toContain('gesperrt');
    expect((rowButton('sug:q07', 'Zuordnen') as HTMLButtonElement).disabled).toBe(true);
  });

  it('Offline: Hinweis oben, Netz-Kandidat sichtbar, Zuordnen gesperrt', async () => {
    setMockSourcesOffline(true);
    await mount('tavern', 'kandidaten');
    expect(host!.textContent).toContain('Offline');
    expect(host!.textContent).toContain('Zuordnen ist gesperrt');
    expect((rowButton('sug:q06', 'Zuordnen') as HTMLButtonElement).disabled).toBe(true);
  });

});

describe('Kandidaten: Suche, Treffer und Vorschau', () => {
  it('Im Netz suchen: Hinweis mit neuen Treffern, Liste lädt neu; Lizenz bestätigen für Treffer mit Lizenz prüfen', async () => {
    await mount('music:abend', 'kandidaten');
    const research = vi.spyOn(backend, 'assetsResearch');
    await click(byText('button', 'Im Netz suchen'));
    await wait(450);
    await flush();
    expect(research).toHaveBeenCalledWith('music:abend', '');
    expect(host!.querySelector('[data-testid="research-note"]')?.textContent).toMatch(/\d+ neue Treffer/);
    const checkRow = [...host!.querySelectorAll('[data-cand^="hit:"]')].find((r) => [...r.querySelectorAll('button')].some((b) => b.textContent === 'Lizenz bestätigen'));
    expect(checkRow).toBeDefined();
    const id = checkRow!.getAttribute('data-cand')!;
    const ok = vi.spyOn(backend, 'assetsLicenseOk');
    await click(rowButton(id, 'Lizenz bestätigen'));
    expect(ok).toHaveBeenCalledWith('music:abend', hitId(id));
  });

  it('Netz-Kandidat mit Link: Knopf Seite öffnet die Adresse', async () => {
    await mount('tavern', 'kandidaten');
    const open = vi.spyOn(backend, 'openUrl').mockImplementation(() => undefined);
    await click(rowButton('sug:q05', 'Seite'));
    expect(open).toHaveBeenCalledWith('https://example.org/q05');
  });

  it('Sound: ▶ in der Vorschau spielt die Datei über die Abspielleiste', async () => {
    await mount('sfx:skill:healer', 'kandidaten');
    const play = host!.querySelector('[aria-label^="Vorschau abspielen"]');
    expect(play).not.toBeNull();
    await click(play);
    expect(playFile).toHaveBeenCalled();
  });

  it('Suchfeld fragt den Katalog neu an', async () => {
    await mount('farm', 'kandidaten');
    const list = vi.spyOn(backend, 'assetCandidates');
    await typeQuery('crate');
    expect(list).toHaveBeenCalledWith('farm', 'crate', 60);
  });
});
