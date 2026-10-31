import { Badge, Button, Flex, IconButton } from '../atoms';

export interface FocusOption {
  id: string;
  label: string;
  /** Zahl hinter der Beschriftung. */
  count?: number;
  /** Eigener Chip: lässt sich entfernen. */
  custom?: boolean;
}

export interface FocusBarProps {
  /** `aria-label` der Gruppe. */
  label: string;
  options: readonly FocusOption[];
  /** Id des gewählten Chips; "" = keiner passt zum aktuellen Filter. */
  value: string;
  onChange: (id: string) => void;
  onRemove?: (id: string) => void;
  /** Knopf „Ansicht merken“ am Ende; ohne Handler fehlt er. */
  onSave?: () => void;
  saveLabel?: string;
}

/** Fokus-Chips (B-677): gespeicherte Filter mit Zähler als Reihe von Chips; eigene Chips tragen ein ✕, am Ende der Knopf zum Merken. */
export function FocusBar({ label, options, value, onChange, onRemove, onSave, saveLabel = 'Ansicht merken' }: FocusBarProps) {
  return (
    <Flex gap="1" align="center" wrap="wrap" role="group" aria-label={label}>
      {options.map((o) => {
        const on = o.id === value;
        return (
          <Flex key={o.id} gap="0" align="center">
            <Button size="1" variant={on ? 'solid' : 'soft'} color={on ? undefined : 'gray'} aria-pressed={on} onClick={() => onChange(o.id)}>
              {o.label}
              {o.count !== undefined && <Badge size="1" variant="outline">{o.count}</Badge>}
            </Button>
            {o.custom && onRemove && <IconButton size="1" variant="ghost" color="gray" aria-label={`${o.label} entfernen`} onClick={() => onRemove(o.id)}>✕</IconButton>}
          </Flex>
        );
      })}
      {onSave && <Button size="1" variant="outline" color="gray" onClick={onSave}>{saveLabel}</Button>}
    </Flex>
  );
}
