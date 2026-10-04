/** Hörprobe (SO3.2): reine Logik ohne Browser, damit sie testbar bleibt (Liste, Gruppen, Crossfade-Kurve, Eingabe-Zuordnung). */
import { FORMAT_ORDER, pickFile, type Format } from '../audio/atlas';

export interface Candidate {
  gruppe: string;
  name: string;
  /** Pfad ohne Endung, relativ zum Seiten-Root (z. B. `audio/probe-a`) */
  datei: string;
  bus: 'music' | 'sfx';
  quelle: string;
  lizenz: string;
}

/** Zustände der Musik (Q16), danach die Ereignisse (Q08); Gruppen außerhalb der Liste kommen am Ende. */
export const ZUSTAENDE = ['Tag', 'Abend', 'Nacht', 'Kampf', 'Tiefe/Höhle', 'Boss', 'Lobby', 'Niederlage/Sieg'] as const;
export const EREIGNISSE = ['Treffer', 'Kill', 'Münze', 'Pfeil', 'Schlag', 'Bau', 'Tod', 'Wiederbeleben', 'Skill', 'Nacht naht', 'Portal'] as const;

/** Rein: gültige Einträge der JSON-Liste, kaputte fallen weg. */
export function parseCandidates(raw: unknown): Candidate[] {
  if (!Array.isArray(raw)) return [];
  const str = (v: unknown): v is string => typeof v === 'string' && v.length > 0;
  return raw.filter(
    (c): c is Candidate => !!c && str(c.gruppe) && str(c.name) && str(c.datei) && (c.bus === 'music' || c.bus === 'sfx') && str(c.quelle) && str(c.lizenz),
  );
}

export interface Group {
  title: string;
  kind: 'Zustand' | 'Ereignis';
  items: Candidate[];
}

/** Rein: Kandidaten nach Zustand, dann Ereignis gruppiert; leere Gruppen fehlen, Reihenfolge der Liste bleibt je Gruppe. */
export function groupCandidates(list: readonly Candidate[]): Group[] {
  const known: readonly string[] = [...ZUSTAENDE, ...EREIGNISSE];
  const titles = [...known, ...new Set(list.map((c) => c.gruppe).filter((g) => !known.includes(g)))];
  return titles
    .map((title) => ({
      title,
      kind: (ZUSTAENDE as readonly string[]).includes(title) ? ('Zustand' as const) : ('Ereignis' as const),
      items: list.filter((c) => c.gruppe === title),
    }))
    .filter((g) => g.items.length > 0);
}

/** Rein: Datei des ersten abspielbaren Formats (Reihenfolge wie im Audio-Kern); keins → `undefined`. */
export function fileFor(c: Candidate, canPlay: (mime: string) => boolean): string | undefined {
  const files: Partial<Record<Format, string>> = {};
  for (const f of FORMAT_ORDER) files[f] = `${c.datei}.${f}`;
  return pickFile(files, canPlay);
}

/** Rein: Auswahl um `delta` verschieben, an den Enden anhalten. */
export const moveSelection = (index: number, count: number, delta: number): number => (count <= 0 ? 0 : Math.min(count - 1, Math.max(0, index + delta)));

/** Rein: Lautstärke in 10er-Schritten, 0–100. */
export const stepVolume = (percent: number, delta: number): number => Math.min(100, Math.max(0, Math.round(percent / 10) * 10 + delta * 10));

/**
 * Rein: Crossfade-Kurven mit gleicher Leistung (cos/sin), `n` Stützpunkte für `setValueCurveAtTime`.
 * `out` startet bei 1 und endet bei 0, `in` umgekehrt; benachbarte Werte springen nie (kein Knacken).
 */
export function crossfadeCurves(n = 64): { out: Float32Array; in: Float32Array } {
  const out = new Float32Array(n);
  const inn = new Float32Array(n);
  for (let i = 0; i < n; i++) {
    const x = (i / (n - 1)) * (Math.PI / 2);
    out[i] = Math.cos(x);
    inn[i] = Math.sin(x);
  }
  return { out, in: inn };
}

export type Action = 'up' | 'down' | 'play' | 'stop' | 'fade' | 'volUp' | 'volDown';

/** Rein: Tastatur → Aktion (Pfeile wählen, Enter/Leertaste spielt, S stoppt, X überblendet, links/rechts oder +/− Lautstärke). */
export function keyAction(code: string): Action | undefined {
  const map: Record<string, Action> = {
    ArrowUp: 'up', ArrowDown: 'down', Enter: 'play', Space: 'play', KeyS: 'stop', KeyX: 'fade',
    ArrowRight: 'volUp', Equal: 'volUp', NumpadAdd: 'volUp', ArrowLeft: 'volDown', Minus: 'volDown', NumpadSubtract: 'volDown',
  };
  return map[code];
}

/** Standard-Mapping: 12/13 Steuerkreuz hoch/runter, 0 = A spielt, 2 = X stoppt, 3 = Y überblendet, 4/5 = LB/RB Lautstärke. B (1) bleibt frei. */
const PAD_BUTTONS: ReadonlyArray<readonly [number, Action]> = [[12, 'up'], [13, 'down'], [0, 'play'], [2, 'stop'], [3, 'fade'], [4, 'volDown'], [5, 'volUp']];

export interface PadState {
  buttons: boolean[];
  stickY: number;
}

/** Rein: Aktionen, die zwischen zwei Pad-Zuständen neu ausgelöst wurden (Flanke); linker Stick (Achse 1) zählt als Steuerkreuz. */
export function padActions(prev: PadState, cur: PadState): Action[] {
  const acts: Action[] = PAD_BUTTONS.filter(([i]) => cur.buttons[i] && !prev.buttons[i]).map(([, a]) => a);
  if (cur.stickY < -0.6 && prev.stickY >= -0.6) acts.push('up');
  if (cur.stickY > 0.6 && prev.stickY <= 0.6) acts.push('down');
  return acts;
}
