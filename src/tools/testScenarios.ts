/**
 * Test-Szenarien der Testseite (B-081): Daten plus eine reine Funktion für die Start-URL von `game.html`.
 * Die Parameter stammen aus B-082 (`?autostart=1&fresh=1&save=NAME&mock=N`); Mock-Spieler sind lokale Slots desselben Geräts.
 */
import { t } from './texts';

export interface Scenario {
  id: string;
  /** Aus der gewählten Sprache gelesen */
  readonly title: string;
  readonly description: string;
  /** Spieler an diesem Gerät, der erste ist echt, der Rest sind Mock-Spieler ohne Eingabe */
  players: number;
}

export const SCENARIOS: readonly Scenario[] = [1, 2, 3, 4].map((n) => ({
  id: `players-${n}`,
  get title() {
    return n === 1 ? t('test.sc.one.title') : t('test.sc.n.title', { n });
  },
  get description() {
    return n === 1 ? t('test.sc.one.desc') : t('test.sc.n.desc', { mocks: n - 1, layout: t(`test.layout.${n as 2 | 3 | 4}`) });
  },
  players: n,
}));

const MAX_SAVE = 32;
const PREFIX = 'test-';

/** Gültiger Spielstandname (`^[a-z0-9-]{1,32}$`) aus einer beliebigen Kennung; ohne brauchbare Zeichen `0`. */
export function saveName(nonce: string): string {
  const id = nonce.toLowerCase().replace(/[^a-z0-9]/g, '').slice(0, MAX_SAVE - PREFIX.length);
  return PREFIX + (id || '0');
}

/** Start-URL des Szenarios; `nonce` macht den Spielstand eindeutig (die Seite übergibt die Uhrzeit, Tests feste Werte). */
export function scenarioUrl(scenario: Scenario, nonce: string): string {
  const params = new URLSearchParams({ autostart: '1', fresh: '1', save: saveName(nonce), mock: String(scenario.players - 1) });
  return `game.html?${params.toString()}`;
}
