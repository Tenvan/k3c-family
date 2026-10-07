import { DropdownMenu, IconButton } from '@radix-ui/themes';
import { Fragment, useState, type ReactNode } from 'react';
import { backend } from '../api';

const stop = { onClick: (e: { stopPropagation(): void }) => e.stopPropagation(), onKeyDown: (e: { stopPropagation(): void }) => e.stopPropagation() };

/** Claude-Funke in Claude-Orange: zwölf spitze Strahlen, abwechselnd lang und kurz. */
function ClaudeIcon() {
  return (
    <svg aria-hidden="true" height="15" viewBox="0 0 24 24" width="15">
      <g fill="#D97757">
        {Array.from({ length: 12 }, (_, i) => (
          <path key={i} d={i % 2 ? 'M12 4.5 L13 11 L12 12.5 L11 11 Z' : 'M12 1.5 L13.2 10.8 L12 12.5 L10.8 10.8 Z'} transform={`rotate(${i * 30} 12 12)`} />
        ))}
      </g>
    </svg>
  );
}

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

/** Prompt zum Weiterarbeiten, zwei Wege: ⧉ kopiert ihn, der Claude-Funke öffnet ihn als neue Code-Session in
 *  Claude Desktop (Deep Link, schickt nicht ab). */
export function CopyPrompt({ prompt, what }: { prompt: string; what: string }) {
  const [copied, settleCopy] = useFeedback();
  const [opened, settleOpen] = useFeedback();
  return (
    <span className="pl-copy" {...stop}>
      <IconButton size="1" variant="ghost" color={toneOf(copied)} aria-label={`Prompt kopieren: ${what}`}
        title={`Prompt kopieren: ${what}`} onClick={() => settleCopy(navigator.clipboard.writeText(prompt))}>
        {iconOf(copied, '⧉')}
      </IconButton>
      <IconButton size="1" variant="ghost" color={toneOf(opened)} aria-label={`In Claude öffnen: ${what}`}
        title={`In Claude öffnen (neue Code-Session): ${what}`} onClick={() => settleOpen(backend.openInClaude(prompt))}>
        {iconOf(opened, <ClaudeIcon />)}
      </IconButton>
    </span>
  );
}

/** Abhängigkeiten als Links (`← GR4.2, S5.1`); ein Klick springt zum Ziel, ohne die umgebende Zeile auszulösen. */
export function DepLinks({ ids, title, onPick }: { ids?: string[]; title: string; onPick: (id: string) => void }) {
  if (!ids?.length) return null;
  return (
    <span className="pl-deps" title={title}>
      ←{' '}
      {ids.map((id, i) => (
        <Fragment key={id}>
          {i > 0 && ', '}
          <button type="button" className="pl-dep" title={`Zu ${id} springen`} {...stop}
            onClick={(e) => { e.stopPropagation(); onPick(id); }}>{id}</button>
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
          <button type="button" className="pl-link" title={`${label} ändern`} aria-label={`${label}: ${value || '–'} ändern`}>{children}</button>
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
