// Alte EXE entfernter Programme in bin/ der Repo-Wurzel löschen (B-1007).

import { lstatSync, unlinkSync, type Stats } from 'node:fs';
import path from 'node:path';

/**
 * Die entfernten Programme (B-850, B-887; k3c-sim seit RU40.4; aufraeumen, regeltabelle und soundatlas seit RU71, sie
 * laufen als TypeScript). Nur diese Namen räumt das Werkzeug in bin/ ab. Bewusst fest: keine Muster, keine Eingaben.
 * Vorhandene Programme (k3c-workbench, k3c-studio) stehen nie hier.
 */
export const REMOVED_PROGRAMS: readonly string[] = [
  'k3c-server',
  'k3c-load',
  'k3c-tui',
  'k3c-starter',
  'k3c-workbench-web',
  'k3c-sim',
  'aufraeumen',
  'regeltabelle',
  'soundatlas',
];

/** Formen je Programm: ohne Endung, EXE, Sicherung des Linkers. */
const BIN_SUFFIXES = ['', '.exe', '.exe~'];

/** Dateinamen, die cleanBin anfassen darf (Name ohne Pfad). */
export function binCandidates(): string[] {
  return REMOVED_PROGRAMS.flatMap((p) => BIN_SUFFIXES.map((s) => p + s));
}

export interface BinOptions {
  dry: boolean;
  out: (text: string) => void;
  /** Löschen, im Test austauschbar (gesperrte Datei) */
  remove?: (file: string) => void;
}

/** Zustand ohne einem Link zu folgen; undefined, wenn nicht vorhanden. */
function linkStat(file: string): Stats | undefined {
  try {
    return lstatSync(file);
  } catch {
    return undefined;
  }
}

function cleanBinFile(o: BinOptions, file: string, name: string): void {
  const info = linkStat(file);
  if (!info) return;
  if (!info.isFile()) {
    o.out(`🚫 bin/${name} übersprungen: keine reguläre Datei\n`);
    return;
  }
  const size = `${(info.size / (1 << 20)).toFixed(1)} MiB`;
  if (o.dry) {
    o.out(`🧹 bin/${name} (${size}) würde gelöscht (--dry)\n`);
    return;
  }
  try {
    (o.remove ?? unlinkSync)(file);
  } catch (e) {
    const reason = e instanceof Error ? e.message : String(e);
    o.out(`🚫 bin/${name} (${size}) nicht gelöscht, vermutlich in Benutzung oder gesperrt: ${reason}\n`);
    return;
  }
  o.out(`🧹 bin/${name} (${size}) gelöscht\n`);
}

/**
 * cleanBin löscht in binDir die alten Programme und zeigt sie mit o.dry nur an. Es trifft ausschließlich reguläre
 * Dateien direkt in binDir (kein Ordner, kein Link, kein Pfad außerhalb); gesperrte Dateien werden gemeldet, der Lauf
 * geht weiter. Fehlt binDir, ist das kein Fehler.
 */
export function cleanBin(o: BinOptions, binDir: string): void {
  const info = linkStat(binDir);
  if (!info) return;
  // lstat meldet auch eine Windows-Junction als Link, nicht als Ordner
  if (!info.isDirectory()) {
    o.out(`🚫 bin/ ist kein echter Ordner, übersprungen: ${binDir}\n`);
    return;
  }
  for (const name of binCandidates()) cleanBinFile(o, path.join(binDir, name), name);
}
