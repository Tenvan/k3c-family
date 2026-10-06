import { Checkbox, IconButton, Select, Text } from '@radix-ui/themes';
import { useState } from 'react';
import { backend, type PlanTicket, type PlanningData } from '../api';
import { errorText } from '../lib/errors';
import { loadText, savePref } from '../lib/prefs';
import { MarkdownView } from '../ui/MarkdownView';
import { StatusBadge } from '../ui/parts';
import { findSession, groupTickets, linkedTickets } from './planning';
import { CopyPrompt, DepLinks, FieldMenu } from './PromptParts';
import { backlogPrompt, NEW_SPRINT, pickable, promptSession, worktreeVariant, type SprintTarget } from './prompts';
import { ModeBadge, prioTone, tone } from './SprintCard';

const PRIOS = ['hoch', 'mittel', 'niedrig', '?'];
const AGENTS = ['autonom', 'Mensch'];
const ENVS = ['offline', 'live', '?'];
const envTone = (e = '') => (e === 'offline' ? 'ok' : e === 'live' ? 'info' : 'neutral');
const TARGET = 'planning-target';

/** Kopf-Feld setzen; ein Fehler (z. B. unbekannter Wert) erscheint als Hinweis, der Rest kommt über `planning:changed`. */
const set = (id: string, field: string, value: string) => backend.planningSet(id, field, value).catch((e) => window.alert(errorText(e)));

/** Ziel der Backlog-Prompts: gemerkter Sprint, fehlt er inzwischen, der erste offene, sonst „neu“. */
function useTarget(ids: string[]): [string, (v: string) => void] {
  const [raw, setRaw] = useState(() => loadText(TARGET, ''));
  const value = raw === NEW_SPRINT || ids.includes(raw) ? raw : (ids[0] ?? NEW_SPRINT);
  return [value, (v) => { setRaw(v); savePref(TARGET, v); }];
}

/** Backlog je Domäne mit Mehrfachauswahl und Prompt zum Einplanen in einen Sprint; Prio per Klick änderbar; Tickets
 *  der ausgewählten Session sind hervorgehoben. */
export function BacklogList({ data, tickets, sel }: { data: PlanningData; tickets: PlanTicket[]; sel: string }) {
  const linked = linkedTickets(data, sel);
  const ids = data.sprints.filter((s) => s.status !== 'erledigt').map((s) => s.id);
  const [goal, choose] = useTarget(ids);
  const [checked, setChecked] = useState<ReadonlySet<string>>(new Set());
  const target: SprintTarget = goal === NEW_SPRINT ? { kind: 'new' } : { kind: 'sprint', id: goal };
  const picked = data.tickets.filter((t) => checked.has(t.nr));
  const check = (nr: string, on: boolean) => setChecked((cur) => { const n = new Set(cur); if (on) n.add(nr); else n.delete(nr); return n; });
  return (
    <>
      <div className="pl-target">
        <Text size="1" color="gray">Einplanen in</Text>
        <Select.Root size="1" value={goal} onValueChange={choose}>
          <Select.Trigger />
          <Select.Content>
            {ids.map((id) => <Select.Item key={id} value={id}>{id}</Select.Item>)}
            <Select.Item value={NEW_SPRINT}>Neuer Sprint</Select.Item>
          </Select.Content>
        </Select.Root>
        {picked.length > 0 && (
          <>
            <Text size="1" weight="medium">{picked.length} markiert</Text>
            <CopyPrompt prompt={backlogPrompt(picked, target)} what={`${picked.length} markierte Tickets`} />
            <button type="button" className="pl-textlink" onClick={() => setChecked(new Set())}>Auswahl aufheben</button>
          </>
        )}
      </div>
      {tickets.length === 0 && <Text color="gray">Kein Ticket passt zum Filter.</Text>}
      {groupTickets(tickets).map(([dom, ts]) => (
        <div key={dom}>
          <div className="pl-group">{dom} <span>{ts.length}</span></div>
          {ts.map((t) => (
            <div key={t.nr} className={`pl-ticket${linked.has(t.nr) ? ' is-linked' : ''}`} title={`${t.typ} · Spec: ${t.spec}`}>
              <Checkbox size="1" aria-label={`${t.nr} markieren`} checked={checked.has(t.nr)} onCheckedChange={(on) => check(t.nr, on === true)} />
              <code>{t.nr}</code>
              <span className="pl-title">{t.title}</span>
              <FieldMenu label="Prio" value={t.prio} options={PRIOS} onPick={(v) => void set(t.nr, 'Prio', v)}>
                <StatusBadge tone={prioTone(t.prio)}>{t.prio || '?'}</StatusBadge>
              </FieldMenu>
              <FieldMenu label="Umgebung" value={t.env} options={ENVS} onPick={(v) => void set(t.nr, 'Umgebung', v)}>
                <StatusBadge tone={envTone(t.env)}>{t.env || '?'}</StatusBadge>
              </FieldMenu>
              <StatusBadge tone={t.status === 'eingeplant' ? 'info' : 'neutral'}>
                {t.status}{t.sprint && t.sprint !== '–' ? ` · ${t.sprint}` : ''}
              </StatusBadge>
              <CopyPrompt prompt={backlogPrompt([t], target)} what={goal === NEW_SPRINT ? `${t.nr} zu neuem Sprint` : `${t.nr} in ${goal}`} />
            </div>
          ))}
        </div>
      ))}
    </>
  );
}

/** Detail-Panel der ausgewählten Session mit Prompt und Agent zum Umstellen; verschwindet die Auswahl, bleibt es leer. */
export function SessionDetail({ data, sel, onSelect, onClose }:
  { data: PlanningData; sel: string; onSelect: (nr: string) => void; onClose: () => void }) {
  const hit = findSession(data, sel);
  if (!hit) return null;
  const { sprint, session: x } = hit;
  const open = pickable(x);
  return (
    <aside className="pl-detail">
      <div className="pl-head pl-sticky">
        <code>{x.nr}</code>
        <strong className="pl-title">{x.titel || sprint.title}</strong>
        <StatusBadge tone={tone(x.status)}>{x.status}</StatusBadge>
        {x.typ && <span className="pl-dim">{x.typ}</span>}
        {open ? (
          <>
            <FieldMenu label="Agent" value={x.agent} options={AGENTS} onPick={(v) => void set(x.nr, 'Agent', v)}>
              <StatusBadge tone={x.agent === 'autonom' ? 'ok' : 'neutral'}>{x.agent || '?'}</StatusBadge>
            </FieldMenu>
            <FieldMenu label="Umgebung" value={x.env ?? ''} options={ENVS} onPick={(v) => void set(x.nr, 'Umgebung', v)}>
              <StatusBadge tone={envTone(x.env)}>{x.env || '?'}</StatusBadge>
            </FieldMenu>
          </>
        ) : <ModeBadge agent={x.agent} env={x.env} />}
        <DepLinks ids={x.deps} title="Abhängig von" onPick={onSelect} />
        {open && <CopyPrompt prompt={promptSession(sprint, x)} what={x.nr} variants={[worktreeVariant([{ sprint, session: x }])]} />}
        <IconButton size="1" variant="ghost" color="gray" onClick={onClose} aria-label="Schließen">×</IconButton>
      </div>
      <div className="pl-md">
        {x.text ? <MarkdownView source={x.text} headingOffset={1} /> : (
          <Text color="gray">Noch keine Session-Datei – Entwurf im Sprint {sprint.id}.</Text>
        )}
      </div>
    </aside>
  );
}
