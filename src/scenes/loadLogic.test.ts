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
    const t = errorText(['grafik/a/sheet-0.png', 'grafik/a/manifest.json', 'grafik/a/sheet-0.png']);
    expect(t.match(/sheet-0\.png/g)).toHaveLength(1);
    expect(t).toContain('grafik/a/manifest.json');
  });
});
