// Programme vor dem Start im PATH auflösen. Ohne das sucht Windows ein bloßes `git` zuerst im Arbeitsordner des
// Kindprozesses; eine dort abgelegte git.exe liefe dann mit (Go lehnt das ebenfalls ab). Nur absolute PATH-Einträge,
// kein Start über eine Shell, daher unter Windows nur .exe und .com.

import { statSync } from 'node:fs';
import path from 'node:path';

function pathValue(env: NodeJS.ProcessEnv): string {
  const key = Object.keys(env).find((k) => k.toUpperCase() === 'PATH');
  return (key && env[key]) || '';
}

function isFile(file: string): boolean {
  try {
    return statSync(file).isFile();
  } catch {
    return false;
  }
}

/** resolveExe liefert den absoluten Pfad des Programms `name` aus dem PATH, sonst undefined. */
export function resolveExe(name: string, env: NodeJS.ProcessEnv = process.env): string | undefined {
  const names = process.platform === 'win32' ? [`${name}.exe`, `${name}.com`] : [name];
  for (const entry of pathValue(env).split(path.delimiter)) {
    const dir = entry.replaceAll('"', '');
    if (!path.isAbsolute(dir)) continue;
    for (const candidate of names) {
      const file = path.join(dir, candidate);
      if (isFile(file)) return file;
    }
  }
  return undefined;
}

/** notFound ist die Meldung, wenn ein Programm im PATH fehlt. */
export function notFound(name: string): string {
  return `exec: "${name}": Programm nicht im PATH gefunden`;
}
