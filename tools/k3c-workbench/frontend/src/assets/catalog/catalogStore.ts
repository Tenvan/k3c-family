import { useEffect, useSyncExternalStore } from 'react';
import { backend } from '../../api';
import { errorText } from '@webkit/lib/errors';
import { assetsCheck, assetsIndex, assetsPrompts, assetsReview, type AssetEintrag, type Frame, type Spec, type Vorlage } from '../api';

/** Katalog-Cache (B-732): Liste, Prompt-Vorlagen, IDs im Diff, Specs und Frames leben auf Modul-Ebene und überleben den
 *  Abbau der Seite beim Tab-Wechsel. Die Liste kommt ungeprüft (`assetsIndex`), die Prüfung folgt in Häppchen
 *  (`assetsCheck`); „Neu laden“ und ein Neuverbinden zum Backend leeren den Cache. */
export interface CatalogState {
  list: AssetEintrag[];
  inDiff: Set<string>;
  prompts: Vorlage[];
  error: string;
  loading: boolean;
  /** Zählt die Ladevorgänge; ein neuer Stand baut die Vorschau neu auf. */
  stand: number;
}

/** Specs je Häppchen: das Backend prüft 4 zugleich, 8 ergeben zwei Runden je Antwort. */
const CHUNK = 8;

const EMPTY: CatalogState = { list: [], inDiff: new Set(), prompts: [], error: '', loading: true, stand: 0 };
let state = EMPTY;
let generation = 0;
let started = false;
let unsubscribeReconnect: (() => void) | null = null;
const subscribers = new Set<() => void>();

/** Erfolgreich geladene Specs und Frames (Fehler bleiben draußen, damit ein neuer Blick neu fragt). */
export const specCache = new Map<string, Spec>();
export const frameCache = new Map<string, Frame>();

function set(patch: Partial<CatalogState>) {
  state = { ...state, ...patch };
  subscribers.forEach((f) => f());
}

/** Gehört eine Antwort noch zum aktuellen Laden? */
export function currentGeneration() {
  return generation;
}

/** Lädt den Katalog einmal; `force` leert den Cache und holt alles neu. */
export function loadCatalog(force = false) {
  if (started && !force) return;
  started = true;
  unsubscribeReconnect ??= backend.on('ui:reconnect', () => loadCatalog(true));
  const gen = ++generation;
  const live = () => gen === generation;
  specCache.clear();
  frameCache.clear();
  set({ loading: true, error: '', stand: gen });
  assetsIndex().then(
    (list) => {
      if (!live()) return;
      set({ list, loading: false });
      void checkOpen(list.filter((e) => e.offen).map((e) => e.pfad), live);
    },
    (e) => live() && set({ error: errorText(e), loading: false }),
  );
  assetsPrompts().then((prompts) => live() && set({ prompts }), (e) => live() && set({ error: errorText(e) }));
  // ein Fehler des Reviews lässt die Liste stehen, nur ohne Abzeichen
  assetsReview('HEAD').then((r) => live() && set({ inDiff: new Set(r.assets.map((a) => a.id)) }), () => undefined);
}

/** Prüft die offenen Specs nacheinander in Häppchen und trägt jedes Ergebnis sofort ein. */
async function checkOpen(pfade: string[], live: () => boolean) {
  for (let i = 0; i < pfade.length; i += CHUNK) {
    const part = pfade.slice(i, i + CHUNK);
    try {
      const done = await assetsCheck(part);
      if (!live()) return;
      merge(new Map(done.map((e) => [e.pfad, e])));
    } catch (e) {
      if (!live()) return;
      const text = errorText(e);
      const fehler = [{ feld: '', regel: 'cli', text }];
      merge(new Map(state.list.filter((a) => part.includes(a.pfad)).map((a) => [a.pfad, { ...a, offen: false, gueltig: false, fehler }])));
    }
  }
}

/** Ersetzt die Prüffelder der Einträge unter ihren Pfaden; geändert-Markierung und Reihenfolge bleiben. */
function merge(byPath: Map<string, AssetEintrag>) {
  set({
    list: state.list.map((a) => {
      const e = byPath.get(a.pfad);
      return e ? { ...a, gueltig: e.gueltig, fehler: e.fehler, offen: false } : a;
    }),
  });
}

function subscribe(fn: () => void) {
  subscribers.add(fn);
  return () => subscribers.delete(fn);
}

/** Katalog aus dem Cache; lädt beim ersten Aufruf. `reload` holt auf Wunsch neu. */
export function useCatalogStore() {
  const s = useSyncExternalStore(subscribe, () => state);
  useEffect(() => loadCatalog(), []);
  const open = s.list.filter((a) => a.offen).length;
  return { ...s, open, reload: () => loadCatalog(true) };
}

/** Nur für Tests: Cache leeren, als wäre die Workbench neu gestartet. */
export function resetCatalogCache() {
  generation++;
  started = false;
  unsubscribeReconnect?.();
  unsubscribeReconnect = null;
  state = EMPTY;
  specCache.clear();
  frameCache.clear();
}
