// Fortschrittszeilen von gen:paper (B-740): eine Zeile je Schritt, Emoji-Präfix wie im Projekt-Log, Dauer am Ende.
import { appendFileSync, mkdirSync } from 'node:fs';
import { dirname } from 'node:path';

/** Nimmt eine fertige Zeile ohne Zeilenumbruch entgegen. */
export type Melde = (zeile: string) => void;

/** Dauer lesbar: unter 1 s in ms, darüber in Sekunden, ab 60 s mit Minuten. */
export function dauer(ms: number): string {
  if (ms < 1000) return `${Math.round(ms)} ms`;
  const s = ms / 1000;
  if (s < 60) return `${s.toFixed(1).replace('.', ',')} s`;
  return `${Math.floor(s / 60)} min ${Math.round(s % 60)} s`;
}

/** Misst einen Schritt und meldet ihn als `<emoji> <text> ✅ <dauer>`; bei einem Fehler `❌` und weiterwerfen. */
export function schritt<T>(melde: Melde | undefined, emoji: string, text: string, f: () => T): T {
  const t0 = performance.now();
  try {
    const r = f();
    melde?.(`${emoji} ${text} ✅ ${dauer(performance.now() - t0)}`);
    return r;
  } catch (e) {
    melde?.(`${emoji} ${text} ❌ ${dauer(performance.now() - t0)}`);
    throw e;
  }
}

/** Zeilen auf stderr (stdout bleibt das JSON-Ergebnis) und angehängt in eine Log-Datei. */
export function konsoleUndLog(logPfad: string): Melde {
  mkdirSync(dirname(logPfad), { recursive: true });
  return (zeile) => {
    process.stderr.write(zeile + '\n');
    appendFileSync(logPfad, `${new Date().toISOString()} ${zeile}\n`);
  };
}
