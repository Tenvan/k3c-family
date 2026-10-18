import { STALE } from './stale';
import type { AssetChoice, AssetHit, AssetSlot, AssetsView, Backend, CatalogAsset } from './types';

// Erfundenes Slot-Register und Katalog für den Mock (Kopie aus k3c-dev, B-459). Bilder liefert der Studio-Vite aus
// public/ (publicDir nur im Dev-Modus). Der Stand zählt jede Änderung hoch; `external` spielt einen fremden Schreiber
// (MCP) nach, damit Neuladen und Schreibregel (AC-07) ohne Wails prüfbar sind; `quiet` hält das Ereignis zurück wie
// der Poll-Takt in Go (Fenster zwischen fremdem Schreiben und assets:changed).

const TOWN = 'grafik/gothicvania-town/props-einzeln';

const CATALOG: CatalogAsset[] = [
  { datei: `${TOWN}/well.png`, art: 'grafik', pack: 'gothicvania-town', gruppe: 'props-einzeln', w: 65, h: 65, lizenz: 'CC0 1.0', urheber: 'Luis Zuno (ansimuz)' },
  { datei: `${TOWN}/wagon.png`, art: 'grafik', pack: 'gothicvania-town', gruppe: 'props-einzeln', w: 93, h: 75, lizenz: 'CC0 1.0', urheber: 'Luis Zuno (ansimuz)' },
  { datei: `${TOWN}/crate.png`, art: 'grafik', pack: 'gothicvania-town', gruppe: 'props-einzeln', w: 39, h: 35, lizenz: 'CC0 1.0' },
  { datei: 'grafik/plants/flower.png', art: 'grafik', pack: 'plants', w: 32, h: 32, lizenz: 'CC BY-NC 4.0', kennzeichen: 'NC' },
  { datei: 'grafik/lpc/tree.png', art: 'grafik', pack: 'lpc', w: 96, h: 128, lizenz: 'CC BY-SA 3.0', kennzeichen: 'SA' },
  { datei: 'sprites/horse/idle.png', art: 'grafik', pack: 'horse', w: 100, h: 65, lizenz: 'CC0 1.0' },
  { datei: 'audio/atlas.ogg', art: 'sound', pack: 'audio', lizenz: 'selbst erzeugt (kein Credit nötig)' },
  { datei: 'audio/probe-a.ogg', art: 'sound', pack: 'audio', lizenz: 'selbst erzeugt (kein Credit nötig)' },
  { datei: 'audio/probe-b.ogg', art: 'sound', pack: 'audio', lizenz: 'selbst erzeugt (kein Credit nötig)' },
];

const slot = (id: string, art: AssetSlot['art'], kategorie: string, beschreibung: string, wahl: AssetChoice[], bus?: string): AssetSlot => ({
  id, art, kategorie, beschreibung, wahl, status: wahl.length ? 'zugeordnet' : 'luecke', ...(bus && { bus }),
});

function initial(): AssetSlot[] {
  return [
    slot('farm', 'grafik', 'gebaeude', 'Gebäude: farm', [{ datei: `${TOWN}/well.png`, ausschnitt: { x: 0, y: 0, w: 65, h: 65 } }, { datei: `${TOWN}/wagon.png` }]),
    slot('storage', 'grafik', 'gebaeude', 'Gebäude: storage', [{ datei: `${TOWN}/crate.png` }]),
    slot('tavern', 'grafik', 'gebaeude', 'Gebäude: tavern', []),
    slot('mount:horse', 'grafik', 'figuren', 'Reittiere: mount:horse', [{ datei: 'sprites/horse/idle.png', sheet: 'horse' }]),
    { ...slot('portal', 'grafik', 'objekte', 'Truhen, Portale: portal', [{ datei: 'grafik/portals/weg.png' }]), status: 'fehlt' },
    slot('sfx:coin', 'sound', 'sfx', 'Münze', [{ datei: 'audio/atlas.ogg', sprite: 'coin' }], 'sfx'),
    slot('music:tag', 'sound', 'musik', 'Tag', [{ datei: 'audio/probe-a.ogg' }], 'music'),
    slot('music:abend', 'sound', 'musik', 'Abend', [], 'music'),
    slot('music:nacht', 'sound', 'musik', 'Nacht', [{ datei: 'audio/probe-b.ogg' }], 'music'),
    slot('music:kampf', 'sound', 'musik', 'Kampf', [{ datei: 'audio/probe-b.ogg' }, { datei: 'audio/probe-a.ogg' }], 'music'),
  ];
}

/** Prüft wie Go (assets.SetChoices): Datei im Katalog, passende Art, Ausschnitt innerhalb des Bildes. */
function check(s: AssetSlot, wahl: AssetChoice[]): void {
  wahl.forEach((c, i) => {
    const a = CATALOG.find((x) => x.datei === c.datei);
    if (!a) throw new Error(`präferenz ${i + 1}: ${c.datei} fehlt unter public/`);
    if (a.art !== s.art) throw new Error(`präferenz ${i + 1}: ${c.datei} passt nicht zur Art ${s.art}`);
    const r = c.ausschnitt;
    if (r && (r.x < 0 || r.y < 0 || r.w <= 0 || r.h <= 0 || r.x + r.w > (a.w ?? 0) || r.y + r.h > (a.h ?? 0))) {
      throw new Error(`präferenz ${i + 1}: ausschnitt ${r.x},${r.y},${r.w},${r.h} außerhalb von ${c.datei} (${a.w}×${a.h})`);
    }
  });
}

/** Erfundene Treffer einer Recherche: je Quelle einer, Lizenz-Status wie in Go eingeordnet */
function fakeHits(query: string): AssetHit[] {
  const q = encodeURIComponent(query);
  const hit = (id: string, quelle: string, titel: string, lizenz: string, lizenzStatus: AssetHit['lizenzStatus'], extra: Partial<AssetHit> = {}): AssetHit =>
    ({ id, quelle, titel, link: `https://example.org/${quelle}/${q}`, lizenz, lizenzStatus, erstellt: '2026-10-14', ...extra });
  return [
    hit(`c-${q}`, 'Wikimedia Commons', `${query} pixel.png`, 'CC BY-SA 4.0', 'erkannt', { kennzeichen: 'SA', download: 'https://example.org/datei.png' }),
    hit(`o-${q}`, 'OpenGameArt', `${query} pack`, '', 'pruefen'),
    hit(`p-${q}`, 'Agent', `${query} (Pixabay)`, 'Pixabay', 'ausgeschlossen'),
  ];
}

function mockImport(slots: AssetSlot[], hits: Map<string, AssetHit[]>, id: string, hit: string): string {
  const s = slots.find((x) => x.id === id);
  const h = hits.get(id)?.find((x) => x.id === hit);
  if (!s || !h) throw new Error(`treffer "${hit}" zu ${id} nicht gefunden`);
  if (h.lizenzStatus === 'ausgeschlossen') throw new Error(`treffer ${hit} ist nicht übernehmbar (Q70: ${h.lizenz})`);
  const datei = `${s.art === 'sound' ? 'audio' : 'grafik'}/kandidaten/${h.titel.toLowerCase().replace(/[^a-z0-9]+/g, '-')}.${s.art === 'sound' ? 'ogg' : 'png'}`;
  CATALOG.push({ datei, art: s.art, pack: 'kandidaten', w: 32, h: 32, lizenz: h.lizenz || 'unbekannt' });
  h.datei = datei;
  s.wahl.push({ datei });
  s.status = 'zugeordnet';
  return datei;
}

type AssetCalls = Pick<Backend, 'assetsList' | 'assetsSet' | 'assetsCatalog' | 'assetsHits' | 'assetsResearch' | 'assetsHitDrop' | 'assetsImport' | 'assetsDownload' | 'assetsLicenseOk'>;

/** Mock der Asset-Bindings; onChange meldet jede Änderung des Registers (wie Watch in Go), external schreibt als Fremder. */
export function mockAssets(onChange: () => void): AssetCalls & { external: (id: string, wahl: AssetChoice[], quiet?: boolean) => void } {
  const slots = initial();
  const hits = new Map<string, AssetHit[]>();
  let stand = 1;
  const changed = (quiet = false) => {
    stand += 1;
    if (!quiet) onChange();
  };
  const write = (s: AssetSlot, wahl: AssetChoice[], quiet = false) => {
    s.wahl = wahl.map((c) => ({ ...c }));
    s.status = wahl.length ? 'zugeordnet' : 'luecke';
    changed(quiet);
  };
  const imported = (id: string, hit: string) => {
    const datei = mockImport(slots, hits, id, hit);
    changed();
    return datei;
  };
  return {
    external: (id, wahl, quiet) => {
      const s = slots.find((x) => x.id === id);
      if (s) write(s, wahl, quiet);
    },
    assetsList: async (): Promise<AssetsView> => ({ slots: slots.map((s) => ({ ...s, wahl: [...s.wahl] })), error: '', preview: '/', stand: String(stand) }),
    assetsSet: async (id, wahl, seen) => {
      const s = slots.find((x) => x.id === id);
      if (!s) throw new Error(`unbekannter Asset-Slot "${id}"`);
      if (seen !== String(stand)) throw new Error(STALE);
      check(s, wahl);
      write(s, wahl);
      return { slot: { ...s, wahl: [...s.wahl] }, stand: String(stand) };
    },
    assetsHits: async (id) => [...(hits.get(id) ?? [])],
    assetsResearch: async (id, query) => {
      await new Promise((r) => setTimeout(r, 400));
      const old = hits.get(id) ?? [];
      const fresh = fakeHits(query || id).filter((h) => !old.some((o) => o.link === h.link));
      hits.set(id, [...old, ...fresh]);
      return { treffer: [...(hits.get(id) ?? [])], neu: fresh.length, hinweise: ['freesound: token fehlt (Mock)'] };
    },
    assetsHitDrop: async (id, hit) => {
      hits.set(id, (hits.get(id) ?? []).filter((h) => h.id !== hit));
    },
    // Import und Download legen im Mock nur einen Katalog-Eintrag an und hängen ihn als letzte Präferenz an.
    assetsImport: async (id, hit) => imported(id, hit),
    assetsDownload: async (id, hit) => imported(id, hit),
    assetsLicenseOk: async (id, hit) => {
      for (const h of hits.get(id) ?? []) if (h.id === hit) h.lizenzStatus = 'bestaetigt';
    },
    assetsCatalog: async (query, art, limit) => {
      const words = query.toLowerCase().split(/\s+/).filter(Boolean);
      const hay = (a: CatalogAsset) => [a.datei, a.pack, a.gruppe, a.titel, a.lizenz].join(' ').toLowerCase();
      return CATALOG.filter((a) => (!art || a.art === art) && words.every((w) => hay(a).includes(w))).slice(0, limit > 0 ? limit : 200);
    },
  };
}
