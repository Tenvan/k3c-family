import { useState, type ReactNode } from 'react';
import { Popover } from '@radix-ui/themes';
import { Button, Flex } from '../atoms';

export interface MenuAction {
  label: ReactNode;
  onSelect: () => void;
  disabled?: boolean;
  danger?: boolean;
}

interface Props {
  /** Beschriftung des Auslösers. */
  label: ReactNode;
  'aria-label'?: string;
  actions: readonly MenuAction[];
}

/** Knopf mit kleiner Liste von Aktionen (Popover, öffnet per Klick); die Auswahl schließt die Liste. */
export function ActionMenu({ label, actions, ...rest }: Props) {
  const [open, setOpen] = useState(false);
  return (
    <Popover.Root open={open} onOpenChange={setOpen}>
      <Popover.Trigger>
        <Button size="1" variant="soft" aria-label={rest['aria-label']}>{label} ▾</Button>
      </Popover.Trigger>
      <Popover.Content size="1" width="auto" align="end">
        <Flex direction="column" gap="1" align="stretch">
          {actions.map((a, i) => (
            <Button key={i} size="1" variant="ghost" color={a.danger ? 'red' : 'gray'} disabled={a.disabled} style={{ justifyContent: 'flex-start' }} onClick={() => { setOpen(false); a.onSelect(); }}>
              {a.label}
            </Button>
          ))}
        </Flex>
      </Popover.Content>
    </Popover.Root>
  );
}
