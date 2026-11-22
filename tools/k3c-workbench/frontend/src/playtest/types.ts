// Typen der Seite Spieltest, gespiegelt von internal/playtest (Go). Zeitstempel sind Unix-Millisekunden.

export type PlaytestStage = 'läuft' | 'unvollständig' | 'aufgenommen' | 'transkribiert' | 'ausgewertet';

/** Aufräum-Aktion: Video einer Session, ganze Session, Videos aller ausgewerteten Sessions. */
export type PlaytestAction = 'video' | 'session' | 'ausgewertet';

export interface PlaytestFile {
  name: string;
  size: number;
}

export interface PlaytestSession {
  id: string;
  date: string;
  startMs: number;
  durationMs: number;
  markers: number;
  size: number;
  stage: PlaytestStage;
  running: boolean;
  hasVideo: boolean;
  hasVoice: boolean;
  files: PlaytestFile[];
}

/** Was eine Aktion löschen würde (vor der Bestätigung) bzw. gelöscht hat. */
export interface PlaytestPlan {
  action: PlaytestAction;
  id: string;
  files: PlaytestFile[];
  total: number;
}

/** Mikrofone aus ffmpeg (DirectShow); `message` nennt den Grund, wenn ffmpeg fehlt. */
export interface PlaytestDevices {
  mics: string[];
  ffmpeg: boolean;
  message: string;
}

export interface PlaytestOptions {
  mic: string;
  /** ddagrab output_idx, 0-basiert. */
  monitor: number;
  gameArgs: string[];
}

export interface PlaytestStatus {
  running: boolean;
  id: string;
  startMs: number;
  encoder: string;
}

/** Ist die Auswertung möglich (ffmpeg ab 8 mit whisper-Filter, Modelldatei)? `message` nennt sonst, was zu tun ist. */
export interface PlaytestCapability {
  ready: boolean;
  ffmpeg: string;
  whisper: boolean;
  model: string;
  modelOk: boolean;
  /** Größe der Modelldatei in MB (0: fehlt). */
  modelMb: number;
  message: string;
}

/** Stand der Auswertung (Transkript, dann Bilder je Marker). */
export interface PlaytestJob {
  running: boolean;
  id: string;
  step: string;
  done: number;
  total: number;
  message: string;
  error: string;
}

/** Eintrag der Zeitleiste: ein Satz oder ein Marker; `videoMs` ist die Videozeit für den Sprung im Player. */
export interface PlaytestEntry {
  kind: 'sprache' | 'marker';
  ms: number;
  videoMs: number;
  endMs?: number;
  text?: string;
  nr?: number;
  frame?: string;
}

export interface PlaytestTimeline {
  id: string;
  captureMs: number;
  hasTranscript: boolean;
  entries: PlaytestEntry[];
}

/** Gefundene Modelldatei (K3C_WHISPER_MODEL, models/, models/whisper/); `id` gibt die Seite beim Auswerten zurück. */
export interface PlaytestModel {
  id: string;
  name: string;
  sizeMb: number;
  default: boolean;
}

/** Modell (leer: Standard) und Sprache (de, en, auto) der Auswertung. */
export interface PlaytestTranscribeOptions {
  model: string;
  language: string;
}

/** Befund aus findings.json (Vorschlag von Claude, Haken von Ralf). */
export interface PlaytestFinding {
  id: string;
  title?: string;
  quote: string;
  ts: number;
  frame?: string;
  context?: string;
  domain?: string;
  type?: string;
  prio?: string;
  project?: string;
  sprint?: string;
  duplicate_of?: string;
  selected: boolean;
  ticket?: string;
  error?: string;
}

/** Änderung von der Seite: Haken und Vorschlag. */
export interface PlaytestFindingPatch {
  selected?: boolean;
  domain?: string;
  type?: string;
  prio?: string;
  project?: string;
  sprint?: string;
}

export interface PlaytestApi {
  list(): Promise<PlaytestSession[]>;
  plan(action: PlaytestAction, id: string): Promise<PlaytestPlan>;
  remove(action: PlaytestAction, id: string, confirm: boolean): Promise<PlaytestPlan>;
  devices(): Promise<PlaytestDevices>;
  start(options: PlaytestOptions): Promise<PlaytestStatus>;
  stop(): Promise<void>;
  status(): Promise<PlaytestStatus>;
  check(): Promise<PlaytestCapability>;
  transcribe(id: string, options: PlaytestTranscribeOptions): Promise<PlaytestJob>;
  models(): Promise<PlaytestModel[]>;
  transcribeStatus(): Promise<PlaytestJob>;
  transcribeCancel(): Promise<void>;
  timeline(id: string): Promise<PlaytestTimeline>;
  findings(id: string): Promise<PlaytestFinding[]>;
  updateFinding(id: string, findingId: string, patch: PlaytestFindingPatch): Promise<PlaytestFinding>;
  /** Nur der Mock: Ausgangsdaten wiederherstellen (Tests). */
  reset?(): void;
}
