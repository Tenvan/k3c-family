import { describe, expect, it } from 'vitest';
import sizes from '../../../../../data/island-sizes.json';
import { MOCK_SIZES } from './mockPlan';

// Die Inselgrößen des Mock-Backends sind die aus data/island-sizes.json (B-485, WZ17.5), damit Tests nicht gegen alte
// Werte grün bleiben.
describe('Inselgrößen im Mock', () => {
  it('gleichen data/island-sizes.json', () => {
    expect(MOCK_SIZES).toEqual(sizes.sizes);
  });
});
