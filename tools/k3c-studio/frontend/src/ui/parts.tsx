import { Button } from '@radix-ui/themes';
import { useState, type ComponentProps, type ReactNode } from 'react';

// Bausteine der Oberfläche, Kopie aus k3c-dev (B-064 › Bausteine): Knopf mit Ladezustand, Status-Badge, Hinweiskarte.

export type Tone = 'ok' | 'warn' | 'error' | 'info' | 'neutral';

type ButtonProps = Omit<ComponentProps<typeof Button>, 'onClick' | 'loading'> & {
  /** Läuft das Promise, zeigt der Knopf den Ladezustand und ist gesperrt. */
  onClick: () => Promise<unknown> | void;
};

export function ActionButton({ onClick, disabled, ...rest }: ButtonProps) {
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

export function StatusBadge({ tone, children }: { tone: Tone; children: ReactNode }) {
  return (
    <span className={`badge tone-${tone}`}>
      <span className="badge-dot" aria-hidden />
      {children}
    </span>
  );
}

export function NoticeCard({ title, tone = 'info', children }: { title: string; tone?: Tone; children: ReactNode }) {
  return (
    <div className={`notice tone-${tone}`} role="note">
      <p className="notice-title">{title}</p>
      <p className="notice-body">{children}</p>
    </div>
  );
}
