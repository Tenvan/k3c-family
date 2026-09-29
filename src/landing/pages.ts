/**
 * Alle Seiten, die auf der Landingpage erscheinen.
 * Neue Seite: `<name>.html` im Projektordner anlegen (wird automatisch gebaut), im Script `installPageChrome()`
 * aus src/core/shell.ts aufrufen (Home-Button + Rückweg) und hier eintragen. Siehe Regel in CLAUDE.md.
 */
export interface PageEntry {
  title: string;
  description: string;
  icon: string;
  /** Ziel-URL, relativ. Eine Funktion wird erst beim Öffnen ausgewertet (z.B. für Zufalls-Seeds). */
  href: string | (() => string);
  section: 'play' | 'test';
  /** Große Hauptkachel */
  primary?: boolean;
}

export const PAGES: PageEntry[] = [
  {
    title: 'Spiel starten',
    description: 'Oberwelt · Seed „k3c“ · bis zu 2 Spieler im Split-Screen',
    icon: '👑',
    href: 'game.html',
    section: 'play',
    primary: true,
  },
  {
    title: 'Zufälliges Level',
    description: 'Neue Welt mit zufälligem Seed',
    icon: '🎲',
    href: () => `game.html?seed=${Math.random().toString(36).slice(2, 8)}`,
    section: 'play',
  },
  {
    title: 'Online spielen',
    description: 'Mit Handy, Tablet oder PC im selben Raum · ein Monarch pro Gerät',
    icon: '🌐',
    href: 'game.html?online=familie',
    section: 'play',
  },
  {
    title: 'Höhle',
    description: 'Direkt in Tiefe 1 starten',
    icon: '🦇',
    href: 'game.html?depth=1',
    section: 'play',
  },
  {
    title: 'Mine',
    description: 'Direkt in Tiefe 2 starten',
    icon: '⛏️',
    href: 'game.html?depth=2',
    section: 'play',
  },
  {
    title: 'Gamepad-Test',
    description: 'Controller, B-Taste, Vollbild, Vibration, FPS · Bericht an den PC',
    icon: '🎮',
    href: 'gamepad-test.html',
    section: 'test',
  },
];

export const SECTIONS: Record<PageEntry['section'], string> = {
  play: 'Spielen',
  test: 'Tests & Werkzeuge',
};
