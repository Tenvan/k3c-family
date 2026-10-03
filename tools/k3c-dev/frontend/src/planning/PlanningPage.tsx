import { Button, Flex, SegmentedControl, Select, Text, TextField } from '@radix-ui/themes';
import { useEffect, useMemo, useState } from 'react';
import { backend, type PlanDoc, type PlanningData, type PlanSprint } from '../api';
import { errorText } from '../lib/errors';
import { loadPref, savePref } from '../lib/prefs';
import { MarkdownView } from '../ui/MarkdownView';
import { NoticeCard, StatusBadge } from '../ui/parts';
import { filterTickets, groupByDomain, nextSession, progress, sessionTone } from './planning';

const VIEWS = ['sprints', 'plan', 'fragen'] as const;
type View = (typeof VIEWS)[number];

/** Reiter `Planung`: Umschalter zwischen Sprints & Backlog, dem Plan und dem Fragenkatalog (docs/). */
export function PlanningPage() {
  const [view, setView] = useState<View>(() => loadPref('planning', VIEWS, 'sprints'));
  const choose = (v: string) => {
    if (!VIEWS.includes(v as View)) return;
    setView(v as View);
    savePref('planning', v);
  };
  return (
    <div className="pl-page">
      <SegmentedControl.Root value={view} onValueChange={choose} size="2">
        <SegmentedControl.Item value="sprints">Sprints &amp; Backlog</SegmentedControl.Item>
        <SegmentedControl.Item value="plan">Plan</SegmentedControl.Item>
        <SegmentedControl.Item value="fragen">Fragenkatalog</SegmentedControl.Item>
      </SegmentedControl.Root>
      {view === 'sprints' ? <SprintsAndBacklog /> : <DocView key={view} name={view} />}
    </div>
  );
}

function DocView({ name }: { name: PlanDoc }) {
  const [text, setText] = useState<string | null>(null);
  const [error, setError] = useState('');
  const load = () => {
    setError('');
    backend.planningDoc(name).then(setText, (e) => setError(errorText(e)));
  };
  useEffect(() => {
    load();
    return backend.on('planning:changed', load); // Plan oder Fragenkatalog wurde gespeichert
  }, [name]);
  if (error) return <NoticeCard title="Dokument nicht geladen" tone="error">{error}</NoticeCard>;
  if (text === null) return <Text color="gray">Lade …</Text>;
  return (
    <article className="pl-doc">
      <Flex justify="end"><Button size="1" variant="soft" color="gray" onClick={load}>Neu laden</Button></Flex>
      <MarkdownView source={text} />
    </article>
  );
}

function SprintsAndBacklog() {
  const [data, setData] = useState<PlanningData | null>(null);
  const [error, setError] = useState('');
  const load = () => backend.planning().then((d) => { setData(d); setError(''); }, (e) => setError(errorText(e)));
  useEffect(() => {
    void load();
    return backend.on('planning:changed', () => void load()); // Wächter in Go: Datei in docs/ geändert
  }, []);
  if (error) return <NoticeCard title="Planung nicht geladen" tone="error">{error}</NoticeCard>;
  if (!data) return <Text color="gray">Lade Planung …</Text>;
  return (
    <div className="pl-grid">
      <section className="pl-col">
        <Flex justify="between" align="center">
          <Text weight="bold">Sprints</Text>
          <Flex gap="3" align="center">
            <Text size="1" color="gray">{data.done} erledigt</Text>
            <Button size="1" variant="soft" onClick={() => void load()}>Neu laden</Button>
          </Flex>
        </Flex>
        {data.sprints.map((s) => <SprintCard key={s.id} s={s} />)}
      </section>
      <Backlog data={data} />
    </div>
  );
}

function SprintCard({ s }: { s: PlanSprint }) {
  const p = progress(s);
  const next = nextSession(s);
  return (
    <div className={s.status === 'aktiv' ? 'pl-card pl-card-on' : 'pl-card'}>
      <Flex align="center" gap="2" wrap="wrap">
        <strong>{s.id}</strong>
        <span className="pl-domain">{s.domain}</span>
        <span className="pl-title">{s.title}</span>
        <StatusBadge tone={s.status === 'aktiv' ? 'info' : 'neutral'}>{s.status}</StatusBadge>
        <StatusBadge tone={s.spec === 'freigegeben' ? 'ok' : 'warn'}>Spec: {s.spec || '–'}</StatusBadge>
      </Flex>
      {p.total > 0 && (
        <div className="pl-bar" title={`${p.done} von ${p.total} Sessions fertig`}>
          <div className="pl-bar-fill" style={{ width: `${(p.done / p.total) * 100}%` }} />
        </div>
      )}
      <ul className="pl-sessions">
        {s.sessions.map((x) => (
          <li key={x.nr} className={x === next ? 'pl-next' : undefined}>
            <StatusBadge tone={sessionTone(x)}>{x.status}</StatusBadge>
            <code>{x.nr}</code>
            <span>{x.titel || [x.typ, x.agent].filter(Boolean).join(' · ')}</span>
          </li>
        ))}
      </ul>
      {s.tickets.length > 0 && <Text size="1" color="gray">Tickets: {s.tickets.join(', ')}</Text>}
    </div>
  );
}

function Backlog({ data }: { data: PlanningData }) {
  const [query, setQuery] = useState('');
  const [status, setStatus] = useState('alle');
  const [domain, setDomain] = useState('alle');
  const groups = useMemo(() => groupByDomain(filterTickets(data.tickets, { query, status, domain })), [data, query, status, domain]);
  const statuses = useMemo(() => ['alle', ...new Set(data.tickets.map((t) => t.status))], [data]);
  const domains = useMemo(() => ['alle', ...new Set(data.tickets.map((t) => t.domain))], [data]);
  const shown = groups.reduce((n, g) => n + g.tickets.length, 0);
  return (
    <section className="pl-col">
      <Flex gap="2" align="center" wrap="wrap">
        <Text weight="bold">Backlog</Text>
        <TextField.Root className="pl-filter" size="1" placeholder="Filter: Nr. oder Titel" value={query}
          onChange={(e) => setQuery(e.target.value)} />
        <Pick value={status} options={statuses} onChange={setStatus} />
        <Pick value={domain} options={domains} onChange={setDomain} />
      </Flex>
      <div className="pl-tickets">
        {groups.map((g) => (
          <div key={g.domain}>
            <div className="pl-group">{g.domain} <span>{g.tickets.length}</span></div>
            {g.tickets.map((t) => (
              <div key={t.nr} className="pl-ticket" title={`${t.typ} · Spec: ${t.spec}`}>
                <code>{t.nr}</code>
                <span className="pl-ticket-title">{t.title}</span>
                <StatusBadge tone={t.prio === 'hoch' ? 'warn' : 'neutral'}>{t.prio}</StatusBadge>
                <StatusBadge tone={t.status === 'eingeplant' ? 'info' : 'neutral'}>{t.status}{t.sprint !== '–' ? ` · ${t.sprint}` : ''}</StatusBadge>
              </div>
            ))}
          </div>
        ))}
        {shown === 0 && <Text color="gray" size="2">Kein Ticket passt zum Filter.</Text>}
      </div>
      <Text size="1" color="gray">{shown} von {data.tickets.length} offenen Tickets</Text>
    </section>
  );
}

function Pick({ value, options, onChange }: { value: string; options: string[]; onChange: (v: string) => void }) {
  return (
    <Select.Root size="1" value={value} onValueChange={onChange}>
      <Select.Trigger />
      <Select.Content>
        {options.map((o) => <Select.Item key={o} value={o}>{o}</Select.Item>)}
      </Select.Content>
    </Select.Root>
  );
}
