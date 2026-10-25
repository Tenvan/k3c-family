import type { AssetSuggestion, SuggestionApi } from './suggestionTypes';

// Erfundene Vorschläge des Chats für den Mock (WZ30.5): mehrere je Art, einer mit Lizenzverstoß (Q70), einer noch nicht
// im Vorrat (g2), Offline-Schalter und Stand des Registers wie im Backend (WZ29.7).
// Vorschau-Adressen zeigen auf vorhandene Dateien unter public/. Zustand liegt im Modul, damit Tests ihn zurücksetzen können.

const TOWN = 'grafik/gothicvania-town/props-einzeln';

const initial = (): AssetSuggestion[] => [
  { id: 's1', slot: 'music:abend', art: 'sound', bus: 'music', titel: 'Abendlied (Flöte)', datei: 'audio/kandidaten/abend.ogg', quelle: 'OpenGameArt',
    urheber: 'Beispiel-Autor', lizenz: 'CC0 1.0', lizenzErlaubt: true, groesse: 1_480_000, vorschau: 'audio/probe-a.ogg', status: 'geladen' },
  { id: 's2', slot: 'sfx:skill:healer', art: 'sound', bus: 'sfx', titel: 'Heilen (Glöckchen)', datei: 'audio/kandidaten/heal.ogg', quelle: 'Wikimedia Commons',
    urheber: 'Beispiel-Studio', lizenz: 'CC BY 4.0', lizenzErlaubt: true, groesse: 86_000, vorschau: 'audio/probe-b.ogg', status: 'geladen' },
  { id: 's3', slot: 'sfx:skill:shield', art: 'sound', bus: 'sfx', titel: 'Schild (Metall)', datei: 'audio/kandidaten/shield.ogg', quelle: 'Pixabay',
    lizenz: 'Pixabay-Lizenz', lizenzErlaubt: false, lizenzGrund: 'Lizenz ohne Weitergabe der Rohdateien (Q70)', groesse: 54_000, vorschau: 'audio/probe-a.ogg', status: 'gesperrt' },
  { id: 'g1', slot: 'tavern', art: 'grafik', titel: 'Taverne (Pixel)', datei: 'grafik/kandidaten/tavern.png', quelle: 'OpenGameArt',
    urheber: 'Luis Zuno (ansimuz)', lizenz: 'CC0 1.0', lizenzErlaubt: true, groesse: 24_500, vorschau: `${TOWN}/wagon.png`, status: 'geladen' },
  { id: 'g2', slot: 'tavern', art: 'grafik', titel: 'Taverne (Fachwerk)', datei: 'grafik/kandidaten/tavern2.png', quelle: 'OpenGameArt',
    urheber: 'Beispiel-Autor', lizenz: 'CC BY-SA 3.0', lizenzErlaubt: true, groesse: 31_200, vorschau: '', status: 'offen' },
  { id: 'g3', slot: 'portal', art: 'grafik', titel: 'Portal (Rune)', datei: 'grafik/kandidaten/portal.png', quelle: 'Wikimedia Commons',
    urheber: 'Beispiel-Autor', lizenz: 'CC BY-ND 4.0', lizenzErlaubt: false, lizenzGrund: 'ND erlaubt keinen Zuschnitt (Q70)', groesse: 18_900, vorschau: `${TOWN}/crate.png`, status: 'gesperrt' },
];

let list = initial();
let stand = 1;
let offline = typeof window !== 'undefined' && new URLSearchParams(window.location.search).has('offline');

/** Schaltet den Mock offline oder wieder online (auch `?offline=1` in der Adresse). */
export function setMockSuggestionsOffline(next: boolean): void {
  offline = next;
}

/** Setzt Vorschläge und Offline-Schalter zurück (Tests). */
export function resetMockSuggestions(): void {
  list = initial();
  stand = 1;
  offline = false;
}

const find = (slotId: string, id: string) => list.find((s) => s.id === id && (slotId === '' || s.slot === slotId));

export function mockSuggestions(): SuggestionApi {
  return {
    assetSuggestions: async (slotId) => ({ offline, stand: `m${stand}`, vorschlaege: offline ? [] : list.filter((s) => slotId === '' || s.slot === slotId) }),
    assetSuggestionAdopt: async (slotId, id, at) => {
      if (offline) throw new Error('offline: Chat und Internet sind nicht erreichbar');
      const s = find(slotId, id);
      if (!s) throw new Error(`Vorschlag ${id} unbekannt`);
      if (!s.lizenzErlaubt) throw new Error(`Lizenz nicht erlaubt: ${s.lizenzGrund ?? s.lizenz}`);
      if (s.status !== 'geladen') throw new Error(`${id} liegt nicht im Vorrat: erst auf der Seite Quellen laden`);
      if (at !== `m${stand}`) throw new Error('Register geändert, bitte erneut speichern');
      list = list.filter((x) => x !== s);
      stand += 1;
      return `m${stand}`;
    },
    assetSuggestionDrop: async (slotId, id) => {
      const s = find(slotId, id);
      if (s) list = list.filter((x) => x !== s);
    },
  };
}
