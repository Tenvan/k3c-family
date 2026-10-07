/**
 * Kacheln der Entwicklerseite (`dev.html`, B-335): alle Werkzeug-Seiten, gegliedert nach Entwicklung, Performance und Balancing.
 * Die Landingpage (`src/landing/pages.ts`) zeigt nur Spieler-Kacheln und eine kleine Kachel hierher.
 * Neue Werkzeug-Seite: `<name>.html` anlegen, `installPageChrome()` aufrufen und hier eintragen (Regel in CLAUDE.md).
 */
export interface DevTile {
  title: string;
  description: string;
  icon: string;
  /** Ziel `name.html` dieses Ordners; geöffnet über `openPage()`, mit `external` in einem neuen Fenster */
  href: string;
  /** Seite läuft außerhalb der Shell (z. B. `dm.html`, B-232) und öffnet sich in einem neuen Fenster bzw. Tab */
  external?: boolean;
}

export interface DevSection {
  id: 'dev' | 'perf' | 'balance';
  title: string;
  tiles: readonly DevTile[];
  /** Hinweis, solange der Abschnitt keine Kacheln hat */
  empty: string;
}

export const DEV_SECTIONS: readonly DevSection[] = [
  {
    id: 'dev',
    title: 'Entwicklung',
    empty: 'Noch keine Aufrufe.',
    tiles: [
      { title: 'Testing', description: 'Test-Szenarien starten · 1 bis 4 Spieler mit Mock-Spielern (Go-Server nötig)', icon: '🧪', href: 'testing.html' },
      {
        title: 'Level-Betrachter',
        description: 'Seed und Biom wählen, das generierte Level ansehen · Warnungen der Prüfung (Go-Server nötig)',
        icon: '🗺️',
        href: 'leveltest.html',
      },
      { title: 'Gamepad-Test', description: 'Controller, B-Taste, Vollbild, Vibration, FPS · Bericht an den PC', icon: '🎮', href: 'gamepad-test.html' },
      { title: 'Unsere Aufstellung', description: 'Jede Rolle im Spiel mit ihrer Figur · Monarchen, Truppen, Gegner', icon: '🛡️', href: 'aufstellung.html' },
      { title: 'Alle Figuren', description: 'Alle Sprites aus LuizMelo und Gothicvania, auch ungenutzte', icon: '🧙', href: 'figuren.html' },
      {
        title: 'Alle Grafiken',
        description: 'Gewählte CC0-Packs für Gebäude, Ressourcen und Hintergründe · mit Urheber, Lizenz und Quelle',
        icon: '🏰',
        href: 'grafiken.html',
      },
      {
        title: 'Hörprobe',
        description: 'Kandidaten für Musik und Effekte anhören · nach Zustand und Ereignis, mit Quelle und Lizenz',
        icon: '🔊',
        href: 'soundtest.html',
      },
      {
        title: 'Dungeon Master',
        description: 'Laufende Räume beobachten und steuern: Zeit, Pause, Gold, Material, Welle, Tageszeit · neues Fenster (Dev-Mode des Servers)',
        icon: '🎲',
        href: 'dm.html',
        external: true,
      },
    ],
  },
  {
    id: 'perf',
    title: 'Performance',
    empty: 'Noch keine Aufrufe.',
    tiles: [
      {
        title: 'Monitor',
        description: 'Serverzustand: Ampel je Raum, Verläufe mit Perzentilen, Fehler-Zeitleiste (Go-Server mit K3C_STATUS_TOKEN)',
        icon: '📈',
        href: 'monitor.html',
      },
    ],
  },
  { id: 'balance', title: 'Balancing', empty: 'Noch keine Aufrufe.', tiles: [] },
];

/** Alle Kacheln der Entwicklerseite in Anzeigereihenfolge. */
export const DEV_TILES: readonly DevTile[] = DEV_SECTIONS.flatMap((s) => s.tiles);
