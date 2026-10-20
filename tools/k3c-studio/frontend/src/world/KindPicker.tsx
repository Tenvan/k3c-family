import { useState, type KeyboardEvent } from 'react';
import { TextField } from '@webkit/atoms';
import { iconOf, nameOf, rankOptions, resolveKind } from './kindIcons';

/** Durchsuchbare Auswahlliste für Chunk-Art und Objekt (B-485/AC-01): ein Feld mit Emoji davor und einer großen Liste
 *  darunter, die alle Einträge zeigt. Angezeigt wird der deutsche Name, der Wert bleibt die Art (`data-value`). Tippen
 *  ändert den Wert sofort (ein deutscher Name wird zur Art, freier Text bleibt erlaubt, die Prüfung nennt Unbekanntes)
 *  und stellt Treffer nach Name oder Wert nach oben; nicht passende bleiben abgeblendet. Klick, Enter oder die Pfeile
 *  wählen, Esc schließt. */
export function KindPicker({ label, value, options, onChange }: {
  label: string; value: string; options: readonly string[]; onChange: (value: string) => void;
}) {
  const [open, setOpen] = useState(false);
  const [text, setText] = useState(''); // Text im Feld, solange die Liste offen ist
  const [typed, setTyped] = useState(false); // erst nach dem Tippen filtert der Text, sonst alle in fester Reihenfolge
  const [active, setActive] = useState(-1);
  const ranked = rankOptions(options, typed ? text : '');

  const show = () => { if (!open) { setOpen(true); setText(nameOf(value)); setTyped(false); setActive(-1); } };
  const type = (next: string) => {
    show();
    setText(next);
    setTyped(true);
    setActive(rankOptions(options, next)[0]?.match ? 0 : -1);
    onChange(resolveKind(next, options));
  };
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
      <TextField.Root className="wd-kind" role="combobox" aria-label={label} aria-expanded={open} autoComplete="off" data-value={value}
        value={open ? text : nameOf(value)} onFocus={(e) => { show(); e.target.select(); }} onClick={show} onBlur={() => setOpen(false)}
        onKeyDown={onKey} onChange={(e) => type(e.target.value)} />
      {open && (
        <ul className="wd-pick-list" role="listbox" aria-label={`${label} Liste`} onMouseDown={(e) => e.preventDefault()}>
          {ranked.map((o, i) => (
            <li key={o.value} role="option" aria-selected={o.value === value} className={`${o.match ? '' : 'wd-dim'}${i === active ? ' wd-active' : ''}`.trim() || undefined}
              onClick={() => pick(o.value)} title={o.value}>{`${iconOf(o.value)} ${nameOf(o.value)}`}</li>
          ))}
        </ul>
      )}
    </span>
  );
}
