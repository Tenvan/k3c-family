/**
 * Seiten der Landingpage: nur Spieler-Kacheln und eine kleine Kachel zur Entwicklerseite (B-335).
 * Neue Seite: `<name>.html` im Projektordner anlegen (wird automatisch gebaut), im Script `installPageChrome()`
 * aus src/core/shell.ts aufrufen (Home-Button + Rückweg) und hier eintragen, Werkzeug-Seiten in
 * `src/tools/devTiles.ts`. Siehe Regel in CLAUDE.md.
 */
import { t } from '../core/texts';

type PageId = 'continue' | 'new' | 'online' | 'licenses' | 'dev';

export interface PageEntry {
  /** Sprachunabhängiger Schlüssel; Titel `landing.<id>` und Beschreibung `landing.<id>.desc` aus den Textdateien (B-369). */
  id: PageId;
  readonly title: string;
  readonly description: string;
  icon: string;
  /** Ziel-URL, relativ. Eine Funktion wird erst beim Öffnen ausgewertet. */
  href: string | (() => string);
  section: 'play' | 'about';
  /** Große Hauptkachel */
  primary?: boolean;
  /** Kleine, unauffällige Kachel (Zugang zur Entwicklerseite) */
  small?: boolean;
}

/** Start-URL „Neues Spiel“: der Server lehnt `fresh` auf vorhandene Stände ab, deshalb trägt jeder Start einen eigenen Namen (aus der Uhrzeit). */
export function newGameHref(now: number = Date.now()): string {
  return `game.html?fresh=1&save=neu-${now.toString(36)}`;
}

/** Texte erst beim Lesen, damit sie der aktuellen Sprache folgen. */
function page(id: PageId, rest: Omit<PageEntry, 'id' | 'title' | 'description'>): PageEntry {
  return {
    id,
    ...rest,
    get title() {
      return t(`landing.${id}`);
    },
    get description() {
      return t(`landing.${id}.desc`);
    },
  };
}

export const PAGES: PageEntry[] = [
  page('continue', { icon: '👑', href: 'game.html', section: 'play', primary: true }),
  page('new', { icon: '🏰', href: () => newGameHref(), section: 'play' }),
  page('online', { icon: '🌐', href: 'game.html', section: 'play' }),
  page('licenses', { icon: '📜', href: 'lizenzen.html', section: 'about' }),
  page('dev', { icon: '🛠️', href: 'dev.html', section: 'about', small: true }),
];

/** Abschnitte in Reihenfolge; Überschrift `landing.section.<id>`. */
export const SECTIONS: readonly PageEntry['section'][] = ['play', 'about'];
