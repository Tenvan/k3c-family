import type { PlaytestFinding, PlaytestAction, PlaytestApi, PlaytestEntry, PlaytestJob, PlaytestFile, PlaytestPlan, PlaytestSession, PlaytestStatus } from './types';

// Mock ohne Wails-Laufzeit (`?mock=1`, Tests): drei Sessions mit allen Ständen; Plan und Löschen verhalten sich wie
// internal/playtest (Bestätigung nötig, laufende Aufnahme gesperrt, Sammelaktion nur für „ausgewertet“).

const file = (name: string, size: number): PlaytestFile => ({ name, size });

const MB = 1024 * 1024;

function seed(): PlaytestSession[] {
  const base = { startMs: Date.parse('2026-11-20T18:00:00'), running: false, hasVoice: true };
  return [
    { ...base, id: '2026-11-22-1', date: '2026-11-22', durationMs: 0, markers: 3, size: 12 * MB, stage: 'läuft', running: true,
      hasVideo: true, hasVoice: false, files: [file('session.json', 300), file('video.mkv', 12 * MB)] },
    { ...base, id: '2026-11-21-1', date: '2026-11-21', durationMs: 25 * 60_000, markers: 14, size: 480 * MB, stage: 'ausgewertet',
      hasVideo: true, files: [file('session.json', 300), file('voice.wav', 40 * MB), file('video.mkv', 440 * MB), file('transcript.json', 9000), file('findings.json', 2000)] },
    { ...base, id: '2026-11-20-1', date: '2026-11-20', durationMs: 42 * 60_000, markers: 6, size: 820 * MB, stage: 'aufgenommen',
      hasVideo: true, files: [file('session.json', 300), file('voice.wav', 70 * MB), file('video.mkv', 750 * MB), file('markers.jsonl', 900)] },
    { ...base, id: '2026-11-19-1', date: '2026-11-19', durationMs: 0, markers: 0, size: 100, stage: 'unvollständig', hasVideo: false,
      hasVoice: false, files: [file('video.mkv', 100)] },
  ];
}

function seedFindings(): PlaytestFinding[] {
  return [
    { id: 'f1', quote: 'Ich finde den Bauarbeiter nicht', ts: Date.parse('2026-11-20T18:00:05'), context: 'Insel 2 Stufe 3', type: 'Problem', prio: 'mittel',
      project: 'WZG', sprint: 'WZ65', selected: false },
    { id: 'f2', quote: 'Zu dunkel', ts: Date.parse('2026-11-20T18:00:30'), type: 'Idee', duplicate_of: 'B-1100', selected: false },
  ];
}

export function mockPlaytest(): PlaytestApi {
  let sessions = seed();
  const find = (id: string) => {
    const s = sessions.find((x) => x.id === id);
    if (!s) throw new Error('Session oder Datei nicht gefunden');
    if (s.running) throw new Error('Aufnahme läuft, Löschen gesperrt');
    return s;
  };
  const plan = async (action: PlaytestAction, id: string): Promise<PlaytestPlan> => {
    if (action === 'ausgewertet') {
      const files = sessions.filter((s) => s.stage === 'ausgewertet' && s.hasVideo)
        .map((s) => file(`${s.id}/video.mkv`, s.files.find((f) => f.name === 'video.mkv')?.size ?? 0));
      return { action, id: '', files, total: files.reduce((n, f) => n + f.size, 0) };
    }
    const s = find(id);
    const files = action === 'session' ? [file(`${s.id}/`, s.size)] : s.files.filter((f) => f.name === 'video.mkv');
    return { action, id, files, total: files.reduce((n, f) => n + f.size, 0) };
  };
  const idle: PlaytestStatus = { running: false, id: '', startMs: 0, encoder: '' };
  let status = idle;
  const noJob: PlaytestJob = { running: false, id: '', step: '', done: 0, total: 0, message: '', error: '' };
  let job = noJob;
  let findings: PlaytestFinding[] = seedFindings();
  const entries = (s: PlaytestSession): PlaytestEntry[] => {
    const done = s.stage === 'transkribiert' || s.stage === 'ausgewertet';
    const t0 = s.startMs;
    const out: PlaytestEntry[] = [{ kind: 'marker', ms: t0 + 5000, videoMs: 5000, text: 'Marker · Spieler 1 · Insel 2 Stufe 3', nr: 1 }];
    if (done) out.unshift({ kind: 'sprache', ms: t0 + 1000, videoMs: 1000, endMs: t0 + 3000, text: 'Hier fehlt der Turm.' });
    return out;
  };
  return {
    reset: () => {
      sessions = seed();
      status = idle;
      job = noJob;
      findings = seedFindings();
    },
    findings: async () => structuredClone(findings),
    updateFinding: async (_id, fid, patch) => {
      const f = findings.find((x) => x.id === fid);
      if (!f) throw new Error('Befund gibt es nicht');
      if (f.ticket) throw new Error(`Ticket ${f.ticket} ist schon angelegt`);
      Object.assign(f, patch);
      return structuredClone(f);
    },
    check: async () => ({ ready: true, ffmpeg: '9.0.2', whisper: true, model: 'models/whisper/ggml-large-v3-turbo-q5_0.bin', modelOk: true, modelMb: 547, message: '' }),
    models: async () => [
      { id: 'models/ggml-large-v3-turbo-german-q5_0.bin', name: 'ggml-large-v3-turbo-german-q5_0.bin', sizeMb: 547, default: true },
      { id: 'models/whisper/ggml-small.bin', name: 'ggml-small.bin', sizeMb: 466, default: false },
    ],
    transcribe: async (id) => {
      const s = find(id);
      if (!s.hasVoice) throw new Error('voice.wav fehlt, nichts zu transkribieren');
      sessions = sessions.map((x) => (x.id === id ? { ...x, stage: 'transkribiert', files: [...x.files, file('transcript.json', 4000)] } : x));
      job = { ...noJob, id };
      return job;
    },
    transcribeStatus: async () => job,
    transcribeCancel: async () => {
      throw new Error('keine Auswertung aktiv');
    },
    timeline: async (id) => {
      const s = find(id);
      return { id, captureMs: s.startMs, hasTranscript: s.stage === 'transkribiert' || s.stage === 'ausgewertet', entries: entries(s) };
    },
    devices: async () => ({
      mics: ['Raummikro (USB)', 'Mikrofon (Realtek)'],
      monitors: [{ index: 0, width: 2560, height: 1440, label: 'Monitor 1 (2560×1440)' }, { index: 1, width: 1920, height: 1080, label: 'Monitor 2 (1920×1080)' }],
      ffmpeg: true, ffmpegPath: 'C:/ffmpeg/bin/ffmpeg.exe', encoder: 'h264_amf 1080p30', message: '',
    }),
    start: async (o) => {
      if (status.running) throw new Error('Aufnahme läuft schon');
      if (!o.mic) throw new Error('kein Mikrofon gewählt');
      status = {
        running: true, id: '2026-11-23-1', startMs: Date.now(), encoder: o.voiceOnly ? '' : 'h264_amf 1080p30', voiceOnly: !!o.voiceOnly,
        stats: { voiceBytes: 320000, videoBytes: o.voiceOnly ? 0 : 1_600_000, voiceRate: [32000, 32000, 32000, 32000], voiceLevel: [12, 40, 65, 30], videoRate: o.voiceOnly ? [] : [90000, 120000, 70000, 100000], warn: '', noVideo: !!o.voiceOnly },
      };
      return status;
    },
    stop: async () => {
      if (!status.running) throw new Error('keine Aufnahme aktiv');
      status = idle;
    },
    status: async () => status,
    list: async () => structuredClone(sessions),
    plan,
    remove: async (action, id, confirm) => {
      const p = await plan(action, id);
      if (!confirm) throw new Error('Löschen nicht bestätigt');
      if (action === 'session') sessions = sessions.filter((s) => s.id !== id);
      else {
        const ids = action === 'video' ? [id] : sessions.filter((s) => s.stage === 'ausgewertet').map((s) => s.id);
        sessions = sessions.map((s) => (ids.includes(s.id)
          ? { ...s, hasVideo: false, size: s.size - (s.files.find((f) => f.name === 'video.mkv')?.size ?? 0), files: s.files.filter((f) => f.name !== 'video.mkv') }
          : s));
      }
      return p;
    },
  };
}
