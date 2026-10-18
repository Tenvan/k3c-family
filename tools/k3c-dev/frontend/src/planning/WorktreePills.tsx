import { createContext, useContext, useEffect, useRef, useState } from 'react';
import { backend, type WorktreeChange, type WorktreeDiff } from '../api';
import { errorText } from '../lib/errors';
import { Pill } from './Pill';
import { EMPTY_INDEX, marksFor, pillLabel, pillTitle, type WtIndex } from './wtPills';

/** Änderungen der Worktrees je Eintrag; die Planungsseite stellt sie bereit, ohne Provider gibt es keine Pills. */
export const WtContext = createContext<WtIndex>(EMPTY_INDEX);

/** Pill „⎇ Worktree“ bzw. „⎇ neu“ am Eintrag; Tooltip nennt Branch und Änderung. Ohne Änderung nichts. */
export function WtPills({ item, id }: { item: WorktreeChange['item']; id: string }) {
  const marks = marksFor(useContext(WtContext), item, id);
  if (marks.length === 0) return null;
  return <Pill tone="violet" title={pillTitle(marks)}>⎇ {pillLabel(marks)}</Pill>;
}

/** Workflow-Check der Worktrees: lädt beim Öffnen, bei `planning:changed` und wenn das Fenster den Fokus bekommt (Änderungen
 *  in einem Worktree meldet der Wächter der Wurzel nicht); nur die jüngste Antwort zählt. Ein Fehler liefert keine Pills,
 *  nur den Text. `loaded` wird wahr, sobald der erste Check beantwortet ist. */
export function useWorktreeDiffs(): { diffs: WorktreeDiff[]; error: string; loaded: boolean; reload: () => void } {
  const [diffs, setDiffs] = useState<WorktreeDiff[]>([]);
  const [error, setError] = useState('');
  const [loaded, setLoaded] = useState(false);
  const seq = useRef(0);
  const reload = () => {
    const mine = ++seq.current;
    const done = (fn: () => void) => { if (mine === seq.current) { fn(); setLoaded(true); } };
    backend.worktreeCheck().then(
      (d) => done(() => { setDiffs(d); setError(''); }),
      (e) => done(() => { setDiffs([]); setError(errorText(e)); }), // Wails reicht Go-Fehler als Text durch
    );
  };
  useEffect(() => {
    reload();
    window.addEventListener('focus', reload);
    const off = backend.on('planning:changed', reload);
    return () => { window.removeEventListener('focus', reload); off(); };
  }, []);
  return { diffs, error, loaded, reload };
}
