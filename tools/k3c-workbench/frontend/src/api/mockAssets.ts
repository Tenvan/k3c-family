import type { AssetEintrag, AssetsBackend, Frame, Groesse, Hintergrund, PixelDiff, Quellen, Review, SoundEintrag, Spec, Stil, Vorlage } from './typesAssets';

// Mock der Asset-Methoden (WB1.1): die Beispielantworten aus tools/paper/fixtures/ui (PV1.4), geladen beim ersten
// Aufruf. Wie die Go-Stubs (internal/assetsui) kennt er nur das Asset monarch.
const FIXTURES = import.meta.glob<string>('../../../../paper/fixtures/ui/*.json', { query: '?raw', import: 'default' });

async function fixture<T>(name: string): Promise<T> {
  const load = Object.entries(FIXTURES).find(([path]) => path.endsWith(`/${name}.json`))?.[1];
  if (!load) throw new Error(`Fixture ${name}.json fehlt`);
  return JSON.parse(await load()) as T;
}

function known(id: string) {
  if (id !== 'monarch') throw new Error(`Asset „${id}“ fehlt`);
}

/** Erfundene Sound-Slots und Quellen (WB1.4), Form wie internal/studioview. */
const SOUNDS: SoundEintrag[] = [
  { id: 'music:tag', kategorie: 'musik', beschreibung: 'Musik am Tag', bus: 'music', status: 'zugeordnet', dateien: ['audio/tag.ogg'] },
  { id: 'sfx:muenze', kategorie: 'sfx', beschreibung: 'Münze aufheben', status: 'fehlt', dateien: ['audio/muenze.ogg'] },
  { id: 'ambient:wald', kategorie: 'ambient', beschreibung: 'Waldrauschen', status: 'luecke', dateien: [] },
];
const QUELLEN: Quellen = {
  katalog: [
    { datei: 'audio/tag.ogg', art: 'sound', pack: 'Freie Musik', lizenz: 'CC0' },
    { datei: 'audio/kampf.ogg', art: 'sound', pack: 'Kampfmusik', lizenz: 'CC BY-NC 4.0', kennzeichen: 'NC' },
  ],
  treffer: {
    'ambient:wald': [{ id: 'h1', titel: 'Wald bei Tag', link: 'https://opengameart.org/', quelle: 'OpenGameArt', lizenz: 'CC0', lizenzStatus: 'erkannt', erstellt: '2026-11-01' }],
  },
};

/** Transparentes 1×1-PNG für die Hintergrund-Ebenen des Mocks. */
const PIXEL = 'data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAAEAAAABCAQAAAC1HAwCAAAAC0lEQVR42mNkYAAAAAYAAjCB0C8AAAAASUVORK5CYII=';

export function mockAssets(): AssetsBackend {
  return {
    assetsList: () => fixture<AssetEintrag[]>('AssetsList'),
    // Index ohne Prüfung: alle offen und gültig, die Prüfung liefert die Fixture (B-732)
    assetsIndex: async () => (await fixture<AssetEintrag[]>('AssetsList')).map((e) => ({ ...e, fehler: undefined, gueltig: true, offen: true })),
    // Größenvergleich (B-733): Maßstab des Monarchen laut data/sprites.json, ein Biom mit 1×1-PNGs
    // Teil ausblenden (B-734): Fixture-Frame ohne die Boxen der ausgeblendeten Teile
    assetsFrameOhne: async (id, _animation, _frame, ohne) => {
      known(id);
      const f = await fixture<Frame>('AssetsFrame');
      return { ...f, teile: Object.fromEntries(Object.entries(f.teile).filter(([t]) => !ohne.includes(t))) };
    },
    assetsGroesse: async (id): Promise<Groesse> => {
      known(id);
      return { pxProEinheit: (81 * 1.5) / 145, quelle: 'Spielrolle Spieler (data/sprites.json)' };
    },
    assetsHintergruende: async () => ['wald'],
    assetsHintergrund: async (teil): Promise<Hintergrund> => {
      if (teil !== 'wald') throw new Error(`Hintergrund „${teil}“ fehlt`);
      return { fern: PIXEL, mitte: PIXEL, nah: PIXEL };
    },
    assetsCheck: async (pfade) => (await fixture<AssetEintrag[]>('AssetsList')).filter((e) => pfade.includes(e.pfad)),
    assetsGet: async (id) => {
      known(id);
      return fixture<Spec>('AssetsGet');
    },
    assetsStyle: () => fixture<Stil>('AssetsStyle'),
    assetsPrompts: () => fixture<Vorlage[]>('AssetsPrompts'),
    assetsReview: () => fixture<Review>('AssetsReview'),
    assetsFrame: async (id) => {
      known(id);
      return fixture<Frame>('AssetsFrame');
    },
    assetsPixelDiff: async (id) => {
      known(id);
      return fixture<PixelDiff>('AssetsPixelDiff');
    },
    studioSounds: async () => SOUNDS,
    studioSoundData: async (pfad) => {
      if (!SOUNDS.some((s) => s.status === 'zugeordnet' && s.dateien.includes(pfad))) throw new Error(`${pfad} fehlt unter public/`);
      return 'data:audio/ogg;base64,T2dnUw==';
    },
    studioSources: async () => QUELLEN,
    studioGameOpen: async (seed) => `http://127.0.0.1:8099/index.html#${encodeURIComponent(`game.html?autostart=1&save=voll-${seed}`)}`,
  };
}
