import { devRpc } from '@webkit/lib/devRpc';
import { backend } from '../api';
import { hasTauri, tauriRpc } from '../api/tauri';
import { mockPlaytest } from './mock';
import type { PlaytestApi } from './types';

// Zugang der Seite Spieltest zu den Methoden PlaytestList, PlaytestPlan und PlaytestDelete (und den übrigen
// Playtest-Methoden): Tauri-Command im Fenster, Mock mit `?mock=1` und in Tests, sonst HTTP wie das übrige Backend.

/** Das Tauri-Fenster liefert Player- und Marker-Dateien über das eigene Protokoll `k3c`, sonst bleibt die Adresse relativ. */
const fileOrigin = (): string => (hasTauri() ? 'http://k3c.localhost' : '');

/** Dieselben Methoden über den Command `rpc` (Tauri) oder HTTP. */
function remote(rpc: <T>(method: string, ...args: unknown[]) => Promise<T>): PlaytestApi {
  return {
    list: () => rpc('PlaytestList'),
    plan: (action, id) => rpc('PlaytestPlan', action, id),
    remove: (action, id, confirm) => rpc('PlaytestDelete', action, id, confirm),
    devices: () => rpc('PlaytestDevices'),
    start: (o) => rpc('PlaytestStart', o),
    stop: () => rpc('PlaytestStop'),
    status: () => rpc('PlaytestStatus'),
    check: () => rpc('PlaytestCheck'),
    transcribe: (id, o) => rpc('PlaytestTranscribe', id, o),
    models: () => rpc('PlaytestModels'),
    transcribeStatus: () => rpc('PlaytestTranscribeStatus'),
    transcribeCancel: () => rpc('PlaytestTranscribeCancel'),
    timeline: (id) => rpc('PlaytestTimeline', id),
    findings: (id) => rpc('PlaytestFindings', id),
    updateFinding: (id, fid, patch) => rpc('PlaytestFindingUpdate', id, fid, patch),
    skipFrame: (id, nr, skip) => rpc('PlaytestFrameSkip', id, nr, skip),
    visionInfo: () => rpc('PlaytestVisionInfo'),
    describe: (id, model, nrs) => rpc('PlaytestDescribe', id, model, nrs),
    describeStatus: () => rpc('PlaytestDescribeStatus'),
    describeCancel: () => rpc('PlaytestDescribeCancel'),
  };
}

function pick(): PlaytestApi {
  if (hasTauri()) return remote(tauriRpc);
  if (backend.mock) return mockPlaytest();
  return remote(devRpc('k3c-workbench'));
}

export const playtestApi: PlaytestApi = pick();

/** Adresse einer Datei für die Player. */
export function mediaUrl(id: string, file: 'voice.wav' | 'video.mkv'): string {
  return `${fileOrigin()}/api/dev/playtest/media/${encodeURIComponent(id)}/${file}`;
}

/** Adresse eines Marker-Bilds (frames/marker-<nr>.jpg). */
export function frameUrl(id: string, nr: number): string {
  return `${fileOrigin()}/api/dev/playtest/frame/${encodeURIComponent(id)}/${nr}`;
}
