import { devRpc } from '@webkit/lib/devRpc';
import { backend } from '../api';
import { hasWails } from '../api/wails';
import { mockPlaytest } from './mock';
import type { PlaytestApi } from './types';

// Zugang der Seite Spieltest zu den Go-Methoden PlaytestList, PlaytestPlan und PlaytestDelete (app_playtest.go):
// Wails-Bindings im Fenster, Mock mit `?mock=1` und in Tests, sonst HTTP wie das übrige Backend.

function pick(): PlaytestApi {
  if (hasWails()) {
    const app = window.go!.main!.App!;
    return {
      list: () => app.PlaytestList(),
      plan: (action, id) => app.PlaytestPlan(action, id),
      remove: (action, id, confirm) => app.PlaytestDelete(action, id, confirm),
      devices: () => app.PlaytestDevices(),
      start: (o) => app.PlaytestStart(o),
      stop: () => app.PlaytestStop(),
      status: () => app.PlaytestStatus(),
      check: () => app.PlaytestCheck(),
      transcribe: (id) => app.PlaytestTranscribe(id),
      transcribeStatus: () => app.PlaytestTranscribeStatus(),
      transcribeCancel: () => app.PlaytestTranscribeCancel(),
      timeline: (id) => app.PlaytestTimeline(id),
    };
  }
  if (backend.mock) return mockPlaytest();
  const rpc = devRpc('k3c-workbench');
  return {
    list: () => rpc('PlaytestList'),
    plan: (action, id) => rpc('PlaytestPlan', action, id),
    remove: (action, id, confirm) => rpc('PlaytestDelete', action, id, confirm),
    devices: () => rpc('PlaytestDevices'),
    start: (o) => rpc('PlaytestStart', o),
    stop: () => rpc('PlaytestStop'),
    status: () => rpc('PlaytestStatus'),
    check: () => rpc('PlaytestCheck'),
    transcribe: (id) => rpc('PlaytestTranscribe', id),
    transcribeStatus: () => rpc('PlaytestTranscribeStatus'),
    transcribeCancel: () => rpc('PlaytestTranscribeCancel'),
    timeline: (id) => rpc('PlaytestTimeline', id),
  };
}

export const playtestApi: PlaytestApi = pick();

/** Adresse einer Datei für die Player; im Wails-Fenster gibt es keinen HTTP-Weg (nur Client-Server-Betrieb). */
export function mediaUrl(id: string, file: 'voice.wav' | 'video.mkv'): string | null {
  return hasWails() ? null : `/api/dev/playtest/media/${encodeURIComponent(id)}/${file}`;
}

/** Adresse eines Marker-Bilds (frames/marker-<nr>.jpg); im Wails-Fenster ohne HTTP-Weg. */
export function frameUrl(id: string, nr: number): string | null {
  return hasWails() ? null : `/api/dev/playtest/frame/${encodeURIComponent(id)}/${nr}`;
}
