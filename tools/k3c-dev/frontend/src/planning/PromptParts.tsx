import { DropdownMenu, IconButton } from '@radix-ui/themes';
import { Fragment, useState, type ReactNode } from 'react';
import type { PromptVariant } from './prompts';

const stop = { onClick: (e: { stopPropagation(): void }) => e.stopPropagation(), onKeyDown: (e: { stopPropagation(): void }) => e.stopPropagation() };

/** Kopiert einen Prompt zum Weiterarbeiten; zeigt 1,6 s lang ✓ bzw. ✕. Mit `variants` öffnet ▾ daneben ein Menü mit
 *  weiteren Prompts (etwa „Autonom im Worktree“); der Klick auf ⧉ bleibt der einfache Weg. */
export function CopyPrompt({ prompt, what, variants = [] }: { prompt: string; what: string; variants?: PromptVariant[] }) {
  const [state, setState] = useState<'idle' | 'ok' | 'fail'>('idle');
  const write = (text: string) => {
    navigator.clipboard.writeText(text).then(() => setState('ok'), () => setState('fail'));
    window.setTimeout(() => setState('idle'), 1600);
  };
  return (
    <span className="pl-copy" {...stop}>
      <IconButton size="1" variant="ghost" color={state === 'ok' ? 'green' : state === 'fail' ? 'red' : 'gray'}
        aria-label={`Prompt kopieren: ${what}`} title={`Prompt zum Weiterarbeiten kopieren: ${what}`} onClick={() => write(prompt)}>
        {state === 'ok' ? '✓' : state === 'fail' ? '✕' : '⧉'}
      </IconButton>
      {variants.length > 0 && (
        <DropdownMenu.Root>
          <DropdownMenu.Trigger>
            <IconButton size="1" variant="ghost" color="gray" aria-label={`Weitere Prompts: ${what}`} title="Weitere Prompts">▾</IconButton>
          </DropdownMenu.Trigger>
          <DropdownMenu.Content size="1">
            <DropdownMenu.Item onSelect={() => write(prompt)}>Prompt kopieren</DropdownMenu.Item>
            {variants.map((v) => (
              <DropdownMenu.Item key={v.label} disabled={!!v.blocked} title={v.blocked} onSelect={() => write(v.prompt)}>
                {v.label}{v.blocked ? ` (${v.blocked})` : ''}
              </DropdownMenu.Item>
            ))}
          </DropdownMenu.Content>
        </DropdownMenu.Root>
      )}
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
