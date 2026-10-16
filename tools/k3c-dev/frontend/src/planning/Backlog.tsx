import { Checkbox, Select, Text } from '@radix-ui/themes';
import { useState } from 'react';
import { backend, type PlanTicket, type PlanningData } from '../api';
import { errorText } from '../lib/errors';
import { loadText, savePref } from '../lib/prefs';
import { FoldButton, useFold } from './fold';
import { Pill, prioTone, type PillTone } from './Pill';
import { groupTickets, linkedTickets } from './planning';
import { CopyPrompt, FieldMenu } from './PromptParts';
import { backlogPrompt, NEW_SPRINT, type SprintTarget } from './prompts';

const PRIOS = ['hoch', 'mittel', 'niedrig', '?'];
export const ENVS = ['offline', 'live', '?'];
export const envTone = (e = ''): PillTone => (e === 'offline' ? 'ok' : e === 'live' ? 'info' : 'neutral');
const TARGET = 'planning.target';

/** Werte des Felds Projekt: `–` (kein Projekt) und die Kürzel aller Projekte. */
export const projectOptions = (data: PlanningData): string[] => ['–', ...(data.projects ?? []).map((p) => p.id)];

/** Kopf-Feld setzen; ein Fehler (z. B. unbekannter Wert) erscheint als Hinweis, der Rest kommt über `planning:changed`. */
export const setField = (id: string, field: string, value: string) => backend.planningSet(id, field, value).catch((e) => window.alert(errorText(e)));

/** Ziel der Backlog-Prompts: gemerkter Sprint, fehlt er inzwischen, der erste offene, sonst „neu“. */
export function useTarget(ids: string[]): [SprintTarget, string, (v: string) => void] {
  const [raw, setRaw] = useState(() => loadText(TARGET, ''));
  const value = raw === NEW_SPRINT || ids.includes(raw) ? raw : (ids[0] ?? NEW_SPRINT);
  const target: SprintTarget = value === NEW_SPRINT ? { kind: 'new' } : { kind: 'sprint', id: value };
  return [target, value, (v) => { setRaw(v); savePref(TARGET, v); }];
}

interface Props {
  data: PlanningData;
  /** Gefilterte Tickets. */
  tickets: PlanTicket[];
  sel: string;
  /** Ticket fürs Detail wählen. */
  onPick: (nr: string) => void;
  /** Ziel der Prompts (useTarget im Board, das Detail nutzt es auch). */
  target: SprintTarget;
  goal: string;
  choose: (v: string) => void;
}

/** Backlog-Spalte (erp-qs-Look): Kopf mit Zähler und Ziel, Leiste der markierten Tickets mit Prompt, Tickets je Domäne
 *  einklappbar mit Sammel-Checkbox. Die Auswahl gilt auch für Tickets, die der Filter gerade ausblendet. */
export function BacklogView({ data, tickets, sel, onPick, target, goal, choose }: Props) {
  const ids = data.sprints.filter((s) => s.status !== 'erledigt').map((s) => s.id);
  const [checked, setChecked] = useState<ReadonlySet<string>>(new Set());
  const setMany = (nrs: string[], on: boolean) => setChecked((cur) => {
    const n = new Set(cur);
    nrs.forEach((nr) => (on ? n.add(nr) : n.delete(nr)));
    return n;
  });
  const picked = data.tickets.filter((t) => checked.has(t.nr));
  const row = { data, sel, linked: linkedTickets(data, sel), target, goal, onPick };
  return (
    <div className="pl-backlog">
      <div className="pl-bl-bar">
        <Text className="pl-heading">Backlog</Text>
        <Text size="1" color="gray">{tickets.length} von {data.tickets.length} offenen Tickets</Text>
        <span className="pl-spacer" />
        <Text size="1">Ziel:</Text>
        <Select.Root size="1" value={goal} onValueChange={choose}>
          <Select.Trigger aria-label="Ziel der Backlog-Prompts" />
          <Select.Content>
            {ids.map((id) => <Select.Item key={id} value={id}>{id}</Select.Item>)}
            <Select.Item value={NEW_SPRINT}>Neuer Sprint</Select.Item>
          </Select.Content>
        </Select.Root>
      </div>
      {picked.length > 0 && (
        <div className="pl-picked">
          <Text size="1" weight="medium">{picked.length} markiert</Text>
          <CopyPrompt prompt={backlogPrompt(picked, target)} what={`${picked.length} markierte Tickets`} />
          <button type="button" className="pl-textlink" onClick={() => setChecked(new Set())}>Auswahl aufheben</button>
        </div>
      )}
      <div className="pl-bl-list">
        {groupTickets(tickets).map(([dom, ts]) => <DomainSection key={dom} dom={dom} tickets={ts} checked={checked} onCheck={setMany} row={row} />)}
        {tickets.length === 0 && <Text color="gray" size="2">{data.tickets.length ? 'Kein Ticket passt zum Filter.' : 'Keine offenen Tickets.'}</Text>}
      </div>
    </div>
  );
}

type RowCtx = Omit<RowProps, 't' | 'checked' | 'onCheck'>;

/** Eine Domäne: Pfeil, Sammel-Checkbox (mit Zwischenzustand) und ihre Ticket-Zeilen. */
function DomainSection({ dom, tickets, checked, onCheck, row }:
  { dom: string; tickets: PlanTicket[]; checked: ReadonlySet<string>; onCheck: (nrs: string[], on: boolean) => void; row: RowCtx }) {
  const [folded, toggle] = useFold(`dom:${dom}`);
  const nrs = tickets.map((t) => t.nr);
  const n = nrs.filter((nr) => checked.has(nr)).length;
  return (
    <section className="pl-bl-domain" data-domain={dom}>
      <div className="pl-bl-domain-head">
        <FoldButton folded={folded} onToggle={toggle} what={`Domäne ${dom}`} />
        <Checkbox aria-label={`Alle sichtbaren Tickets in ${dom} markieren`} checked={n === nrs.length ? true : n > 0 ? 'indeterminate' : false}
          onCheckedChange={(on) => onCheck(nrs, on === true)} />
        <button type="button" className="pl-bl-domain-name" onClick={toggle}>{dom} ({tickets.length})</button>
      </div>
      {!folded && (
        <ul>
          {tickets.map((t) => <TicketRow key={t.nr} t={t} checked={checked.has(t.nr)} onCheck={(on) => onCheck([t.nr], on)} {...row} />)}
        </ul>
      )}
    </section>
  );
}

interface RowProps {
  t: PlanTicket;
  data: PlanningData;
  sel: string;
  linked: Set<string>;
  checked: boolean;
  onCheck: (on: boolean) => void;
  target: SprintTarget;
  goal: string;
  onPick: (nr: string) => void;
}

/** Ticket-Zeile: Auswahl, Nr., Titel, Prio/Projekt/Umgebung zum Ändern, Status und Prompt; Klick zeigt das Detail. */
function TicketRow({ t, data, sel, linked, checked, onCheck, target, goal, onPick }: RowProps) {
  const cls = ['pl-bl-ticket', sel === t.nr && 'is-sel', linked.has(t.nr) && 'is-linked'].filter(Boolean).join(' ');
  const planned = t.sprint && t.sprint !== '–';
  return (
    <li className={cls} title={`${t.typ} · Spec: ${t.spec}`} role="button" tabIndex={0} onClick={() => onPick(t.nr)}
      onKeyDown={(e) => e.target === e.currentTarget && (e.key === 'Enter' || e.key === ' ') && (e.preventDefault(), onPick(t.nr))}>
      <Checkbox size="1" aria-label={`${t.nr} markieren`} checked={checked} onCheckedChange={(on) => onCheck(on === true)} onClick={(e) => e.stopPropagation()} />
      <span className="pl-bl-id">{t.nr}</span>
      <span className="pl-bl-title">{t.title}</span>
      <span className="pl-pills">
        <FieldMenu label="Prio" value={t.prio} options={PRIOS} onPick={(v) => void setField(t.nr, 'Prio', v)}>
          <Pill tone={prioTone(t.prio)}>{t.prio || '?'}</Pill>
        </FieldMenu>
        <FieldMenu label="Projekt" value={t.project ?? '–'} options={projectOptions(data)} onPick={(v) => void setField(t.nr, 'Projekt', v)}>
          <Pill tone="neutral">{t.project || '–'}</Pill>
        </FieldMenu>
        <FieldMenu label="Umgebung" value={t.env} options={ENVS} onPick={(v) => void setField(t.nr, 'Umgebung', v)}>
          <Pill tone={envTone(t.env)}>{t.env || '?'}</Pill>
        </FieldMenu>
        <Pill tone={planned ? 'info' : 'neutral'}>{planned ? `eingeplant · ${t.sprint}` : t.status}</Pill>
      </span>
      <CopyPrompt prompt={backlogPrompt([t], target)} what={target.kind === 'new' ? `${t.nr} zu neuem Sprint` : `${t.nr} in ${goal}`} />
    </li>
  );
}
