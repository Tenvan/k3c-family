/**
 * Alle Seiten, die auf der Landingpage erscheinen.
 * Neue Seite: `<name>.html` im Projektordner anlegen (wird automatisch gebaut), im Script `installPageChrome()`
 * aus src/core/shell.ts aufrufen (Home-Button + Rückweg) und hier eintragen. Siehe Regel in CLAUDE.md.
 */
export interface PageEntry {
  title: string;
  description: string;
  icon: string;
  /** Ziel-URL, relativ. Eine Funktion wird erst beim Öffnen ausgewertet. */
  href: string | (() => string);
  section: 'play' | 'test' | 'about';
  /** Große Hauptkachel */
  primary?: boolean;
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
    title: 'Gamepad-Test',
    description: 'Controller, B-Taste, Vollbild, Vibration, FPS · Bericht an den PC',
    icon: '🎮',
    href: 'gamepad-test.html',
    section: 'test',
  },
  {
    title: 'Testing',
    description: 'Test-Szenarien starten · 1 bis 4 Spieler mit Mock-Spielern (Go-Server nötig)',
    icon: '🧪',
    href: 'testing.html',
    section: 'test',
  },
  {
    title: 'Level-Betrachter',
    description: 'Seed und Biom wählen, das generierte Level ansehen · Warnungen der Prüfung (Go-Server nötig)',
    icon: '🗺️',
    href: 'leveltest.html',
    section: 'test',
  },
  {
    title: 'Unsere Aufstellung',
    description: 'Jede Rolle im Spiel mit ihrer Figur · Monarchen, Truppen, Gegner',
    icon: '🛡️',
    href: 'aufstellung.html',
    section: 'test',
  },
  {
    title: 'Alle Figuren',
    description: 'Alle Sprites aus LuizMelo und Gothicvania, auch ungenutzte',
    icon: '🧙',
    href: 'figuren.html',
    section: 'test',
  },
  {
    title: 'Alle Grafiken',
    description: 'Gewählte CC0-Packs für Gebäude, Ressourcen und Hintergründe · mit Urheber, Lizenz und Quelle',
    icon: '🏰',
    href: 'grafiken.html',
    section: 'test',
  },
  {
    title: 'Hörprobe',
    description: 'Kandidaten für Musik und Effekte anhören · nach Zustand und Ereignis, mit Quelle und Lizenz',
    icon: '🔊',
    href: 'soundtest.html',
    section: 'test',
  },
  {
    title: 'Lizenzen & Danksagung',
    description: 'Unsere Lizenz (nicht-kommerziell) · Grafiken, Software und ein großes Danke an alle Urheber',
    icon: '📜',
    href: 'lizenzen.html',
    section: 'about',
  },
];

export const SECTIONS: Record<PageEntry['section'], string> = {
  play: 'Spielen',
  test: 'Tests & Werkzeuge',
  about: 'Über das Spiel',
};
