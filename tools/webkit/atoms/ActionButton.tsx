import { Button } from '@radix-ui/themes';
import { useState, type ComponentProps } from 'react';

type Props = Omit<ComponentProps<typeof Button>, 'onClick' | 'loading'> & {
  /** Läuft das Promise, zeigt der Knopf den Ladezustand und ist gesperrt. */
  onClick: () => Promise<unknown> | void;
};

/** Knopf mit Ladezustand, sperrt Doppelklicks (B-064 › Bausteine). */
export function ActionButton({ onClick, disabled, ...rest }: Props) {
  const [busy, setBusy] = useState(false);
  const click = async () => {
    setBusy(true);
    try {
      await onClick();
    } finally {
      setBusy(false);
    }
  };
  return <Button {...rest} loading={busy} disabled={disabled || busy} onClick={() => void click()} />;
}
