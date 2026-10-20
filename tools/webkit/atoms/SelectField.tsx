import { Select } from '@radix-ui/themes';
import type { ComponentProps, ReactNode } from 'react';

export interface SelectOption<T extends string> {
  value: T;
  label: ReactNode;
  disabled?: boolean;
}

type TriggerProps = Omit<ComponentProps<typeof Select.Trigger>, 'value' | 'onChange' | 'children'>;

interface Props<T extends string> extends TriggerProps {
  value: T;
  options: readonly SelectOption<T>[];
  onChange: (value: T) => void;
  disabled?: boolean;
  size?: ComponentProps<typeof Select.Root>['size'];
}

// Radix erlaubt keinen leeren Wert an einem Eintrag; "" (z. B. „Keine“) läuft intern unter diesem Platzhalter.
// Ohne solchen Eintrag bleibt "" leer, dann zeigt der Auslöser seinen `placeholder`.
const EMPTY = '\u0000leer';

/** Auswahlliste aus Einträgen (Radix Select) statt `<select>`; `aria-label`, `placeholder` und `className` gehen an den Auslöser. */
export function SelectField<T extends string>({ value, options, onChange, disabled, size = '2', ...trigger }: Props<T>) {
  const hasEmpty = options.some((o) => o.value === '');
  const toRadix = (v: string) => (v === '' && hasEmpty ? EMPTY : v);
  const pick = (v: string) => {
    const hit = options.find((o) => toRadix(o.value) === v);
    if (hit) onChange(hit.value);
  };
  return (
    <Select.Root value={toRadix(value)} onValueChange={pick} disabled={disabled} size={size}>
      <Select.Trigger {...trigger} />
      <Select.Content position="popper">
        {options.map((o) => (
          <Select.Item key={o.value} value={toRadix(o.value)} disabled={o.disabled}>
            {o.label}
          </Select.Item>
        ))}
      </Select.Content>
    </Select.Root>
  );
}

/** Einträge aus einfachen Werten, Anzeige = Wert. */
export const plainOptions = <T extends string>(values: readonly T[]): SelectOption<T>[] =>
  values.map((v) => ({ value: v, label: v }));
