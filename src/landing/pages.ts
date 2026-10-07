/**
 * Seiten der Landingpage: nur Spieler-Kacheln und eine kleine Kachel zur Entwicklerseite (B-335).
 * Neue Seite: `<name>.html` im Projektordner anlegen (wird automatisch gebaut), im Script `installPageChrome()`
 * aus src/core/shell.ts aufrufen (Home-Button + Rückweg) und hier eintragen, Werkzeug-Seiten in
 * `src/tools/devTiles.ts`. Siehe Regel in CLAUDE.md.
 */
export interface PageEntry {
  title: string;
  description: string;
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

export const PAGES: PageEntry[] = [
  {
    title: 'Weiterspielen',
    description: 'Lobby öffnen · letzten Spielstand wählen · speichert automatisch bei Tagesanbruch',
    icon: '👑',
    href: 'game.html',
    section: 'play',
    primary: true,
  },
  {
    title: 'Neues Spiel',
    description: 'Lobby öffnen · Oberwelt · frischer Spielstand mit eigenem Namen · bis zu 2 Spieler · alte Spielstände bleiben',
    icon: '🏰',
    href: () => newGameHref(),
    section: 'play',
  },
  {
    title: 'Online spielen',
    description: 'Lobby öffnen · Raum beitreten oder anlegen · mit Handy, Tablet oder PC, ein Monarch pro Gerät',
    icon: '🌐',
    href: 'game.html',
    section: 'play',
  },
  {
    title: 'Lizenzen & Danksagung',
    description: 'Unsere Lizenz (nicht-kommerziell) · Grafiken, Software und ein großes Danke an alle Urheber',
    icon: '📜',
    href: 'lizenzen.html',
    section: 'about',
  },
  {
    title: 'Entwicklung',
    description: 'Werkzeuge für Entwicklung, Performance und Balancing',
    icon: '🛠️',
    href: 'dev.html',
    section: 'about',
    small: true,
  },
];

export const SECTIONS: Record<PageEntry['section'], string> = {
  play: 'Spielen',
  about: 'Über das Spiel',
};
