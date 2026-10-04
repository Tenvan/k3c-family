/**
 * Sound-Atlas (B-011): ein Audio-Sprite je Format statt vieler Dateien; `public/audio/atlas.json` nennt Dateien und Start/Dauer je Sound.
 * Formatwahl rein und an einer Stelle: erstes Format der Reihenfolge, das der Browser abspielt.
 */

export type Format = 'ogg' | 'mp3' | 'wav';

/** Reihenfolge laut Xbox-Messung (B-166, `docs/game-design.md`): ogg, mp3 und wav gehen, m4a nicht. */
export const FORMAT_ORDER: readonly Format[] = ['ogg', 'mp3', 'wav'];

const MIME: Record<Format, string> = { ogg: 'audio/ogg; codecs="vorbis"', mp3: 'audio/mpeg', wav: 'audio/wav' };

export interface Sprite {
  start: number;
  duration: number;
}
export interface Atlas {
  files: Partial<Record<Format, string>>;
  sprites: Record<string, Sprite>;
}

/** Rein: Datei des ersten unterstützten Formats aus `FORMAT_ORDER`, die der Atlas anbietet; keine → `undefined`. */
export function pickFile(files: Atlas['files'], canPlay: (mime: string) => boolean): string | undefined {
  for (const f of FORMAT_ORDER) {
    const file = files[f];
    if (file && canPlay(MIME[f])) return file;
  }
  return undefined;
}

/** Rein: prüft die JSON-Beschreibung; ungültige Sprites fallen weg, ohne Dateien → `undefined`. */
export function parseAtlas(raw: unknown): Atlas | undefined {
  const r = raw as { files?: Atlas['files']; sprites?: Record<string, Sprite> } | null;
  if (!r?.files || !r.sprites) return undefined;
  const sprites: Atlas['sprites'] = {};
  for (const [name, s] of Object.entries(r.sprites)) {
    if (Number.isFinite(s?.start) && Number.isFinite(s?.duration) && s.start >= 0 && s.duration > 0) sprites[name] = s;
  }
  return { files: r.files, sprites };
}
