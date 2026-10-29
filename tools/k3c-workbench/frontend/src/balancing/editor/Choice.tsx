import type { ReactNode } from 'react';
import { SegmentedControl } from '@webkit/atoms';

export interface ChoiceOption {
  value: string;
  label: ReactNode;
  /** Der Eintrag bleibt sichtbar, wählt aber nicht (Radix kennt kein `disabled` je Eintrag). */
  disabled?: boolean;
  title?: string;
}

interface Props {
  /** Name der Gruppe (`aria-label`). */
  label: string;
  value: string;
  options: readonly ChoiceOption[];
  onChange: (value: string) => void;
  size?: '1' | '2';
}

// Radix lässt keinen leeren Wert an einem Eintrag zu; "" (z. B. „keine“) läuft intern unter diesem Schlüssel.
const EMPTY = '__leer';
const key = (v: string) => (v === '' ? EMPTY : v);

/** Umschalter (Segmented Control) statt Auswahlliste für höchstens fünf feste Werte; jeder Eintrag trägt `data-value`. */
export function Choice({ label, value, options, onChange, size = '2' }: Props) {
  const pick = (k: string) => {
    const hit = options.find((o) => key(o.value) === k);
    if (hit && !hit.disabled) onChange(hit.value);
  };
  return (
    <SegmentedControl.Root aria-label={label} value={key(value)} onValueChange={pick} size={size} className="bal-ed-choice">
      {options.map((o) => (
        <SegmentedControl.Item key={key(o.value)} value={key(o.value)} data-value={o.value} data-off={o.disabled || undefined} title={o.title}>
          {o.label}
        </SegmentedControl.Item>
      ))}
    </SegmentedControl.Root>
  );
}
