import { useState, type KeyboardEvent } from 'react';
import { iconOf, rankOptions } from './kindIcons';

/** Durchsuchbare Auswahlliste für Chunk-Art und Objekt (B-485/AC-01): ein Feld mit Emoji davor und einer großen Liste
 *  darunter, die alle Einträge zeigt. Tippen ändert den Wert sofort (freier Text bleibt erlaubt, die Prüfung nennt
 *  Unbekanntes) und stellt Treffer nach oben; nicht passende bleiben abgeblendet in der Liste. Klick, Enter oder die
 *  Pfeile wählen, Esc schließt. */
export function KindPicker({ label, value, options, onChange }: {
  label: string; value: string; options: readonly string[]; onChange: (value: string) => void;
}) {
  const [open, setOpen] = useState(false);
  const [query, setQuery] = useState(''); // Suchtext seit dem Öffnen; leer = alle in fester Reihenfolge
  const [active, setActive] = useState(-1);
  const ranked = rankOptions(options, query);

  const show = () => { if (!open) { setOpen(true); setQuery(''); setActive(-1); } };
  const pick = (next: string) => { onChange(next); setOpen(false); };
  const onKey = (e: KeyboardEvent<HTMLInputElement>) => {
    if (e.key === 'ArrowDown' || e.key === 'ArrowUp') {
      e.preventDefault();
      if (!open) { show(); return; }
      const n = ranked.length;
      const step = e.key === 'ArrowDown' ? 1 : -1;
      if (n > 0) setActive((i) => (i < 0 ? (step > 0 ? 0 : n - 1) : (i + step + n) % n));
    } else if (e.key === 'Enter' && open && active >= 0) {
      e.preventDefault();
      pick(ranked[active].value);
    } else if (e.key === 'Escape') setOpen(false);
  };

  return (
    <span className="wd-pick">
      <span className="wd-pick-icon" aria-hidden="true">{iconOf(value)}</span>
      <input className="wd-kind" role="combobox" aria-label={label} aria-expanded={open} autoComplete="off" value={value}
        onFocus={show} onClick={show} onBlur={() => setOpen(false)} onKeyDown={onKey}
        onChange={(e) => { show(); setQuery(e.target.value); setActive(rankOptions(options, e.target.value)[0]?.match ? 0 : -1); onChange(e.target.value); }} />
      {open && (
        <ul className="wd-pick-list" role="listbox" aria-label={`${label} Liste`} onMouseDown={(e) => e.preventDefault()}>
          {ranked.map((o, i) => (
            <li key={o.value} role="option" aria-selected={o.value === value} className={`${o.match ? '' : 'wd-dim'}${i === active ? ' wd-active' : ''}`.trim() || undefined}
              onClick={() => pick(o.value)}>{`${iconOf(o.value)} ${o.value}`}</li>
          ))}
        </ul>
      )}
    </span>
  );
}
