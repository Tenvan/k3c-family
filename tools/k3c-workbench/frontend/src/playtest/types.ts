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

export interface PlaytestApi {
  list(): Promise<PlaytestSession[]>;
  plan(action: PlaytestAction, id: string): Promise<PlaytestPlan>;
  remove(action: PlaytestAction, id: string, confirm: boolean): Promise<PlaytestPlan>;
  devices(): Promise<PlaytestDevices>;
  start(options: PlaytestOptions): Promise<PlaytestStatus>;
  stop(): Promise<void>;
  status(): Promise<PlaytestStatus>;
  /** Nur der Mock: Ausgangsdaten wiederherstellen (Tests). */
  reset?(): void;
}
