import { useCallback, useEffect, useState } from 'react';
import { loadText, savePref } from '../lib/prefs';
import { playtestApi } from './api';
import type { PlaytestCapability, PlaytestJob, PlaytestModel, PlaytestSession, PlaytestTimeline } from './types';

export const message = (e: unknown) => (e instanceof Error ? e.message : String(e));
export const LANGS = ['de', 'en', 'auto'];
const NO_JOB: PlaytestJob = { running: false, id: '', step: '', done: 0, total: 0, message: '', error: '' };

/** Zeitleiste der Session: laden, Marker-Bilder ab- und anwählen. */
export function useTimeline(s: PlaytestSession, setError: (m: string) => void) {
  const [timeline, setTimeline] = useState<PlaytestTimeline | null>(null);
  const loadTimeline = useCallback(() => {
    if (s.running) return;
    playtestApi.timeline(s.id).then(setTimeline, (e) => setError(message(e)));
  }, [s.id, s.running, setError]);
  useEffect(() => {
    setTimeline(null);
    loadTimeline();
  }, [loadTimeline, s.stage]);

  // Alle Marker-Bilder auf einmal an- oder abwählen (nacheinander, damit session.json nie gleichzeitig geschrieben wird).
  const setAll = async (skip: boolean) => {
    try {
      for (const e of timeline?.entries ?? []) if (e.frame && e.nr && !!e.skip !== skip) await playtestApi.skipFrame(s.id, e.nr, skip);
    } catch (err) {
      setError(message(err));
    }
    loadTimeline();
  };
  const toggleSkip = (nr: number, skip: boolean) =>
    void playtestApi.skipFrame(s.id, nr, skip).then(() => loadTimeline(), (e) => setError(message(e)));
  return { timeline, loadTimeline, setAll, toggleSkip };
}

/** Auswerten (Transkript): Voraussetzungen, Lauf mit Fortschritt, Start und Abbrechen. */
export function useTranscribe(s: PlaytestSession, refresh: number, onDone: () => void, loadTimeline: () => void, setError: (m: string) => void) {
  const [cap, setCap] = useState<PlaytestCapability | null>(null);
  const [job, setJob] = useState(NO_JOB);
  const busy = job.running;

  useEffect(() => {
    playtestApi.check().then(setCap, (e) => setError(message(e)));
    playtestApi.transcribeStatus().then(setJob, () => undefined);
  }, [refresh, setError]);
  useEffect(() => {
    if (!busy) return undefined;
    const t = setInterval(() => void playtestApi.transcribeStatus().then((j) => {
      setJob(j);
      if (!j.running) {
        loadTimeline();
        onDone();
      }
    }, () => undefined), 1000);
    return () => clearInterval(t);
  }, [busy, loadTimeline, onDone]);

  const start = async (model: string, language: string) => {
    setError('');
    try {
      setJob(await playtestApi.transcribe(s.id, { model, language }));
      onDone();
      loadTimeline();
    } catch (e) {
      setError(message(e));
    }
  };
  const cancel = () => void playtestApi.transcribeCancel().catch((e) => setError(message(e)));
  return { cap, job, busy, mine: job.id === s.id, start, cancel };
}

/** Whisper-Modell und Sprache (gemerkt); ohne Wahl gilt das Standardmodell. */
export function useEvalSettings(refresh: number) {
  const [models, setModels] = useState<PlaytestModel[]>([]);
  const [model, setModel] = useState(() => loadText('playtest.model', ''));
  const [lang, setLang] = useState(() => loadText('playtest.language', 'de'));
  useEffect(() => {
    playtestApi.models().then(setModels, () => undefined);
  }, [refresh]);
  const chosen = models.find((m) => m.id === model) ?? models.find((m) => m.default) ?? models[0];
  const pickModel = (v: string) => { setModel(v); savePref('playtest.model', v); };
  const pickLang = (v: string) => { setLang(v); savePref('playtest.language', v); };
  return { models, chosen, lang, pickModel, pickLang };
}
