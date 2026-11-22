// @vitest-environment happy-dom
import { Theme } from '@radix-ui/themes';
import { act } from 'react';
import { createRoot, type Root } from 'react-dom/client';
import { afterEach, beforeEach, describe, expect, it, vi } from 'vitest';
import { PAGES } from '../Header';
import { playtestApi } from './api';
import { PlaytestPage } from './PlaytestPage';
import { RecordMonitor } from './RecordMonitor';
import type { PlaytestStats } from './types';

// B-1218/AC-01 und AC-03 (Oberfläche): Die Seite listet die Sessions neueste zuerst mit Stand, jeder Löschknopf nennt vor
// dem Löschen die Dateien und löscht erst nach der Bestätigung; eine laufende Aufnahme ist gesperrt.

(globalThis as { IS_REACT_ACT_ENVIRONMENT?: boolean }).IS_REACT_ACT_ENVIRONMENT = true;

let root: Root | null = null;

async function flush() {
  for (let i = 0; i < 5; i++) await act(async () => new Promise((r) => setTimeout(r, 0)));
}

async function mount(): Promise<HTMLElement> {
  const host = document.createElement('div');
  document.body.appendChild(host);
  root = createRoot(host);
  act(() => root!.render(<Theme><PlaytestPage /></Theme>));
  await flush();
  return host;
}

const button = (label: string, within: ParentNode = document): HTMLButtonElement =>
  [...within.querySelectorAll('button')].find((b) => b.textContent?.trim() === label) as HTMLButtonElement;

const rows = (host: HTMLElement) => [...host.querySelectorAll('[data-session]')].map((e) => e.getAttribute('data-session'));

beforeEach(() => playtestApi.reset?.());

afterEach(() => {
  act(() => root?.unmount());
  root = null;
  document.body.innerHTML = '';
});

describe('Seite Spieltest', () => {
  it('ist im Bereich Development als Reiter eingetragen', () => {
    expect(PAGES).toContain('spieltest');
  });

  it('listet die Sessions neueste zuerst mit Stand', async () => {
    const host = await mount();
    expect(rows(host)).toEqual(['2026-11-22-1', '2026-11-21-1', '2026-11-20-1', '2026-11-19-1']);
    const list = host.querySelector('[data-session="2026-11-21-1"]')!.textContent!;
    expect(list).toContain('ausgewertet');
    expect(list).toContain('14 Marker');
    expect(host.querySelector('[data-session="2026-11-19-1"]')!.textContent).toContain('unvollständig');
  });

  it('Video löschen: erst Dialog mit Datei und Größe, nach Bestätigung fehlt nur das Video', async () => {
    const host = await mount();
    act(() => (host.querySelector('[data-session="2026-11-21-1"]') as HTMLElement).click());
    await flush();
    act(() => button('Video löschen', host).click());
    await flush();
    const dialog = document.querySelector('[role="alertdialog"]')!;
    expect(dialog.textContent).toContain('video.mkv (440,0 MB)');
    expect(host.querySelector('[data-detail="2026-11-21-1"]')!.textContent).toContain('video.mkv');
    act(() => button('Video löschen', dialog).click());
    await flush();
    const detail = host.querySelector('[data-detail="2026-11-21-1"]')!.textContent!;
    expect(detail).not.toContain('video.mkv');
    expect(detail).toContain('transcript.json');
    expect(detail).toContain('findings.json');
  });

  it('Abbrechen löscht nichts', async () => {
    const host = await mount();
    act(() => (host.querySelector('[data-session="2026-11-20-1"]') as HTMLElement).click());
    await flush();
    act(() => button('Session löschen', host).click());
    await flush();
    expect(document.querySelector('[role="alertdialog"]')!.textContent).toContain('2026-11-20-1/');
    act(() => button('Abbrechen').click());
    await flush();
    expect(rows(host)).toContain('2026-11-20-1');
  });

  it('Session löschen entfernt sie erst nach Bestätigung aus der Liste', async () => {
    const host = await mount();
    act(() => (host.querySelector('[data-session="2026-11-19-1"]') as HTMLElement).click());
    await flush();
    act(() => button('Session löschen', host).click());
    await flush();
    expect(rows(host)).toContain('2026-11-19-1');
    act(() => button('Session löschen', document.querySelector('[role="alertdialog"]')!).click());
    await flush();
    expect(rows(host)).not.toContain('2026-11-19-1');
  });

  it('Videos ausgewerteter Sessions: Dialog nennt alle Videos, danach nur diese weg; Knopf dann gesperrt', async () => {
    const host = await mount();
    act(() => button('Videos ausgewerteter Sessions löschen', host).click());
    await flush();
    const dialog = document.querySelector('[role="alertdialog"]')!;
    expect(dialog.textContent).toContain('2026-11-21-1/video.mkv');
    expect(dialog.textContent).not.toContain('2026-11-20-1/video.mkv');
    act(() => button('Videos löschen', dialog).click());
    await flush();
    expect(button('Videos ausgewerteter Sessions löschen', host).disabled).toBe(true);
    act(() => (host.querySelector('[data-session="2026-11-20-1"]') as HTMLElement).click());
    await flush();
    expect(host.querySelector('[data-detail="2026-11-20-1"]')!.textContent).toContain('video.mkv');
  });

  it('laufende Aufnahme: Löschknöpfe gesperrt', async () => {
    const host = await mount();
    act(() => (host.querySelector('[data-session="2026-11-22-1"]') as HTMLElement).click());
    await flush();
    expect(button('Video löschen', host).disabled).toBe(true);
    expect(button('Session löschen', host).disabled).toBe(true);
  });

  it('Player: Audio und Video mit Adresse der Datei', async () => {
    const host = await mount();
    act(() => (host.querySelector('[data-session="2026-11-20-1"]') as HTMLElement).click());
    await flush();
    expect(host.querySelector('audio')!.getAttribute('src')).toBe('/api/dev/playtest/media/2026-11-20-1/voice.wav');
    expect(host.querySelector('video')!.getAttribute('src')).toBe('/api/dev/playtest/media/2026-11-20-1/video.mkv');
  });

  it('Bild-Player steht über dem Ton-Player, die Liste nennt die Uhrzeit statt eines Zählers', async () => {
    const host = await mount();
    act(() => (host.querySelector('[data-session="2026-11-20-1"]') as HTMLElement).click());
    await flush();
    const order = [...host.querySelectorAll('[data-player]')].map((e) => e.getAttribute('data-player'));
    expect(order).toEqual(['video', 'voice']);
    expect(host.querySelector('[data-session="2026-11-20-1"]')!.textContent).toContain('20.11.2026 · 18:00');
  });

  it('Neu laden liest die Modelle neu (neue Modelldatei in models/)', async () => {
    const host = await mount();
    const calls = vi.spyOn(playtestApi, 'models');
    act(() => button('Neu laden', host).click());
    await flush();
    expect(calls).toHaveBeenCalled();
    calls.mockRestore();
  });
});

describe('Spieltest starten (B-1219)', () => {
  it('Start gesperrt ohne Mikro; mit Aufnahme: Start gesperrt, Stopp frei; Stopp beendet sie', async () => {
    const host = await mount();
    const bar = () => host.querySelector('[data-record]')!;
    expect(button('Spieltest starten', host).disabled).toBe(true);
    await playtestApi.start({ mic: 'Raummikro (USB)', monitor: 1, gameArgs: [] });
    await act(async () => new Promise((r) => setTimeout(r, 1100)));
    expect(bar().getAttribute('data-record')).toBe('running');
    expect(bar().textContent).toContain('Aufnahme 2026-11-23-1');
    expect(button('Spieltest starten', host).disabled).toBe(true);
    expect(button('Stopp', host).disabled).toBe(false);
    await expect(playtestApi.start({ mic: 'x', monitor: 0, gameArgs: [] })).rejects.toThrow('läuft schon');
    act(() => button('Stopp', host).click());
    await act(async () => new Promise((r) => setTimeout(r, 1100)));
    expect(bar().getAttribute('data-record')).toBe('idle');
  });
});

describe('Mikro-Session', () => {
  it('startet nur mit Mikro, zeigt nur das Ton-Diagramm und ist als Mikro-Session beschriftet', async () => {
    const host = await mount();
    const bar = () => host.querySelector('[data-record]')!;
    expect(button('Nur Mikro starten', host).disabled).toBe(true);
    await playtestApi.start({ mic: 'Raummikro (USB)', monitor: 0, gameArgs: [], voiceOnly: true });
    await act(async () => new Promise((r) => setTimeout(r, 1100)));
    expect(bar().textContent).toContain('Mikro-Session 2026-11-23-1');
    expect(host.querySelector('[data-chart="Ton"]')).toBeTruthy();
    expect(host.querySelector('[data-chart="Bild"]')).toBeNull();
    act(() => button('Stopp', host).click());
    await act(async () => new Promise((r) => setTimeout(r, 1100)));
  });
});

describe('Geräte (B-1230)', () => {
  it('Monitore kommen aus der Geräteprobe mit Auflösung', async () => {
    const host = await mount();
    expect(host.querySelector('[aria-label="Monitor"]')!.textContent).toContain('Monitor 1 (2560×1440)');
    expect(button('Geräte neu suchen', host)).toBeTruthy();
  });

  it('ohne Aufnahmegerät steht ein Hinweis da und der Start bleibt gesperrt', async () => {
    const real = playtestApi.devices;
    playtestApi.devices = async () => ({ mics: [], monitors: [], ffmpeg: true, ffmpegPath: 'ffmpeg', encoder: '', message: '' });
    try {
      const host = await mount();
      expect(host.textContent).toContain('Kein Mikrofon gefunden');
      expect(button('Spieltest starten', host).disabled).toBe(true);
      expect(host.querySelector('[aria-label="Monitor"]')!.textContent).toContain('Monitor 1');
    } finally {
      playtestApi.devices = real;
    }
  });

  it('ein gemerktes Mikro, das es nicht mehr gibt, sperrt den Start und warnt', async () => {
    localStorage.setItem('k3c-workbench.playtest.mic', 'Abgezogenes Mikro');
    try {
      const host = await mount();
      expect(button('Spieltest starten', host).disabled).toBe(true);
      expect(host.textContent).toContain('Mikrofon nicht gefunden');
    } finally {
      localStorage.removeItem('k3c-workbench.playtest.mic');
    }
  });
});

describe('Aufnahme-Monitor', () => {
  it('bricht nicht ab, wenn die Raten beim Start noch null sind (Go liefert nil-Listen als null)', () => {
    const host = document.createElement('div');
    const root = createRoot(host);
    const stats = { voiceBytes: 0, videoBytes: 0, voiceRate: null, videoRate: null, voiceLevel: null, warn: '' } as unknown as PlaytestStats;
    act(() => root.render(<RecordMonitor stats={stats} />));
    expect(host.querySelector('[data-chart="Ton"]')!.textContent).toContain('Pegel 0 %');
    act(() => root.unmount());
  });

  it('zeigt zwei Diagramme (Ton, Bild) während der Aufnahme und warnt, wenn etwas steht', async () => {
    const host = await mount();
    await playtestApi.start({ mic: 'Raummikro (USB)', monitor: 0, gameArgs: [] });
    await act(async () => new Promise((r) => setTimeout(r, 1100)));
    const mon = host.querySelector('[data-monitor]')!;
    expect(mon.getAttribute('data-monitor')).toBe('ok');
    expect(mon.querySelector('[data-chart="Ton"]')!.textContent).toContain('Pegel 30 %');
    expect(mon.querySelector('[data-chart="Bild"]')).toBeTruthy();
    expect(mon.textContent).not.toContain('Aufnahme prüfen');
    act(() => button('Stopp', host).click());
    await act(async () => new Promise((r) => setTimeout(r, 1100)));
  });
});

describe('Auswertung (B-1223)', () => {
  it('Auswerten ergibt Stand transkribiert und eine Zeitleiste mit Satz und Marker; Klick springt im Player', async () => {
    const host = await mount();
    act(() => (host.querySelector('[data-session="2026-11-20-1"]') as HTMLElement).click());
    await flush();
    const panel = () => host.querySelector('[data-timeline]')!;
    expect(panel().querySelectorAll('[data-entry="sprache"]').length).toBe(0);
    expect(panel().querySelectorAll('[data-entry="marker"]').length).toBe(1);
    expect(button('Auswerten', host).disabled).toBe(false);
    act(() => button('Auswerten', host).click());
    await flush();
    expect(host.querySelector('[data-session="2026-11-20-1"]')!.textContent).toContain('transkribiert');
    const kinds = [...panel().querySelectorAll('[data-entry]')].map((e) => e.getAttribute('data-entry'));
    expect(kinds).toEqual(['sprache', 'marker']);
    expect(panel().textContent).toContain('Hier fehlt der Turm.');
    const video = host.querySelector('video') as HTMLVideoElement;
    act(() => (panel().querySelector('[data-entry="sprache"]') as HTMLElement).click());
    expect(video.currentTime).toBe(1);
  });

  it('Befunde: Haken speichern, Duplikat und Prompt-Knöpfe (B-1224)', async () => {
    const host = await mount();
    act(() => (host.querySelector('[data-session="2026-11-21-1"]') as HTMLElement).click());
    await flush();
    const panel = () => host.querySelector('[data-findings]')!;
    const has = (t: string) => [...panel().querySelectorAll('button')].some((b) => b.textContent === t);
    expect(panel().querySelectorAll('[data-finding]').length).toBe(2);
    expect(panel().textContent).toContain('Duplikat von B-1100');
    expect(has('Prompt: Auswerten')).toBe(true);
    expect(has('Prompt: Tickets anlegen')).toBe(false);
    const box = panel().querySelector('[data-finding="f1"] [role="checkbox"]') as HTMLElement;
    act(() => box.click());
    await flush();
    expect((await playtestApi.findings('2026-11-21-1')).find((f) => f.id === 'f1')?.selected).toBe(true);
    expect(has('Prompt: Tickets anlegen')).toBe(true);
  });

  it('Modell und Sprache sind wählbar, Standard ist das deutsche Modell', async () => {
    const host = await mount();
    act(() => (host.querySelector('[data-session="2026-11-20-1"]') as HTMLElement).click());
    await flush();
    expect(host.querySelector('[aria-label="Whisper-Modell"]')!.textContent).toContain('german-q5_0');
    expect(host.querySelector('[aria-label="Sprache"]')!.textContent).toContain('de');
    expect((await playtestApi.models()).filter((m) => m.default).length).toBe(1);
  });

  it('Auswerten ist bei laufender Aufnahme und ohne Sprachspur gesperrt', async () => {
    const host = await mount();
    expect(button('Auswerten', host).disabled).toBe(true);
    act(() => (host.querySelector('[data-session="2026-11-19-1"]') as HTMLElement).click());
    await flush();
    expect(button('Auswerten', host).disabled).toBe(true);
  });
});
