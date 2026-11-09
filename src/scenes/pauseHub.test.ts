import { afterEach, describe, expect, it } from 'vitest';
import { setLanguage } from '../core/texts';
import { GAME_HEIGHT, GAME_WIDTH } from '../core/constants';
import { OPTION_IDS, SETTINGS_IDS } from './optionsLogic';
import { HUB_COLORS, HUB_LAYOUT, PAUSE_TILES, START_TILES, hubAction, hubChoice, hubDir, hubEnabled, hubLabel, hubSub, hubTileAt, moveHub, type HubTileId } from './pauseHub';

afterEach(() => setLanguage('de'));

const ids = PAUSE_TILES.map((tile) => tile.id);
const at = (id: HubTileId) => ids.indexOf(id);

describe('Lage und Lesbarkeit', () => {
  it('Fortsetzen ist die erste Kachel und die größte', () => {
    expect(PAUSE_TILES[0]!.id).toBe('resume');
    const area = (i: number) => PAUSE_TILES[i]!.w * PAUSE_TILES[i]!.h;
    expect(area(0)).toBeGreaterThan(Math.max(...[1, 2, 3, 4].map(area)) * 1.5);
  });

  it('alle Kacheln liegen im Bild, unter dem Titel und über dem Hinweis, ohne sich zu überlappen', () => {
    for (const tile of PAUSE_TILES) {
      expect(tile.x).toBeGreaterThanOrEqual(0);
      expect(tile.x + tile.w).toBeLessThanOrEqual(GAME_WIDTH);
      expect(tile.y).toBeGreaterThan(HUB_LAYOUT.worldY + HUB_LAYOUT.worldPx / 2);
      expect(tile.y + tile.h).toBeLessThan(HUB_LAYOUT.hintY - HUB_LAYOUT.hintPx);
      expect(tile.y + tile.h).toBeLessThanOrEqual(GAME_HEIGHT);
    }
    PAUSE_TILES.forEach((a, i) =>
      PAUSE_TILES.slice(i + 1).forEach((b) => {
        const apart = a.x + a.w <= b.x || b.x + b.w <= a.x || a.y + a.h <= b.y || b.y + b.h <= a.y;
        expect(apart).toBe(true);
      }),
    );
  });

  it('Schrift aus Sofa-Abstand: Beschriftung ≥ 52 px, Zeile darunter ≥ 32 px', () => {
    for (const tile of PAUSE_TILES) expect(tile.labelPx).toBeGreaterThanOrEqual(52);
    expect(HUB_LAYOUT.subPx).toBeGreaterThanOrEqual(32);
  });

  it('Kachelfarben nach Bereich: Gold, Lila, Türkis, Blau, Koralle, alle verschieden', () => {
    expect(PAUSE_TILES.map((tile) => tile.color)).toEqual([HUB_COLORS.gold, HUB_COLORS.purple, HUB_COLORS.teal, HUB_COLORS.blue, HUB_COLORS.coral]);
    expect(new Set(PAUSE_TILES.map((tile) => tile.color)).size).toBe(PAUSE_TILES.length);
  });
});

describe('Auswahl mit dem Pad', () => {
  it('jede Kachel ist mit D-Pad und Stick erreichbar', () => {
    const seen = new Set([0]);
    let frontier = [0];
    while (frontier.length > 0) {
      const next: number[] = [];
      for (const from of frontier)
        for (const dir of ['up', 'down', 'left', 'right'] as const) {
          const to = moveHub(PAUSE_TILES, from, dir);
          if (!seen.has(to)) {
            seen.add(to);
            next.push(to);
          }
        }
      frontier = next;
    }
    expect([...seen].sort()).toEqual(PAUSE_TILES.map((_, i) => i));
  });

  it('Richtungen führen zur Nachbarkachel, am Rand bleibt die Auswahl', () => {
    expect(moveHub(PAUSE_TILES, at('resume'), 'right')).toBe(at('options'));
    expect(moveHub(PAUSE_TILES, at('options'), 'left')).toBe(at('resume'));
    expect(moveHub(PAUSE_TILES, at('options'), 'right')).toBe(at('keys'));
    expect(moveHub(PAUSE_TILES, at('options'), 'down')).toBe(at('removePlayer'));
    expect(moveHub(PAUSE_TILES, at('keys'), 'down')).toBe(at('leave'));
    expect(moveHub(PAUSE_TILES, at('leave'), 'up')).toBe(at('keys'));
    expect(moveHub(PAUSE_TILES, at('resume'), 'up')).toBe(at('resume'));
    expect(moveHub(PAUSE_TILES, at('leave'), 'down')).toBe(at('leave'));
    expect(moveHub(PAUSE_TILES, at('keys'), 'right')).toBe(at('keys'));
  });
});

describe('Aktionen', () => {
  it('jede Kachel löst ihre Aktion aus', () => {
    expect(ids.map((id) => hubAction(id, [0]))).toEqual(['close', 'settings', 'keys', 'removePlayer', 'leave']);
  });

  it('„Spieler abmelden“ ist ohne Slot ausgegraut, die anderen nie', () => {
    expect(hubEnabled('removePlayer', [])).toBe(false);
    expect(hubAction('removePlayer', [])).toBeNull();
    for (const id of ids.filter((i) => i !== 'removePlayer')) expect(hubEnabled(id, [])).toBe(true);
  });

  it('Richtung des Frames: Hoch/Runter vor Links/Rechts, Bestätigen ist keine Richtung', () => {
    expect(hubDir(-1, 'left')).toBe('up');
    expect(hubDir(1, null)).toBe('down');
    expect(hubDir(0, 'right')).toBe('right');
    expect(hubDir(0, 'confirm')).toBeNull();
    expect(hubDir(0, null)).toBeNull();
  });

  it('Tippen trifft die Kachel unter dem Punkt, daneben nichts', () => {
    for (const tile of PAUSE_TILES) expect(hubTileAt(PAUSE_TILES, tile.x + tile.w / 2, tile.y + tile.h / 2)).toBe(at(tile.id));
    expect(hubTileAt(PAUSE_TILES, 5, 5)).toBe(-1);
  });

  it('Links/Rechts wählt bei zwei Spielern das Ziel, sonst die Kachel', () => {
    expect(hubChoice('removePlayer', 'right', 0, 2)).toBe(1);
    expect(hubChoice('removePlayer', 'left', 1, 2)).toBe(0);
    expect(hubChoice('removePlayer', 'left', 0, 2)).toBeNull(); // am Rand: bewegt die Auswahl
    expect(hubChoice('removePlayer', 'right', 0, 1)).toBeNull();
    expect(hubChoice('removePlayer', 'down', 0, 2)).toBeNull();
    expect(hubChoice('options', 'right', 0, 2)).toBeNull();
  });
});

describe('Texte', () => {
  it('jede Kachel hat Beschriftung und Zeile, deutsch und englisch', () => {
    for (const lang of ['de', 'en']) {
      setLanguage(lang);
      for (const id of ids) {
        expect(hubLabel(id).length).toBeGreaterThan(0);
        expect(hubSub(id, [0], 0).length).toBeGreaterThan(0);
      }
    }
  });

  it('bei mehreren Zielen nennt die Zeile den gewählten Spieler', () => {
    expect(hubSub('removePlayer', [0, 1], 1)).toContain('2');
    expect(hubSub('removePlayer', [0], 0)).not.toContain('Links/Rechts');
  });

  it('Optionen-Seite: nur Regler, Schalter und Sprache; die Kacheln der Kachelseite fehlen dort', () => {
    expect(SETTINGS_IDS).toEqual(['music', 'sfx', 'ambient', 'screenshake', 'flash', 'colorblind', 'guide', 'tech', 'language']);
    for (const id of OPTION_IDS) expect(SETTINGS_IDS.includes(id) || ['keys', 'resume', 'removePlayer', 'leave'].includes(id)).toBe(true);
  });
});

describe('Startseite der Lobby', () => {
  const ids = START_TILES.map((tile) => tile.id);

  it('Fortsetzen groß, dazu Neues Spiel, Raum beitreten, Optionen, Seiten in den Farben des Produktkontexts', () => {
    expect(ids).toEqual(['resume', 'new', 'join', 'options', 'pages']);
    expect(START_TILES.map((tile) => tile.color)).toEqual([HUB_COLORS.gold, HUB_COLORS.green, HUB_COLORS.blue, HUB_COLORS.purple, HUB_COLORS.teal]);
    for (const tile of START_TILES) expect(tile.labelPx).toBeGreaterThanOrEqual(52);
  });

  it('gleiche Lage wie das Pausenmenü: dieselben Rechtecke, nur andere Einträge', () => {
    expect(START_TILES.map(({ x, y, w, h }) => [x, y, w, h])).toEqual(PAUSE_TILES.map(({ x, y, w, h }) => [x, y, w, h]));
  });

  it('jede Kachel ist per Pad erreichbar und löst ihre Aktion aus', () => {
    expect(moveHub(START_TILES, 0, 'right')).toBe(1);
    expect(moveHub(START_TILES, 1, 'right')).toBe(2);
    expect(moveHub(START_TILES, 1, 'down')).toBe(3);
    expect(moveHub(START_TILES, 2, 'down')).toBe(4);
    expect(moveHub(START_TILES, 3, 'up')).toBe(1);
    expect(ids.map((id) => hubAction(id, []))).toEqual(['close', 'new', 'join', 'settings', 'pages']);
  });

  it('Texte: Fortsetzen nennt den Spielstand, jede Kachel hat Beschriftung und Zeile (de, en)', () => {
    expect(hubSub('resume', [], 0, 'familie')).toContain('familie');
    for (const lang of ['de', 'en']) {
      setLanguage(lang);
      for (const id of ids) {
        expect(hubLabel(id).length).toBeGreaterThan(0);
        expect(hubSub(id, [], 0).length).toBeGreaterThan(0);
      }
    }
  });
});
