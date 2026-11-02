import type { PlanSaved, WorldsView } from './types';

// Typen des Welten Editors für Welten und Inseln (WZ18.6, B-488/AC-05), getrennt von types.ts (Dateigröße).

/** Faktoren der Skalierung je Tiefe einer Insel (Go: DraftScaling, Feld depthScaling der Welt-Datei) */
export interface DraftScaling {
  hp: number;
  damage: number;
  speed: number;
}

/** Insel im Entwurf (Go: IslandDraft); from ist ihre id in der Datei (leer = neue Insel), Planung und Chunk-Anzahl gehen mit. */
export interface IslandDraft {
  from: string;
  id: string;
  name: string;
  depths: number[];
  depthScaling: DraftScaling;
  /** Planung und Chunk-Anzahl, nur wenn der Entwurf sie mitbringt (Antwort aus dem Chat); sonst bleibt die Datei, wie sie ist. */
  plannedChunks?: unknown;
  chunkCounts?: unknown;
}

/** Entwurf der gewählten Welt (Go: WorldDraft); die id der Welt bleibt (gleich dem Dateinamen). */
export interface WorldDraft {
  name: string;
  notes: string;
  islands: IslandDraft[];
  /** übersteuerte Basis-Werte je Basis-Datei, nur Abweichungen (B-682, welten.md § 1.1); fehlt = Datei bleibt, {} entfernt sie */
  overrides?: Overrides;
}

/** `overrides` einer Welt-Datei: Schlüssel = Basis-Datei ohne .json (buildings, waves …), darin dieselbe Struktur wie die Basis */
export type Overrides = Record<string, unknown>;

/** Ein Feld einer Basis-Datei (Go: WorldValueEntry): path wie im Override (Listen von Objekten je id bzw. Index); base ist
 *  null, wenn die Basis das Feld nicht nennt; override fehlt, wenn die gespeicherte Welt es erbt. */
export interface WorldValueEntry {
  file: string;
  path: string[];
  base: unknown;
  override?: unknown;
}

/** Werte der gewählten Welt (Go: WorldValuesView): Eltern-Welt (leer = Basis data/), Kind-Welten und Felder. */
export interface WorldValuesView {
  world: string;
  extends: string;
  children: string[];
  /** je Nachfolger, der nicht lädt oder ungültig ist, der Grund (B-693) */
  childErrors?: Record<string, string>;
  entries: WorldValueEntry[];
  error: string;
}

/** Feld eines Prüffehlers: an einer Insel id, name, depths, depthScaling, plan; an der Welt name, islands; leer = ohne Feld */
export type DraftField = 'id' | 'name' | 'depths' | 'depthScaling' | 'plan' | 'islands' | 'overrides' | '';

/** Prüfung eines Entwurfs (Go: WorldCheckView): island ist der Platz im Entwurf (-1 = Welt), preview der Inhalt der Datei,
 *  wie Speichern ihn schreiben würde. */
export interface WorldCheckView {
  ok: boolean;
  error: string;
  island: number;
  field: DraftField;
  preview: string;
  /** Nachfolger, die schon vorher ungültig waren oder nicht laden; blockieren das Speichern nicht (B-693) */
  warnings?: string[];
}

/** Prüfung einer Antwort des Modells (Go: WorldAnswerView): draft ist gesetzt, wenn die Welt lesbar ist (auch mit Prüffehler);
 *  ok nur, wenn sie außerdem die Prüfung des Ladens besteht. */
export interface WorldAnswerView {
  ok: boolean;
  error: string;
  draft: WorldDraft | null;
  /** was das Studio ergänzt hat, z. B. „Hafen ergänzt: …“ (B-686) */
  hint?: string;
}

/** Aufrufe für Welten und Inseln (Go: PlanWorldCheck, PlanWorldSave, PlanWorldCopy) */
export interface WorldDraftApi {
  /** Entwurf prüfen wie beim Laden der Welt, ohne zu schreiben */
  planWorldCheck(draft: WorldDraft): Promise<WorldCheckView>;
  /** Entwurf speichern: nur gültig, nur auf dem Stand aus planLoad (sonst stale, der Entwurf bleibt), format-stabil */
  planWorldSave(draft: WorldDraft, stand: string): Promise<PlanSaved>;
  /** Kopie der gewählten Welt mit neuem Namen anlegen (wird gewählt) */
  planWorldCopy(name: string): Promise<WorldsView>;
  /** Werte der gewählten Welt: Basiswert (Basis und Eltern-Welten), Override, Kind-Welten (B-682) */
  planWorldValues(): Promise<WorldValuesView>;
  /** Antwort des Modells prüfen und als Entwurf liefern; schreibt nichts */
  planWorldAnswer(text: string): Promise<WorldAnswerView>;
}
