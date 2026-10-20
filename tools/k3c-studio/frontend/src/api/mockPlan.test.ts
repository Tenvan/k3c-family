import { describe, expect, it } from 'vitest';
import sizes from '../../../../../data/island-sizes.json';
import { MOCK_SIZES } from './mockPlan';
import { HINT, staleText } from './mockWorldDraft';

// Die Inselgrößen des Mock-Backends sind die aus data/island-sizes.json (B-485, WZ17.5), damit Tests nicht gegen alte
// Werte grün bleiben.
describe('Inselgrößen im Mock', () => {
  it('gleichen data/island-sizes.json', () => {
    expect(MOCK_SIZES).toEqual(sizes.sizes);
  });
});

// Die Texte des Mocks sind die aus Go (tools/k3c-studio/backend: staleError, planRegHint), damit Tests der Oberfläche den echten Text prüfen.
describe('Texte im Mock wie in Go', () => {
  it('Stale nennt die Welt-Datei der gewählten Welt, der REG-Hinweis ist der aus Go', () => {
    expect(staleText('nordmeer')).toBe('data/worlds/nordmeer.json geändert, bitte neu laden');
    expect(HINT).toBe('REG-Änderung: Commit mit Begründung, `task golden:update` nur bei gewollter Änderung');
  });
});
