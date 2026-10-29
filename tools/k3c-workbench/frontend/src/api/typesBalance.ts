// Balancing (WZ28.4/28.5): Go-Typen aus internal/simrun (Spec, Info, ReportInfo) und internal/balance (Summary, Figures).

/** Auftrag eines Laufs (Go: simrun.Spec); `depth` 0 = Wald, fehlt = Ziel-Tiefe. */
export interface BalanceSpec {
  seeds?: number;
  /** Genau diese Seeds statt der ersten N (B-634); nicht zusammen mit `seeds`. */
  seedList?: string[];
  depth?: number;
  players?: number;
  bot?: string;
  grade?: string;
  /** Welt (BS11): spielt die ganze erste Insel statt einer Stufe; leer = eine Stufe. */
  world?: string;
  /** Dev-Hilfen wie im Stufen-Test (hilfen, voll, boss), nur mit Welt; fehlt = keine. */
  helps?: string;
  days?: number;
  vary?: string[];
  tune?: BalanceTune;
  label?: string;
  /** Name einer gesicherten Baseline als Vergleich (B-640). */
  baseline?: string;
  /** Je feste Variante ein Lauf gegen einen gemeinsamen Basis-Lauf (B-662): n + 1 Läufe, nicht mit `tune`. */
  perVariant?: boolean;
}

/** Gesicherter Basis-Lauf (Go: balance.BaselineInfo, reports/baseline/<name>/). */
export interface BalanceBaseline {
  name: string;
  saved: string;
  run: string;
  seeds: number;
  seedMode: string;
  bots: string[];
  worlds: string[];
  pass: boolean;
  frozen: string[];
  /** Auftrag des Laufs; fehlt bei Baselines ohne Spec. */
  spec?: BalanceSpec;
}

export type TuneMode = 'sweep' | 'search';

/** Automatisches Balancing (Go: simrun.Tune); `params` je `datei.json:pfad=min..max`, 0/fehlt = Vorgabe der CLI. */
export interface BalanceTune {
  mode: TuneMode;
  params: string[];
  steps?: number;
  budget?: number;
}

export type BalanceRunState = 'wartet' | 'läuft' | 'fertig' | 'abgebrochen' | 'fehler';

/** Ein Lauf der Warteschlange (Go: simrun.Info). */
export interface BalanceRun {
  id: string;
  label: string;
  where: string;
  state: BalanceRunState;
  phase: string;
  done: number;
  total: number;
  started: string;
  ended?: string;
  verdict: string;
  report: string;
  lines: string[];
  spec?: BalanceSpec;
}

/** Eintrag der Berichtsliste (Go: simrun.ReportInfo). */
export interface BalanceReportInfo {
  path: string;
  run: string;
  modified: string;
  seeds: number;
  pass: boolean;
  varianten?: string[];
  /** `balance` (balance-*.json) oder `tuning` (tuning-*.json). */
  kind: 'balance' | 'tuning';
  /** Bewertung aus reports/ratings.json (WZ28.13); fehlt = keine. */
  rating?: ReportRating;
  /** Label des Laufs (B-662); fehlt bei älteren Berichten. */
  label?: string;
  /** Auftrag des Laufs (B-662); fehlt bei älteren Berichten. */
  spec?: BalanceSpec;
  /** Lauf-ID (`run-3`, B-662); fehlt bei älteren Berichten. */
  runId?: string;
  /** Getroffene und alle Ziele des Berichts (`3/16 Ziele`); 0/0 bei `tuning`. */
  targetsPass: number;
  targetsTotal: number;
}

/** Bewertung eines Berichts (Go: simrun.Ratings); leer = keine. */
export type ReportRating = '' | 'gut' | 'schlecht' | 'unklar' | 'referenz';

/** Bewertetes Ziel (Go: balance.Entry = Verdict + Target + Pass). */
export interface BalanceEntry {
  measure: string;
  name: string;
  kind: string;
  players: number;
  bot: string;
  depth: number;
  days: number;
  grade?: string;
  lower: number | null;
  upper: number | null;
  margin?: number;
  rule: string;
  value: number;
  status: string;
  failSeeds?: string[];
  invalidSeeds?: string[];
  pass: boolean;
}

export interface BalanceResources {
  wood: number;
  stone: number;
  copper: number;
  iron: number;
  crystal: number;
}

export interface NightFigures {
  grade: string;
  night: number;
  runs: number;
  survival: number;
  dayIncome: number;
  dayExpense: number;
  nightIncome: number;
  nightExpense: number;
  resourceIn: BalanceResources;
  resourceOut: BalanceResources;
  idleShare: number;
}

export interface BalanceFigures {
  runs: number;
  firstWallTickMedian: number | null;
  firstWorkshopTickMedian: number | null;
  nights: NightFigures[];
}

/** Bericht eines Laufs (Go: balance.Summary). */
export interface BalanceSummary {
  version: number;
  seeds: number;
  pass: boolean;
  targets: BalanceEntry[];
  varianten?: string[];
  kennzahlen?: BalanceFigures;
}

/** Abgestimmter Wert (Go: balance.TuneParam). */
export interface TuneParam {
  file: string;
  path: string;
  min: number;
  max: number;
  base: number;
  int: boolean;
}

/** Ausgewerteter Punkt (Go: balance.TunePoint); `values` je Parameter in der Reihenfolge von `params`. */
export interface TunePoint {
  values: number[];
  score: number;
  pass: boolean;
  failed: string[];
}

/** Ergebnis eines Abstimmens, tuning-*.json (Go: balance.Tuning). */
export interface Tuning {
  version: number;
  mode: TuneMode;
  params: TuneParam[];
  points: TunePoint[];
  best: { values: number[]; score: number; pass: boolean };
  reason: string;
  diff: string;
}

/** Datei in data/, die das Balancing beeinflusst (Go: datadoc.File, WZ28.10); Zweck und Werte aus docs/data/<doc>.md. */
export interface BalanceDataFile {
  name: string;
  doc: string;
  readBy: string[];
  purpose: string;
  key: string;
  /** Zahlenwerte als Variante nutzbar (nur Dateien direkt unter data/, die die Simulation liest). */
  vary: boolean;
  missing: boolean;
}

/** Zahlenwert einer Datei mit Pfad `a.b.0.c` (Go: datadoc.Value). */
export interface BalanceDataValue {
  path: string;
  value: number;
}

/** Zahlenwert mit Datei und Pfad aus allen variierbaren Dateien (Go: datadoc.FileValue, B-662). */
export interface BalanceDataFileValue {
  file: string;
  path: string;
  value: number;
}

/** Ampel der Auswertung (B-632): grün ok, gelb knapp, rot verletzt, grau ungültig/ohne Messwert. */
export type Light = 'green' | 'yellow' | 'red' | 'grey';

/** Hebel in data/ (Go: balance.Lever); `step` relativ (0.1 = 10 %). */
export interface Lever { path: string; direction: '+' | '-'; step: number; label: string }

/** Ziel mit Ampel (Go: balance.TargetInsight); `distance` in % der Korridorbreite, negativ = außerhalb. */
export interface TargetInsight {
  name: string; measure: string; kind: string; unit: string; grade: string; scenario: string;
  value: number; lower: number | null; upper: number | null; status: string; light: Light;
  distance: number | null; trend: { before: number; delta: number; better: boolean; beforeStatus: string } | null;
  failSeeds: string[] | null; invalidSeeds: string[] | null; rule: string; levers: Lever[];
}

/** Kennzahl mit Ampel (Go: balance.FigureLight). */
export interface FigureLight { key: string; name: string; unit: string; grade?: string; value: number | null; light: Light; text: string }

export interface InsightHint { target: string; light: Light; text: string; why: string; levers: Lever[] }

/** Kopierbarer Befehl; `where`: Chat an Claude, Terminal oder Workbench (Go: balance.Command). */
export interface InsightCommand { label: string; where: 'chat' | 'shell' | 'workbench'; text: string }

/** Auswertung eines Berichts (Go: balance.Insights, B-632). */
export interface BalanceInsights {
  title: string; run: string; generated: string; seeds: number; pass: boolean;
  counts: Record<Light, number>;
  summary: string[]; strengths: string[]; weaknesses: string[];
  targets: TargetInsight[];
  figures: { note: string; bands: Record<string, [number | null, number | null]>; lights: FigureLight[]; nights: NightFigures[] };
  hints: InsightHint[];
  next: { steps: string[]; seeds: string[]; commands: InsightCommand[] };
  baseline: { present: boolean; note: string; changes: { name: string; before: string; after: string; tipped: boolean }[] };
  varianten: string[];
}

export interface BalanceBackend {
  balanceStart(spec: BalanceSpec): Promise<BalanceRun>;
  balanceRuns(): Promise<BalanceRun[]>;
  balanceStop(id: string): Promise<void>;
  balanceReports(): Promise<BalanceReportInfo[]>;
  /** Gesicherte Basis-Läufe (B-640), nach Name. */
  balanceBaselines(): Promise<BalanceBaseline[]>;
  balanceReport(path: string): Promise<BalanceSummary>;
  balanceTuning(path: string): Promise<Tuning>;
  balanceDataFiles(): Promise<BalanceDataFile[]>;
  balanceDataDoc(name: string): Promise<string>;
  balanceDataValues(name: string): Promise<BalanceDataValue[]>;
  /** Zahlenwerte aller variierbaren Dateien in einem Aufruf (B-662). */
  balanceDataValuesAll(): Promise<BalanceDataFileValue[]>;
  balanceReportRate(path: string, rating: ReportRating): Promise<void>;
  balanceReportDelete(path: string): Promise<void>;
  /** Mehrere Berichte bewerten (B-662); ein Pfad außerhalb reports/ lehnt den ganzen Aufruf ab. */
  balanceReportsRate(paths: string[], rating: ReportRating): Promise<void>;
  /** Mehrere Berichte samt .md und Bewertung löschen (B-662); ein fremder Pfad lehnt alles ab, nichts wird gelöscht. */
  balanceReportsDelete(paths: string[]): Promise<void>;
  /** Durchgelaufenen Bericht reports/<lauf>/balance-*.json als Baseline sichern, wie sim_test {action: baseline}. */
  balanceSaveBaseline(path: string, name: string): Promise<BalanceBaseline>;
  /** Auswertung eines balance-*.json (B-632). */
  balanceInsights(path: string): Promise<BalanceInsights>;
  /** Schreibt .md und .html neu und öffnet den HTML-Bericht; Ergebnis: Pfad relativ zum Checkout. */
  balanceReportHTML(path: string): Promise<string>;
}
