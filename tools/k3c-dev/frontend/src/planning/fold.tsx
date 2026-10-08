import { useEffect, useState } from 'react';
import { loadText, savePref } from '../lib/prefs';

// Eingeklappte Projekte und Sprints der Planungsseite (B-364), gemerkt in den Prefs. Ein Ereignis hält alle Karten
// gleich, wenn „alle ein-/ausklappen“ die Liste auf einmal setzt.

const KEY = 'planning.fold';
const EVENT = 'k3c-dev:planning-fold';

function read(): Set<string> {
  try {
    return new Set(JSON.parse(loadText(KEY, '[]')) as string[]);
  } catch {
    return new Set();
  }
}

function write(ids: Set<string>): void {
  savePref(KEY, JSON.stringify([...ids]));
  window.dispatchEvent(new Event(EVENT));
}

/** Eingeklappt-Zustand einer Karte (Projekt- oder Sprint-ID) und Umschalter. */
export function useFold(id: string): [boolean, () => void] {
  const [folded, setFolded] = useState(() => read().has(id));
  useEffect(() => {
    const sync = () => setFolded(read().has(id));
    window.addEventListener(EVENT, sync);
    return () => window.removeEventListener(EVENT, sync);
  }, [id]);
  const toggle = () => {
    const ids = read();
    if (!ids.delete(id)) ids.add(id);
    write(ids);
  };
  return [folded, toggle];
}

/** Klappt die genannten Karten auf (Auswahl von außen: Sprint-Chip, Detail, Abhängigkeits-Link). */
export const unfold = (ids: string[]) => {
  const cur = read();
  const changed = ids.filter((id) => cur.delete(id)).length > 0; // filter statt some: alle löschen, nicht nur das erste
  if (changed) write(cur);
};

/**
 * Klick auf den Kopf einer Karte (Workbench-Spec § 1 › Einklappen): schaltet auf/zu und wählt aus. Knöpfe, Links und
 * Menüs im Kopf reichen den Klick nicht weiter.
 */
export function headClick(toggle: () => void, select: () => void) {
  return (e: React.MouseEvent) => {
    if ((e.target as HTMLElement).closest('button, a, input, [role="menuitem"], [role="checkbox"]')) return;
    toggle();
    select();
  };
}

/** Alle genannten Karten ein- oder ausklappen. */
export const foldAll = (ids: string[], fold: boolean) => write(fold ? new Set(ids) : new Set());

/** Pfeil vor dem Kartenkopf. */
export function FoldButton({ folded, onToggle, what }: { folded: boolean; onToggle: () => void; what: string }) {
  return (
    <button type="button" className="pl-fold" aria-expanded={!folded} aria-label={`${what} ${folded ? 'aufklappen' : 'einklappen'}`}
      onClick={(e) => { e.stopPropagation(); onToggle(); }}>
      {folded ? '▸' : '▾'}
    </button>
  );
}
