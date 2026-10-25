import type { NetworkMode, SourceApi, SourceCleanupView, SourceInfo, SourceProgress, SourceProposal } from './sourceTypes';

// Erfundene Quellen und Vorschlagsliste für den Mock (WZ30.4): Einträge in allen Status, simulierter Download mit
// Fortschritt (je Abfrage von sourceProgress +25 %) und einem Fehlerfall (q06 scheitert bei 50 %), Offline-Schalter.
// Zustand liegt im Modul, damit Tests ihn zurücksetzen können.

const REGELN: Record<string, string> = {
  OpenGameArt: 'CC0, CC-BY, OGA-BY, CC-BY-SA; NC und ND nur markiert',
  Kenney: 'CC0, ohne Namensnennung',
  'itch.io': 'Lizenz je Angebot prüfen; nur CC0 und CC-BY',
  freesound: 'CC0 und CC-BY; Sampling Plus und NC gesperrt',
};

type Seed = Partial<SourceProposal> & Pick<SourceProposal, 'slot' | 'art' | 'titel' | 'quelle' | 'groesse'>;

const p = (id: string, o: Seed): SourceProposal => ({
  id, lizenz: 'CC0 1.0', lizenzErlaubt: true, dateiname: `${id}.${o.art === 'sound' ? 'ogg' : 'png'}`, prioritaet: 2, status: 'offen', verwendet: false,
  seite: `https://example.org/${id}`, dateiUrl: `https://example.org/${id}/datei`, ...o,
});

const initial = (): SourceProposal[] => [
  p('q01', { slot: 'sfx:skill:healer', art: 'sound', titel: 'Heilen (Glöckchen)', quelle: 'OpenGameArt', urheber: 'Beispiel-Autor', groesse: 86_000, prioritaet: 1, status: 'geladen', verwendet: true,
    pruefsumme: 'a1b2c3d4', vorschau: 'audio/probe-a.ogg', hinweis: 'schon am Slot' }),
  p('q02', { slot: 'sfx:skill:healer', art: 'sound', titel: 'Heilen (Harfe)', quelle: 'freesound', urheber: 'Beispiel-Studio', lizenz: 'CC BY 4.0', groesse: 120_000, status: 'geladen',
    pruefsumme: 'e5f6a7b8', vorschau: 'audio/probe-b.ogg' }),
  p('q03', { slot: 'music:abend', art: 'sound', titel: 'Abendlied (Flöte)', quelle: 'OpenGameArt', groesse: 1_480_000, prioritaet: 1 }),
  p('q04', { slot: 'sfx:skill:shield', art: 'sound', titel: 'Schild (Metall)', quelle: 'freesound', lizenz: 'Sampling Plus 1.0', lizenzErlaubt: false, sperrgrund: 'Sampling Plus nicht erlaubt (Q70)', groesse: 54_000, prioritaet: 3 }),
  p('q05', { slot: 'tavern', art: 'grafik', titel: 'Taverne (Pixel)', quelle: 'Kenney', urheber: 'Kenney', groesse: 24_500, prioritaet: 1 }),
  p('q06', { slot: 'tavern', art: 'grafik', titel: 'Taverne (Fachwerk)', quelle: 'itch.io', urheber: 'Beispiel-Autor', lizenz: 'CC BY-SA 3.0', groesse: 31_200, hinweis: 'Download scheitert im Mock' }),
  p('q07', { slot: 'portal', art: 'grafik', titel: 'Portal (Rune)', quelle: 'OpenGameArt', lizenz: 'CC BY-ND 4.0', lizenzErlaubt: false, sperrgrund: 'ND erlaubt keinen Zuschnitt (Q70)', groesse: 18_900, prioritaet: 3 }),
  p('q08', { slot: 'pickaxe', art: 'grafik', titel: 'Spitzhacke (Symbol)', quelle: 'Kenney', groesse: 4_200, status: 'verworfen' }),
  p('q09', { slot: 'beetle', art: 'grafik', titel: 'Käfer (Sheet)', quelle: 'OpenGameArt', lizenz: 'CC-BY 3.0', urheber: 'Beispiel-Autor', groesse: 64_000, status: 'gesperrt',
    sperrgrund: 'Quelle nennt jetzt CC BY-NC (Eintrag: CC-BY)' }),
  p('q10', { slot: 'beetle', art: 'grafik', titel: 'Käfer (einzeln)', quelle: 'OpenGameArt', groesse: 22_000, status: 'geladen', pruefsumme: 'c9d0e1f2', vorschau: 'grafik/gothicvania-town/props-einzeln/crate.png' }),
];

let list = initial();
let offline = typeof window !== 'undefined' && new URLSearchParams(window.location.search).has('offline');
let mode: NetworkMode = 'auto';
/** Offline laut Schalter, im Modus auto laut Mock-Probe (`offline`). */
const isOffline = () => (mode === 'auto' ? offline : mode === 'offline');
let jobs = new Map<string, SourceProgress>();

/** Schaltet den Mock offline oder wieder online (auch `?offline=1` in der Adresse). */
export function setMockSourcesOffline(next: boolean): void {
  offline = next;
}

/** Setzt Liste, Downloads und Offline-Schalter zurück (Tests). */
export function resetMockSources(): void {
  list = initial();
  jobs = new Map();
  offline = false;
  mode = 'auto';
}

const info = (): SourceInfo[] =>
  Object.entries(REGELN).map(([name, lizenzregeln]) => {
    const mine = list.filter((s) => s.quelle === name);
    return { name, lizenzregeln, vorgeschlagen: mine.length, geladen: mine.filter((s) => s.status === 'geladen').length, verwendet: mine.filter((s) => s.verwendet).length };
  });

const update = (id: string, patch: Partial<SourceProposal>) => {
  list = list.map((s) => (s.id === id ? { ...s, ...patch } : s));
};

const need = (id: string): SourceProposal => {
  const s = list.find((x) => x.id === id);
  if (!s) throw new Error(`Eintrag ${id} unbekannt`);
  return s;
};

/** Ein Schritt aller Downloads: +25 %, q06 bricht bei 50 % ab. */
function tick(): SourceProgress[] {
  for (const [id, j] of jobs) {
    if (j.status === 'fertig' || j.status === 'fehler') continue;
    const geladen = Math.min(j.gesamt, j.geladen + Math.ceil(j.gesamt / 4));
    if (id === 'q06' && geladen >= j.gesamt / 2) {
      const fehler = 'Verbindung abgebrochen (Mock)';
      jobs.set(id, { ...j, geladen, status: 'fehler', fehler });
      update(id, { fehler });
    } else if (geladen >= j.gesamt) {
      jobs.set(id, { ...j, geladen, status: 'fertig' });
      update(id, { status: 'geladen', fehler: undefined, pruefsumme: `mock-${id}`, vorschau: need(id).art === 'sound' ? 'audio/probe-a.ogg' : 'grafik/gothicvania-town/props-einzeln/well.png' });
    } else jobs.set(id, { ...j, geladen, status: 'laedt' });
  }
  return [...jobs.values()];
}

function cleanupView(): SourceCleanupView {
  const dateien = list.filter((s) => s.status === 'geladen' && !s.verwendet).map((s) => ({ id: s.id, dateiname: s.dateiname, groesse: s.groesse }));
  return { dateien, groesse: dateien.reduce((a, d) => a + d.groesse, 0) };
}

export function mockSources(): SourceApi {
  return {
    network: async () => ({ mode, offline: isOffline() }),
    setNetwork: async (next) => {
      mode = next;
      return { mode, offline: isOffline() };
    },
    sources: async () => ({ offline: isOffline(), quellen: info(), vorschlaege: list.map((s) => ({ ...s })) }),
    sourceDownload: async (ids) => {
      if (isOffline()) throw new Error('offline: Download braucht Internet');
      const todo = ids.map(need);
      for (const s of todo) {
        if (!s.lizenzErlaubt || s.status === 'gesperrt') throw new Error(`${s.id} gesperrt: ${s.sperrgrund ?? s.lizenz}`);
      }
      for (const s of todo) {
        if (s.status === 'geladen') continue;
        jobs.set(s.id, { id: s.id, status: 'wartet', geladen: 0, gesamt: s.groesse });
        update(s.id, { fehler: undefined });
      }
    },
    sourceProgress: async () => tick(),
    sourceAdopt: async (id) => {
      const s = need(id);
      if (s.status !== 'geladen') throw new Error(`${id} ist nicht geladen`);
      update(id, { verwendet: true });
    },
    sourceCleanupPreview: async () => cleanupView(),
    sourceCleanup: async () => {
      const view = cleanupView();
      for (const d of view.dateien) update(d.id, { status: 'offen', pruefsumme: undefined, vorschau: undefined });
      return view;
    },
  };
}
