import { renderToStaticMarkup } from 'react-dom/server';
import { describe, expect, it, vi } from 'vitest';
import { httpBackend } from '../api/http';
import { mockSources } from '../api/mockSources';
import { gotoSources, takePendingSlot } from './jump';
import { ProposalRow } from './ProposalRow';
import { SourcesPage } from './SourcesPage';

// WZ30.4 (AC-04): Seite und Zeilen der Quellen; Dialoge und Download-Ablauf prüft sourcesLogic.test.ts über den Mock.
const rows = async () => (await mockSources().sources()).vorschlaege;
const row = (id: string, offline: boolean, rs: Awaited<ReturnType<typeof rows>>) =>
  renderToStaticMarkup(<ProposalRow s={rs.find((s) => s.id === id)!} offline={offline} checked={false} onCheck={() => undefined} onAdopt={() => undefined} />);

describe('Seite Quellen', () => {
  it('zeigt vor dem Laden den Hinweis', () => {
    expect(renderToStaticMarkup(<SourcesPage />)).toContain('Lade Quellen');
  });

  it('graut Ungeladenes offline aus, Geladenes bleibt mit Hörprobe', async () => {
    const rs = await rows();
    expect(row('q03', true, rs)).toContain('data-dim="true"');
    expect(row('q02', true, rs)).toContain('data-dim="false"');
    expect(row('q02', true, rs)).toContain('Hörprobe');
    expect(row('q03', false, rs)).toContain('data-dim="false"');
  });

  it('markiert gesperrte Einträge mit Grund und geladene als verwendet oder unverwendet', async () => {
    const rs = await rows();
    expect(row('q04', false, rs)).toContain('Sampling Plus nicht erlaubt');
    expect(row('q01', false, rs)).toContain('>verwendet<');
    expect(row('q02', false, rs)).toMatch(/>unverwendet<.*Übernehmen/);
  });

  it('merkt sich den Slot des Sprungs einmalig', () => {
    const seen = vi.fn();
    vi.stubGlobal('window', { dispatchEvent: seen });
    gotoSources('tavern');
    vi.unstubAllGlobals();
    expect(seen).toHaveBeenCalledOnce();
    expect(takePendingSlot()).toBe('tavern');
    expect(takePendingSlot()).toBe('');
  });

  it('ruft im HTTP-Backend die RPC-Methoden des Studios (WZ29.6)', async () => {
    const calls: { url: string; body: string }[] = [];
    vi.stubGlobal('fetch', (url: string, init: RequestInit) => {
      calls.push({ url, body: String(init.body) });
      return Promise.resolve(new Response(JSON.stringify({ result: null }), { status: 200 }));
    });
    try {
      await httpBackend().sources();
      await httpBackend().sourceDownload(['q1', 'q2'], 300);
      await httpBackend().sourceAdopt('q1');
    } finally {
      vi.unstubAllGlobals();
    }
    expect(calls).toEqual([
      { url: '/api/studio/rpc/Sources', body: '[]' },
      { url: '/api/studio/rpc/SourceDownload', body: '[["q1","q2"],300]' },
      { url: '/api/studio/rpc/SourceAdopt', body: '["q1"]' },
    ]);
  });
});
