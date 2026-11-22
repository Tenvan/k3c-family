import { Fragment, useState, type ReactNode } from 'react';
import { ActionButton, Code, IconButton, LinkButton } from '@webkit/atoms';
import { DropdownMenu } from '@webkit/organisms';

const stop = { onClick: (e: { stopPropagation(): void }) => e.stopPropagation(), onKeyDown: (e: { stopPropagation(): void }) => e.stopPropagation() };

type Feedback = 'idle' | 'ok' | 'fail';

/** Zeigt 1,6 s lang ✓ bzw. ✕ statt des Symbols. */
function useFeedback() {
  const [state, setState] = useState<Feedback>('idle');
  const settle = (task: Promise<unknown>) => {
    task.then(() => setState('ok'), () => setState('fail'));
    window.setTimeout(() => setState('idle'), 1600);
  };
  return [state, settle] as const;
}

const toneOf = (f: Feedback) => (f === 'ok' ? 'green' : f === 'fail' ? 'red' : 'gray');
const iconOf = (f: Feedback, idle: ReactNode) => (f === 'ok' ? '✓' : f === 'fail' ? '✕' : idle);

/** Prompt zum Weiterarbeiten: ⧉ kopiert ihn in die Zwischenablage (Claude wird von hier aus nicht gestartet). */
export function CopyPrompt({ prompt, what }: { prompt: string; what: string }) {
  const [copied, settleCopy] = useFeedback();
  return (
    <span className="pl-copy" {...stop}>
      <IconButton size="1" variant="ghost" color={toneOf(copied)} aria-label={`Prompt kopieren: ${what}`}
        title={`Prompt kopieren: ${what}`} onClick={() => settleCopy(navigator.clipboard.writeText(prompt))}>
        {iconOf(copied, '⧉')}
      </IconButton>
    </span>
  );
}

/** Wie CopyPrompt, aber als Knopf mit Text („Prompt: Auswerten“); zeigt nach dem Kopieren kurz ✓. */
export function CopyPromptButton({ prompt, label }: { prompt: string; label: string }) {
  const [copied, settleCopy] = useFeedback();
  return (
    <ActionButton size="1" variant="soft" color={toneOf(copied)} title="Kopiert den Prompt in die Zwischenablage"
      onClick={() => settleCopy(navigator.clipboard.writeText(prompt))}>
      {copied === 'ok' ? '✓ Kopiert' : copied === 'fail' ? '✕ Fehlgeschlagen' : label}
    </ActionButton>
  );
}

/** Abhängigkeiten als Links (`← GR4.2, S5.1`); ein Klick springt zum Ziel, ohne die umgebende Zeile auszulösen.
 *  Ohne Vorsatz (`lead=""`) erscheinen die IDs als eckige Ticket-Links. */
export function DepLinks({ ids, title, onPick, lead = '←' }: { ids?: string[]; title: string; onPick: (id: string) => void; lead?: string }) {
  if (!ids?.length) return null;
  return (
    <span className={lead ? 'pl-deps' : 'pl-deps is-tickets'} title={title}>
      {lead && `${lead} `}
      {ids.map((id, i) => (
        <Fragment key={id}>
          {i > 0 && ', '}
          <LinkButton size="1" title={`Zu ${id} springen`} {...stop} onClick={(e) => { e.stopPropagation(); onPick(id); }}>
            {lead ? id : <Code size="1" variant="outline">{id}</Code>}
          </LinkButton>
        </Fragment>
      ))}
    </span>
  );
}

/** Kopf-Feld direkt bearbeiten (Prio, Agent): Klick auf die Marke öffnet die erlaubten Werte. */
export function FieldMenu({ value, options, label, onPick, children }:
  { value: string; options: string[]; label: string; onPick: (v: string) => void; children: ReactNode }) {
  return (
    <span {...stop}>
      <DropdownMenu.Root>
        <DropdownMenu.Trigger>
          <LinkButton underline="none" title={`${label} ändern`} aria-label={`${label}: ${value || '–'} ändern`}>{children}</LinkButton>
        </DropdownMenu.Trigger>
        <DropdownMenu.Content size="1">
          <DropdownMenu.Label>{label}</DropdownMenu.Label>
          {options.map((o) => (
            <DropdownMenu.Item key={o} disabled={o === value} onSelect={() => onPick(o)}>{o}</DropdownMenu.Item>
          ))}
        </DropdownMenu.Content>
      </DropdownMenu.Root>
    </span>
  );
}
