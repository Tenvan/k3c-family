// Credits aus den CREDITS.md-Dateien (einzige Quelle, B-165): Parser und HTML-Ausgabe für lizenzen.html.
import { t } from './texts';
import grafik from '../../public/grafik/CREDITS.md?raw';
import sprites from '../../public/sprites/CREDITS.md?raw';

export interface Credit {
  /** Ordner unter public/<root>/ (eine Zeile kann mehrere nennen) */
  folders: string[];
  root: 'grafik' | 'sprites';
  title: string;
  author: string;
  license: string;
  source: string;
}

const COLUMNS: Record<string, string> = {
  ordner: 'folders', pack: 'title', werk: 'title', urheber: 'author', 'autor:innen': 'author', lizenz: 'license', quelle: 'source',
};

const cells = (line: string) => line.trim().replace(/^\||\|$/g, '').split('|').map((c) => c.trim());

/** Liest alle Tabellen mit Kopfzeile ab „Ordner“; Spalten werden über die Kopfzeile zugeordnet. */
export function parseCredits(markdown: string, root: Credit['root']): Credit[] {
  const lines = markdown.replace(/\r\n/g, '\n').split('\n');
  const out: Credit[] = [];
  for (let i = 0; i < lines.length; i++) {
    if (!lines[i]!.startsWith('| Ordner |')) continue;
    const keys = cells(lines[i]!).map((h) => COLUMNS[h.toLowerCase()]);
    for (let j = i + 2; lines[j]?.startsWith('|'); j++) {
      const row: Record<string, string> = {};
      cells(lines[j]!).forEach((c, n) => {
        if (keys[n]) row[keys[n]!] = c;
      });
      out.push({
        root,
        folders: (row.folders ?? '').split(',').map((f) => f.trim()).filter(Boolean),
        title: row.title ?? '',
        author: row.author ?? '',
        license: row.license ?? '',
        source: row.source ?? '',
      });
    }
  }
  return out;
}

export const CREDITS: readonly Credit[] = [...parseCredits(grafik, 'grafik'), ...parseCredits(sprites, 'sprites')];

const esc = (s: string) => s.replace(/&/g, '&amp;').replace(/</g, '&lt;').replace(/>/g, '&gt;').replace(/"/g, '&quot;');

/** CC-BY (Namensnennung Pflicht); CC BY-NC zählt nicht, die Projekt-Assets stehen nicht in den CREDITS. */
export const isCcBy = (c: Credit) => /\bCC[- ]BY\b(?!-)/i.test(c.license);

/** Tabelle aller Einträge als HTML (Werk, Urheber, Lizenz, Quelle); jeder Eintrag eine Zeile. */
export function renderCredits(credits: readonly Credit[]): string {
  const rows = credits.map((c) => {
    const link = /^https?:\/\//.test(c.source)
      ? `<a href="${esc(c.source)}" target="_blank" rel="noopener">${esc(c.source.replace(/^https?:\/\//, ''))}</a>`
      : esc(c.source);
    return `<tr${isCcBy(c) ? ' class="by"' : ''}><td>${esc(c.title)}</td><td>${esc(c.author)}</td><td>${isCcBy(c) ? `<strong>${esc(c.license)}</strong> (${t('credits.attribution')})` : esc(c.license)}</td><td>${link}</td></tr>`;
  });
  return `<table><thead><tr><th>${t('credits.work')}</th><th>${t('credits.author')}</th><th>${t('credits.license')}</th><th>${t('credits.source')}</th></tr></thead><tbody>${rows.join('')}</tbody></table>`;
}
