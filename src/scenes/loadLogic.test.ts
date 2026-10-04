import { describe, expect, it } from 'vitest';
import { barFill, errorText, progressText } from './loadLogic';

describe('loadLogic', () => {
  it('Fortschritt als Prozent, begrenzt', () => {
    expect(progressText(0.5)).toContain('50 %');
    expect(progressText(-1)).toContain('0 %');
    expect(progressText(2)).toContain('100 %');
    expect(progressText(NaN)).toContain('0 %');
  });

  it('Balkenbreite', () => {
    expect(barFill(0.25, 400)).toBe(100);
    expect(barFill(3, 400)).toBe(400);
  });

  it('Fehlermeldung nennt jeden Dateinamen einmal', () => {
    const t = errorText(['atlas/atlas-0.png', 'atlas/atlas.json', 'atlas/atlas-0.png']);
    expect(t.match(/atlas-0\.png/g)).toHaveLength(1);
    expect(t).toContain('atlas/atlas.json');
  });
});
