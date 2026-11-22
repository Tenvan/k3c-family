import { useState } from 'react';
import { Badge, Button, Code, Text } from '@webkit/atoms';
import { errorText } from '@webkit/lib/errors';
import { backend, type BalanceInsights, type InsightCommand, type Lever } from '../api';
import { leverLine, leverText } from './variant';

const WHERE: Record<InsightCommand['where'], string> = { chat: 'Chat an Claude', shell: 'Terminal', workbench: 'Balancing-Seite' };

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

interface Props {
  insights: BalanceInsights;
  onEdit: (variant: string) => void;
}

/** Hinweise (B-660): Tuning-Hinweise mit Stellschrauben und „Als Variante übernehmen“, darunter die Befehle. */
export function HintsTab({ insights, onEdit }: Props) {
  const [note, setNote] = useState('');
  // Die Variante braucht den aktuellen Wert der Stellschraube aus data/ (Datei-Werte, ein Aufruf).
  const take = async (lever: Lever) => {
    try {
      const line = leverLine(lever, await backend.balanceDataValuesAll());
      if (line) onEdit(line);
      else setNote(`Wert von ${lever.path} nicht gefunden.`);
    } catch (e) {
      setNote(`Fehlgeschlagen: ${errorText(e)}`);
    }
  };
  return (
    <>
      {insights.hints.length === 0 && <Text size="2" color="gray">Keine Hinweise: alle Ziele im Korridor.</Text>}
      {insights.hints.map((h) => (
        <div key={h.target + h.text} className="bal-insight-hint" data-light={h.light}>
          <Text size="2"><b>{h.target}</b> – {h.text}</Text>
          <Text as="p" size="1" color="gray">{h.why}</Text>
          {h.levers.length === 0 && <Text as="p" size="1" color="gray">Keine Stellschraube hinterlegt (data/balance-hints.json).</Text>}
          {h.levers.map((l) => (
            <div key={l.path + l.direction} className="bal-lever">
              <Text size="2">{l.label}</Text>
              <Code size="1">{l.path}</Code>
              <Text size="1" color="gray">{leverText(l)}</Text>
              <Button size="1" variant="soft" onClick={() => void take(l)}>Als Variante übernehmen</Button>
            </div>
          ))}
        </div>
      ))}
      {note && <Text size="1" color="gray" role="status">{note}</Text>}
      <section className="bal-card" aria-label="Befehle">
        <Text weight="bold" size="2">Befehle</Text>
        {insights.next.commands.map((c) => <CommandRow key={c.label} cmd={c} onNote={setNote} />)}
      </section>
    </>
  );
}
