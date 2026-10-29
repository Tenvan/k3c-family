import type { ReactNode } from 'react';
import { Badge, Button, Flex, Text } from '../atoms';

export interface SwitchOption<T extends string> {
  value: T;
  label: ReactNode;
  /** Zähler hinter der Beschriftung. */
  count?: number;
  disabled?: boolean;
}

export interface ChipSwitchProps<T extends string> {
  /** `aria-label` der Gruppe. */
  label: string;
  options: readonly SwitchOption<T>[];
  /** Gewählter Wert; ein Wert außerhalb der Einträge (z. B. "") heißt: keiner gewählt. */
  value: T | '';
  onChange: (value: T) => void;
  /** Sichtbare Beschriftung links. */
  caption?: ReactNode;
}

/** Umschalter aus Chips (ein Wert gewählt, `aria-pressed`); ein Klick auf den gewählten Chip meldet ihn erneut,
 *  der Aufrufer entscheidet, ob das abwählt. */
export function ChipSwitch<T extends string>({ label, options, value, onChange, caption }: ChipSwitchProps<T>) {
  return (
    <Flex gap="1" align="center" wrap="wrap" role="group" aria-label={label}>
      {caption && <Text size="1" color="gray">{caption}</Text>}
      {options.map((o) => {
        const on = o.value === value;
        return (
          <Button key={o.value} size="1" variant={on ? 'solid' : 'soft'} color={on ? undefined : 'gray'} aria-pressed={on} disabled={o.disabled} onClick={() => onChange(o.value)}>
            {o.label}
            {o.count !== undefined && <Badge size="1" variant="outline">{o.count}</Badge>}
          </Button>
        );
      })}
    </Flex>
  );
}
