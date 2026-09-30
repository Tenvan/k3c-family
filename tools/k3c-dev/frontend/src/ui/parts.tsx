import { Button, Tooltip } from '@radix-ui/themes';
import { useState, type ComponentProps, type ReactNode } from 'react';

// Bausteine der Oberfläche (B-064 › Bausteine): Knopf mit Ladezustand, Status-Badge, Hinweiskarte, Tooltip.

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

/** Tooltip; ohne Inhalt nur das Kind. */
export function Tip({ content, children }: { content: ReactNode; children: ReactNode }) {
  if (!content) return <>{children}</>;
  return (
    <Tooltip content={content}>
      <span>{children}</span>
    </Tooltip>
  );
}
