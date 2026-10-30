// Typen des Gebäude-Planers (Go: app_hub*.go), ausgelagert aus types.ts (Dateigrenze 400 Zeilen).

/** Zelle der Matrix des Gebäude-Planers (Go: HubCell): verfügbar ab der Stufe der Zeile, dann mit Platz und Tiefe. */
export interface HubCell {
  available: boolean;
  /** Platz zur Hub-Mitte in Units; bei Mauerlinie und Tor je Seite (±) */
  offsetUnits: number;
  fromDepth: number;
  needsDeeper: boolean;
  /** nur in dieser Tiefe (Händler); -1 = jede Tiefe */
  onlyDepth: number;
}

/** Herkunft einer Zeile: Liste bzw. Block in hub.json */
export type HubSource = 'sites' | 'islandSites' | 'sitePlacement' | 'merchant';

/** Bauplatz der Matrix (Go: HubRow): Hub-Platz, Insel-Platz, Baugrund der Zone 1 (Mauer mit Turm) oder Händler. */
export interface HubRow {
  /** kind@offset wie room.PreviewSiteKey, Baugrund der Zone 1: zone1 */
  key: string;
  /** Art aus hub.json; wall (Baugrund), merchantBuy, merchantSell */
  kind: string;
  source: HubSource;
  /** Eintrag in seiner Liste (sites, islandSites; Baugrund 0); Händler 0 */
  index: number;
  hubLevel: number;
  /** Baugrund der Zone 1: 1; sonst 0 */
  line: number;
  bothSides: boolean;
  towerUnits: number;
  /** Streuung nach außen je Seed; 0 = fest */
  jitterUnits: number;
  /** je Hub-Stufe 1 … levels */
  cells: HubCell[];
}

/** Matrix Hub-Stufe × Bauplatz (Go: HubMatrix); levels = Anzahl hub.json › levels, keine feste Zahl. */
export interface HubMatrix {
  levels: number;
  rows: HubRow[];
}

/** Befund der Prüfung im Go-Backend (sim.ValidateHub) */
export interface HubFinding {
  rule: string;
  site: string;
  detail: string;
}

/** Stand des Gebäude-Planers (Go: HubPlanView): raw ist der Kandidat der Live-Vorschau; error nennt eine nicht lesbare
 *  oder ungültige Datei mit Pfad, dann ist die Matrix leer. */
export interface HubPlanView {
  path: string;
  raw: string;
  matrix: HubMatrix;
  findings: HubFinding[];
  notes: string[];
  error: string;
  /** Stand beider Dateien beim Laden (Schreibregel); leer bei einem geprüften Kandidaten */
  stand: HubStand;
  /** Arten, die als Platz hinzukommen dürfen (entscheidet die Prüfung im Go-Backend) */
  kinds: string[];
}

/** Stand von data/hub.json und Regeltabelle beim Laden (Go: HubStand, SHA-256) */
export interface HubStand {
  hub: string;
  rules: string;
}

/** Ergebnis des Speicherns (Go: HubSaveResult): stale = Datei extern geändert, nichts geschrieben; rulesError = hub.json
 *  gespeichert, Regeltabelle veraltet; notice = Hinweis auf die REG-Änderung. */
export interface HubSaveResult {
  saved: boolean;
  /** Stand bedeutet dasselbe wie die Datei: nichts geschrieben, keine REG-Änderung */
  unchanged: boolean;
  stale: string;
  findings: HubFinding[];
  error: string;
  rulesError: string;
  notice: string;
  stand: HubStand;
}
