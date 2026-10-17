/**
 * Kacheln der Entwicklerseite (`dev.html`, B-335): alle Werkzeug-Seiten, gegliedert nach Entwicklung, Performance und Balancing.
 * Die Landingpage (`src/landing/pages.ts`) zeigt nur Spieler-Kacheln und eine kleine Kachel hierher.
 * Neue Werkzeug-Seite: `<name>.html` anlegen, `installPageChrome()` aufrufen und hier eintragen (Regel in CLAUDE.md).
 */
import { t, textOf } from './texts';

export interface DevTile {
  /** Titel und Beschreibung stehen in den Textdateien (`tile.<name>.title`/`.desc`) und werden beim Lesen übersetzt */
  readonly title: string;
  readonly description: string;
  icon: string;
  /** Ziel `name.html` dieses Ordners; geöffnet über `openPage()`, mit `external` in einem neuen Fenster */
  href: string;
  /** Seite läuft außerhalb der Shell (z. B. `dm.html`, B-232) und öffnet sich in einem neuen Fenster bzw. Tab */
  external?: boolean;
}

export interface DevSection {
  id: 'dev' | 'perf' | 'balance';
  /** Aus der gewählten Sprache gelesen, nicht beim Laden des Moduls */
  readonly title: string;
  tiles: readonly DevTile[];
  /** Hinweis, solange der Abschnitt keine Kacheln hat */
  readonly empty: string;
}

/** Kachel mit Text aus `tile.<name>.title` und `tile.<name>.desc`; der Text wird erst beim Lesen aufgelöst. */
export function tile(name: string, icon: string, href: string, external?: true): DevTile {
  return {
    get title() {
      return textOf(`tile.${name}.title`, name);
    },
    get description() {
      return textOf(`tile.${name}.desc`, '');
    },
    icon,
    href,
    ...(external && { external }),
  };
}

const section = (id: DevSection['id'], tiles: readonly DevTile[]): DevSection => ({
  id,
  get title() {
    return t(`dev.section.${id}`);
  },
  tiles,
  get empty() {
    return t('dev.empty');
  },
});

export const DEV_SECTIONS: readonly DevSection[] = [
  section('dev', [
    tile('testing', '🧪', 'testing.html'),
    tile('level', '🗺️', 'leveltest.html'),
    tile('gamepad', '🎮', 'gamepad-test.html'),
    tile('aufstellung', '🛡️', 'aufstellung.html'),
    tile('figuren', '🧙', 'figuren.html'),
    tile('grafiken', '🏰', 'grafiken.html'),
    tile('sound', '🔊', 'soundtest.html'),
    tile('dm', '🎲', 'dm.html', true),
  ]),
  section('perf', [tile('monitor', '📈', 'monitor.html')]),
  section('balance', []),
];

/** Alle Kacheln der Entwicklerseite in Anzeigereihenfolge. */
export const DEV_TILES: readonly DevTile[] = DEV_SECTIONS.flatMap((s) => s.tiles);
