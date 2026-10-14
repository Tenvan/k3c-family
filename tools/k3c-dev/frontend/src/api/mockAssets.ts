import type { AssetChoice, AssetSlot, AssetsView, Backend, CatalogAsset } from './types';

// Erfundenes Slot-Register und Katalog für den Mock. Bilder und Mini-Szene kommen vom Vite-Dienst des Spiels:
// im Browser mit `?preview=http://localhost:5173/` aufrufen, sonst zeigt die Seite den Hinweis „Vite starten“.

const TOWN = 'grafik/gothicvania-town/props-einzeln';

const CATALOG: CatalogAsset[] = [
  { datei: `${TOWN}/house-a.png`, art: 'grafik', pack: 'gothicvania-town', gruppe: 'props-einzeln', w: 168, h: 183, lizenz: 'CC0 1.0', urheber: 'Luis Zuno (ansimuz)' },
  { datei: `${TOWN}/house-b.png`, art: 'grafik', pack: 'gothicvania-town', gruppe: 'props-einzeln', w: 210, h: 244, lizenz: 'CC0 1.0', urheber: 'Luis Zuno (ansimuz)' },
  { datei: `${TOWN}/crate-stack.png`, art: 'grafik', pack: 'gothicvania-town', gruppe: 'props-einzeln', w: 73, h: 68, lizenz: 'CC0 1.0' },
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
    slot('farm', 'grafik', 'gebaeude', 'Gebäude: farm', [{ datei: `${TOWN}/house-a.png`, ausschnitt: { x: 0, y: 0, w: 168, h: 183 } }, { datei: `${TOWN}/house-b.png` }]),
    slot('storage', 'grafik', 'gebaeude', 'Gebäude: storage', [{ datei: `${TOWN}/crate-stack.png` }]),
    slot('tavern', 'grafik', 'gebaeude', 'Gebäude: tavern', []),
    slot('mount:horse', 'grafik', 'figuren', 'Reittiere: mount:horse', [{ datei: 'sprites/horse/idle.png', sheet: 'horse' }]),
    { ...slot('portal', 'grafik', 'objekte', 'Truhen, Portale: portal', [{ datei: 'grafik/portals/weg.png' }]), status: 'fehlt' },
    slot('sfx:coin', 'sound', 'sfx', 'Münze', [{ datei: 'audio/atlas.ogg', sprite: 'coin' }], 'sfx'),
    slot('music:tag', 'sound', 'musik', 'Tag', [{ datei: 'audio/probe-a.ogg' }], 'music'),
    slot('music:abend', 'sound', 'musik', 'Abend', [], 'music'),
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

export function mockAssets(): Pick<Backend, 'assetsList' | 'assetsSet' | 'assetsCatalog'> {
  const slots = initial();
  const preview = new URLSearchParams(window.location.search).get('preview') ?? '';
  return {
    assetsList: async (): Promise<AssetsView> => ({ slots: slots.map((s) => ({ ...s, wahl: [...s.wahl] })), error: '', preview }),
    assetsSet: async (id, wahl) => {
      const s = slots.find((x) => x.id === id);
      if (!s) throw new Error(`unbekannter Asset-Slot "${id}"`);
      check(s, wahl);
      s.wahl = wahl.map((c) => ({ ...c }));
      s.status = wahl.length ? 'zugeordnet' : 'luecke';
      return { ...s, wahl: [...s.wahl] };
    },
    assetsCatalog: async (query, art, limit) => {
      const words = query.toLowerCase().split(/\s+/).filter(Boolean);
      const hay = (a: CatalogAsset) => [a.datei, a.pack, a.gruppe, a.titel, a.lizenz].join(' ').toLowerCase();
      return CATALOG.filter((a) => (!art || a.art === art) && words.every((w) => hay(a).includes(w))).slice(0, limit > 0 ? limit : 200);
    },
  };
}
