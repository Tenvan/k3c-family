import { useEffect, useRef, useState } from 'react';
import { Flex, Switch, Text, TextField } from '../atoms';
import { parseEntry, stepText, valueKind } from '../lib/valueEdit';

export interface ValueFieldProps {
  /** Wert im Feld (Text); ändert er sich von außen, ersetzt er die Eingabe. */
  shown: string;
  /** Standardwert: bestimmt Art (Zahl, Ja/Nein, Text, JSON) und Prüfung. */
  base: unknown;
  /** `aria-label` des Felds. */
  label: string;
  /** Eingabe ist gültig und weicht von `shown` ab (Enter oder Verlassen). */
  onCommit: (value: unknown) => void;
}

/** Eingabefeld für einen Wert: prüft beim Tippen (rot mit Grund), Enter und Verlassen übernehmen, Esc verwirft, Pfeil
 *  hoch/runter ändern Zahlen um einen Schritt (Umschalt ×10); Ja/Nein-Werte sind ein Schalter. Alt+Pfeil springt zum Feld
 *  der Zeile darüber oder darunter (`data-value-field`). */
export function ValueField({ shown, base, label, onCommit }: ValueFieldProps) {
  const kind = valueKind(base);
  const [text, setText] = useState(shown);
  // zuletzt gesendet: Enter und das folgende Verlassen schicken denselben Wert nur einmal
  const sent = useRef<string | null>(null);
  useEffect(() => {
    setText(shown);
    sent.current = null;
  }, [shown]);
  const parsed = parseEntry(text, base);
  const bad = text !== shown && !parsed.ok;
  // `force` (Enter) sendet auch einen Text erneut, den das Backend abgelehnt hat; das Verlassen danach sendet ihn nicht doppelt
  const commit = (force = false) => {
    const t = text;
    if (t === shown || (!force && t === sent.current)) return;
    const p = parseEntry(t, base);
    if (!p.ok) return;
    sent.current = t;
    onCommit(p.value);
  };
  const jump = (el: HTMLElement, dir: 1 | -1) => {
    const all = [...(el.closest('[data-value-table]') ?? document).querySelectorAll<HTMLElement>('[data-value-field]')];
    all[all.indexOf(el) + dir]?.focus();
  };
  const onKey = (e: React.KeyboardEvent<HTMLInputElement>) => {
    const dir = e.key === 'ArrowUp' ? -1 : e.key === 'ArrowDown' ? 1 : 0;
    if (e.key === 'Enter') commit(true);
    else if (e.key === 'Escape') setText(shown);
    else if (dir && e.altKey) {
      e.preventDefault();
      jump(e.currentTarget, dir);
    } else if (dir && kind === 'number') {
      e.preventDefault();
      setText(stepText(text, Number(shown), base, dir === -1 ? 1 : -1, e.shiftKey));
    }
  };
  if (kind === 'boolean') {
    return (
      <Switch size="1" aria-label={label} checked={shown === 'true'} data-value-field onCheckedChange={(c) => onCommit(c)}
        onKeyDown={(e) => {
          if (e.altKey && (e.key === 'ArrowUp' || e.key === 'ArrowDown')) {
            e.preventDefault();
            jump(e.currentTarget, e.key === 'ArrowUp' ? -1 : 1);
          }
        }} />
    );
  }
  return (
    <Flex direction="column" gap="0">
      <TextField.Root size="1" aria-label={label} aria-invalid={bad} color={bad ? 'red' : undefined} value={text} data-value-field
        inputMode={kind === 'number' ? 'decimal' : undefined} spellCheck={false}
        onChange={(e) => setText(e.target.value)} onFocus={() => { sent.current = null; }} onBlur={() => commit()} onKeyDown={onKey} />
      {bad && !parsed.ok && <Text size="1" color="red" role="alert">{parsed.error}</Text>}
    </Flex>
  );
}
