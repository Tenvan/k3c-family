import { describe, expect, it } from 'vitest';
import { formatMessage, parseFieldError, pruneSelection, splitPath, statusTone } from './git';

describe('git', () => {
  it('formatiert die Nachricht wie Go', () => {
    expect(formatMessage({ type: 'feat', scope: ' srv ', subject: ' Git ', body: '' })).toBe('feat(srv): Git');
    expect(formatMessage({ type: 'docs', scope: '', subject: 'x', body: 'Rumpf' })).toBe('docs: x\n\nRumpf');
  });

  it('trennt Feld und Grund eines Go-Fehlers', () => {
    expect(parseFieldError('subject: darf nicht leer sein')).toEqual({ field: 'subject', text: 'darf nicht leer sein' });
    expect(parseFieldError('nichts gestaged')).toEqual({ field: '', text: 'nichts gestaged' });
  });

  it('färbt Status und zerlegt Pfade', () => {
    expect(statusTone('A')).toBe('ok');
    expect(statusTone('?')).toBe('neutral');
    expect(splitPath('a/b/c.go')).toEqual({ dir: 'a/b/', name: 'c.go' });
    expect(splitPath('c.go')).toEqual({ dir: '', name: 'c.go' });
  });

  it('behält nur vorhandene Pfade in der Auswahl', () => {
    expect([...pruneSelection(new Set(['a', 'b']), [{ path: 'b', status: 'M' }])]).toEqual(['b']);
  });
});
