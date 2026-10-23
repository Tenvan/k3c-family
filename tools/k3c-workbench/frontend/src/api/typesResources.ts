// Typen der Seite Monitoring (Go: app_resources.go).

/** Eine (verdichtete) Probe des Ressourcen-Messgeräts (Go: ResPoint); t in Unix-Millisekunden, Bytes als Zahl. */
export interface ResPoint {
  t: number;
  heapLive: number;
  total: number;
  private: number;
  rss: number;
  childrenRss: number;
  children: number;
  goroutines: number;
  handles: number;
  gcCycles: number;
  /** Puffer-Messwerte in der Reihenfolge von `ResourceView.gauges`. */
  gaugeValues: number[];
}

/** Ein Puffer-Messwert (Konsole, Prüfläufe, Log-Dateien …); bytes: Wert in Bytes, sonst Zähler. */
export interface ResGauge {
  name: string;
  bytes: boolean;
}

/** Vorgang mit Summen: Tool-Aufruf (`tool:<name>`) oder Oberflächen-Aufruf (`ui:<Methode>`). */
export interface ResOp {
  name: string;
  calls: number;
  totalMs: number;
  maxMs: number;
  alloc: number;
  maxAlloc: number;
}

/** Steigung einer Leitgröße je Stunde über windowMin Minuten. */
export interface ResTrend {
  name: string;
  bytes: boolean;
  perHour: number;
  windowMin: number;
}

/** Kindprozess der Workbench mit Arbeitssatz. */
export interface ResProc {
  name: string;
  pid: number;
  rss: number;
}

/** Alles für die Seite Monitoring (Go: ResourceView); enabled falsch: kein Messgerät (Mock ohne Daten, alte EXE). */
export interface ResourceView {
  enabled: boolean;
  started: number;
  intervalS: number;
  windowMin: number;
  points: ResPoint[];
  gauges: ResGauge[];
  ops: ResOp[];
  trends: ResTrend[];
  children: ResProc[];
  profiles: string[];
}
