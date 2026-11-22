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
      transcribe: (id, o) => app.PlaytestTranscribe(id, o),
      models: () => app.PlaytestModels(),
      transcribeStatus: () => app.PlaytestTranscribeStatus(),
      transcribeCancel: () => app.PlaytestTranscribeCancel(),
      timeline: (id) => app.PlaytestTimeline(id),
      findings: (id) => app.PlaytestFindings(id),
      updateFinding: (id, fid, patch) => app.PlaytestFindingUpdate(id, fid, patch),
      skipFrame: (id, nr, skip) => app.PlaytestFrameSkip(id, nr, skip),
      visionInfo: () => app.PlaytestVisionInfo(),
      describe: (id, model, nrs) => app.PlaytestDescribe(id, model, nrs),
      describeStatus: () => app.PlaytestDescribeStatus(),
      describeCancel: () => app.PlaytestDescribeCancel(),
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

export const playtestApi: PlaytestApi = pick();

/** Adresse einer Datei für die Player; im Wails-Fenster gibt es keinen HTTP-Weg (nur Client-Server-Betrieb). */
export function mediaUrl(id: string, file: 'voice.wav' | 'video.mkv'): string {
  return `/api/dev/playtest/media/${encodeURIComponent(id)}/${file}`; // im Fenster liefert sie windowMediaHandler (app_playtest.go)
}

/** Adresse eines Marker-Bilds (frames/marker-<nr>.jpg); im Wails-Fenster ohne HTTP-Weg. */
export function frameUrl(id: string, nr: number): string {
  return `/api/dev/playtest/frame/${encodeURIComponent(id)}/${nr}`;
}
