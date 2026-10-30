import type { HubRow } from '../api';

// Bearbeiten im Gebäude-Planer (B-462, WZ9.3): Änderungen am Kandidaten (Inhalt der hub.json als Objekt). Hier wird
// nur geändert, nichts geprüft: Matrix und Befunde kommen nach jeder Änderung aus dem Go-Backend (hubCheck).

/** Ein Platz in `sites` bzw. `islandSites`; weitere Felder der Datei bleiben, wie sie sind. */
export interface HubSiteJson {
  kind: string;
  offsetUnits: number;
  fromDepth?: number;
  needsDeeper?: boolean;
  hubLevel?: number;
  [key: string]: unknown;
}

/** Die Teile der hub.json, die der Planer ändert; der Rest bleibt unangetastet. */
export interface HubJson {
  sites: HubSiteJson[];
  islandSites: HubSiteJson[];
  sitePlacement: { zone1WallUnits: number; zone1TowerUnits: number; [key: string]: unknown };
  merchant: { buyOffsetUnits: number; sellOffsetUnits: number; onlyDepth: number; [key: string]: unknown };
  [key: string]: unknown;
}

export type SiteList = 'sites' | 'islandSites';
export type SiteField = 'hubLevel' | 'offsetUnits' | 'fromDepth' | 'needsDeeper';
export type MerchantField = 'buyOffsetUnits' | 'sellOffsetUnits' | 'onlyDepth';

/** Hinweis beim Entfernen eines Platzes (B-462 › Bearbeitbar). */
export const REMOVE_HINT = 'Alte Spielstände verlieren den entfernten Platz.';

export function parseHub(raw: string): HubJson {
  return JSON.parse(raw) as HubJson;
}

/** Kandidat als Text: zwei Leerzeichen; das Backend schreibt ihn format-stabil in die Form der Datei. */
export function candidateText(hub: HubJson): string {
  return JSON.stringify(hub, null, 2);
}

/** Setzt ein Feld eines Platzes; `fromDepth` 0 und `needsDeeper` false entfallen wie in der Datei. */
export function setSiteField(hub: HubJson, list: SiteList, index: number, field: SiteField, value: number | boolean): HubJson {
  const next = structuredClone(hub);
  const site = next[list][index];
  if (!site) return hub;
  if ((field === 'fromDepth' && value === 0) || (field === 'needsDeeper' && value === false)) delete site[field];
  else (site as Record<SiteField, number | boolean>)[field] = value;
  return next;
}

/** Baugrund der Zone 1: Mauer (Abstand zur Hub-Mitte je Seite). */
export function setZone1Wall(hub: HubJson, value: number): HubJson {
  const next = structuredClone(hub);
  next.sitePlacement.zone1WallUnits = value;
  return next;
}

/** Baugrund der Zone 1: Turm (Abstand zur Hub-Mitte je Seite, innen vor der Mauer). */
export function setZone1Tower(hub: HubJson, value: number): HubJson {
  const next = structuredClone(hub);
  next.sitePlacement.zone1TowerUnits = value;
  return next;
}

export function setMerchant(hub: HubJson, field: MerchantField, value: number): HubJson {
  const next = structuredClone(hub);
  next.merchant[field] = value;
  return next;
}

/** Hängt einen Platz einer bekannten Art an die Liste an. */
export function addSite(hub: HubJson, list: SiteList, kind: string, offsetUnits: number, hubLevel: number): HubJson {
  const next = structuredClone(hub);
  next[list] = [...(next[list] ?? []), { kind, offsetUnits, hubLevel }];
  return next;
}

export function removeSite(hub: HubJson, list: SiteList, index: number): HubJson {
  const next = structuredClone(hub);
  next[list] = next[list].filter((_, i) => i !== index);
  return next;
}

/** Der Platz einer Matrix-Zeile im Kandidaten, falls die Zeile ein Hub- oder Insel-Platz ist. */
export function siteOf(hub: HubJson, row: Pick<HubRow, 'source' | 'index'>): HubSiteJson | undefined {
  if (row.source !== 'sites' && row.source !== 'islandSites') return undefined;
  return hub[row.source]?.[row.index];
}

/** Zahl aus einem Eingabefeld; leer oder keine Zahl = keine Änderung. */
export function numberOf(text: string): number | null {
  if (text.trim() === '') return null;
  const n = Number(text);
  return Number.isFinite(n) ? n : null;
}

/** Ganze Zahl aus einem Eingabefeld (Stufe, Tiefe: in Go int); Kommazahl oder zu groß = keine Änderung. */
export function integerOf(text: string): number | null {
  const n = numberOf(text);
  return n !== null && Number.isSafeInteger(n) ? n : null;
}

/** Ändert die Art eines Platzes; die übrigen Felder bleiben (B-670, Inspektor). */
export function setSiteKind(hub: HubJson, list: SiteList, index: number, kind: string): HubJson {
  const next = structuredClone(hub);
  const site = next[list][index];
  if (!site) return hub;
  site.kind = kind;
  return next;
}

/** Verschiebt einen Platz in die andere Liste, dort hinten angehängt (neuer Index = bisherige Länge der Ziel-Liste). */
export function moveSite(hub: HubJson, from: SiteList, index: number, to: SiteList): HubJson {
  const site = hub[from][index];
  if (!site || from === to) return hub;
  const next = removeSite(hub, from, index);
  next[to] = [...(next[to] ?? []), structuredClone(site)];
  return next;
}
