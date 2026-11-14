// Sammelaufruf der Render-CLI (B-856): `batch <schritte.json>` führt mehrere Befehle in einem Prozess aus, damit
// assets_set bun einmal statt zwölfmal startet. Gemeinsam für paper.ts und die Attrappe cli-stub.ts.
import { readFileSync } from 'node:fs';

/** Ergebnis eines Befehls wie in paper.ts › run: Exit-Code, stdout (ein JSON-Objekt oder leer), stderr. */
export type Lauf = { code: 0 | 1 | 2; stdout: string; stderr: string };

/**
 * Ein Schritt: `args` wie auf der Kommandozeile (`["frame", spec, "--animation", …]`). `stopp`: scheitert der Schritt,
 * werden alle folgenden übersprungen. `braucht`: Indizes früherer Schritte, die mit Exit 0 enden müssen, sonst wird
 * dieser Schritt übersprungen.
 */
export type Schritt = { args: string[]; stopp?: boolean; braucht?: number[] };

/** Antwort je Schritt: Exit-Code, die geparste Antwort (Exit 0 und 1) oder stderr (Exit 2); übersprungen ohne Lauf. */
export type SchrittErgebnis = { code: 0 | 1 | 2; antwort?: unknown; stderr?: string } | { uebersprungen: true };

/** Fachfremde Eingabe (kein JSON, falsche Form) → Exit 2 des ganzen Aufrufs. */
export class BatchFehler extends Error {}

/** Liest und prüft die Schritt-Liste aus der Datei. */
export function schritteAus(pfad: string): Schritt[] {
  let wert: unknown;
  try {
    wert = JSON.parse(readFileSync(pfad, 'utf8'));
  } catch (e) {
    throw new BatchFehler(`batch: Schritte nicht lesbar (${(e as Error).message})`);
  }
  if (!Array.isArray(wert)) throw new BatchFehler('batch: Schritte müssen eine Liste sein');
  return wert.map((s, i) => {
    const r = s as Schritt;
    const ok = r && Array.isArray(r.args) && r.args.length > 0 && r.args.every((a) => typeof a === 'string')
      && (r.braucht === undefined || (Array.isArray(r.braucht) && r.braucht.every((j) => Number.isInteger(j) && j >= 0 && j < i)));
    if (!ok) throw new BatchFehler(`batch: Schritt ${i} braucht args (Liste von Texten) und braucht nur frühere Indizes`);
    if (r.args[0] === 'batch') throw new BatchFehler(`batch: Schritt ${i} darf kein batch sein`);
    return r;
  });
}

/** Führt die Schritte nacheinander mit `lauf` aus (dieselbe Funktion wie der Einzelaufruf). */
export function batch(schritte: Schritt[], lauf: (args: string[]) => Lauf): SchrittErgebnis[] {
  const out: SchrittErgebnis[] = [];
  let abbruch = false;
  for (const s of schritte) {
    const fehlt = (s.braucht ?? []).some((j) => (out[j] as { code?: number }).code !== 0);
    if (abbruch || fehlt) {
      out.push({ uebersprungen: true });
      continue;
    }
    const r = lauf(s.args);
    out.push(r.code === 2 ? { code: 2, stderr: r.stderr } : { code: r.code, antwort: JSON.parse(r.stdout) });
    if (r.code !== 0 && s.stopp) abbruch = true;
  }
  return out;
}
