import { useState } from 'react';
import { Badge, Button, Code, Text, TONE_COLOR, type Tone } from '@webkit/atoms';
import { NoticeCard } from '@webkit/molecules';
import { errorText } from '@webkit/lib/errors';
import { backend, type BalanceInsights, type InsightCommand, type Lever, type Light } from '../api';
import { useInsights } from './useBalancing';

// Auswertung eines Berichts (B-632): Kurzauswertung, Ampeln, Stärken/Schwächen, Tuning-Hinweise, nächster Lauf und
// „HTML-Bericht öffnen“. Die Texte rechnet die Go-Seite (internal/balance/insights*.go).

const TONE: Record<Light, Tone> = { green: 'ok', yellow: 'warn', red: 'error', grey: 'neutral' };
const color = (l: Light) => TONE_COLOR[TONE[l]];
const LIGHT_TEXT: Record<Light, string> = { green: 'gut', yellow: 'knapp', red: 'verletzt', grey: 'ungültig' };
const WHERE: Record<InsightCommand['where'], string> = { chat: 'Chat an Claude', shell: 'Terminal', workbench: 'Balancing-Seite' };

function Lights({ insights }: { insights: BalanceInsights }) {
  return (
    <div className="bal-run-actions" aria-label="Ampeln">
      {(['green', 'yellow', 'red', 'grey'] as Light[]).map((l) => (
        <Badge key={l} color={color(l)} variant="soft" size="2">
          {insights.counts[l]} {LIGHT_TEXT[l]}
        </Badge>
      ))}
      {insights.figures.lights.map((f) => (
        <Badge key={f.key + (f.grade ?? '')} color={color(f.light)}
          variant="outline" size="1" title={f.text}>
          {f.name}{f.grade && f.grade !== 'normal' ? ` (${f.grade})` : ''}: {f.value ?? '–'} {f.unit}
        </Badge>
      ))}
    </div>
  );
}

function Bullets({ title, items, empty }: { title: string; items: string[]; empty: string }) {
  return (
    <div>
      <Text weight="bold" size="2">{title}</Text>
      {items.length === 0 ? <Text as="p" size="2" color="gray">{empty}</Text> : <ul className="bal-insight-list">{items.map((s) => <li key={s}>{s}</li>)}</ul>}
    </div>
  );
}

function Levers({ levers }: { levers: Lever[] }) {
  if (!levers.length) return null;
  return <ul className="bal-insight-list">{levers.map((l) => <li key={l.path + l.direction}>{l.label} <Code size="1">{l.path}</Code> ({l.direction}{Math.round(l.step * 100)} %)</li>)}</ul>;
}

/** Ein Befehl mit „Kopieren“ und bei Chat-Befehlen „In Claude öffnen“ (neue Session mit dem Text im Eingabefeld). */
function CommandRow({ cmd, onNote }: { cmd: InsightCommand; onNote: (s: string) => void }) {
  const run = (p: Promise<unknown>, ok: string) => p.then(() => onNote(ok), (e) => onNote(`Fehlgeschlagen: ${errorText(e)}`));
  return (
    <div className="bal-insight-cmd">
      <Text size="2" weight="medium">{cmd.label} <Badge size="1" variant="outline">{WHERE[cmd.where]}</Badge></Text>
      <Code size="1" className="bal-insight-code">{cmd.text}</Code>
      <div className="bal-run-actions">
        <Button size="1" variant="soft" onClick={() => run(navigator.clipboard.writeText(cmd.text), 'Kopiert.')}>Kopieren</Button>
        {cmd.where === 'chat' && <Button size="1" variant="soft" onClick={() => run(backend.openInClaude(cmd.text), 'In Claude geöffnet.')}>In Claude öffnen</Button>}
      </div>
    </div>
  );
}

function Next({ insights }: { insights: BalanceInsights }) {
  const [note, setNote] = useState('');
  return (
    <div>
      <Text weight="bold" size="2">Nächster Lauf</Text>
      <ol className="bal-insight-list">{insights.next.steps.map((s) => <li key={s}>{s}</li>)}</ol>
      {insights.next.commands.map((c) => <CommandRow key={c.label} cmd={c} onNote={setNote} />)}
      {note && <Text size="1" color="gray" role="status">{note}</Text>}
    </div>
  );
}

/** „HTML-Bericht öffnen“: schreibt .md/.html neu (auch für alte Berichte) und öffnet den Bericht im Browser. */
export function HtmlButton({ path }: { path: string }) {
  const [note, setNote] = useState('');
  const open = () => backend.balanceReportHTML(path).then((p) => setNote(`Geöffnet: ${p}`), (e) => setNote(`Fehlgeschlagen: ${errorText(e)}`));
  return (
    <>
      <Button size="1" onClick={open}>HTML-Bericht öffnen</Button>
      {note && <Text size="1" color="gray" role="status">{note}</Text>}
    </>
  );
}

/** Auswertung zum Bericht `path`. */
export function InsightsView({ path }: { path: string }) {
  const { insights, error } = useInsights(path);
  if (error) return <NoticeCard title="Auswertung nicht möglich" tone="warn">{error}</NoticeCard>;
  if (!insights) return <Text size="2" color="gray">Lade Auswertung …</Text>;
  return (
    <section className="bal-insights" aria-label="Auswertung">
      <Lights insights={insights} />
      <ul className="bal-insight-summary">{insights.summary.map((s) => <li key={s}>{s}</li>)}</ul>
      <div className="bal-insight-grid">
        <Bullets title="Stärken" items={insights.strengths} empty="Keine erkannt." />
        <Bullets title="Schwächen" items={insights.weaknesses} empty="Keine erkannt." />
      </div>
      <div>
        <Text weight="bold" size="2">Tuning-Hinweise</Text>
        {insights.hints.length === 0 && <Text as="p" size="2" color="gray">Keine: alle Ziele im Korridor.</Text>}
        {insights.hints.map((h) => (
          <div key={h.target + h.text} className="bal-insight-hint" data-light={h.light}>
            <Text size="2"><b>{h.target}</b> – {h.text}</Text>
            <Text as="p" size="1" color="gray">{h.why}</Text>
            <Levers levers={h.levers} />
          </div>
        ))}
      </div>
      <Next insights={insights} />
    </section>
  );
}
