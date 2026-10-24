import { useState } from 'react';
import { Button, Flex, IconButton, SelectField, StatusBadge, Text } from '@webkit/atoms';
import type { BalanceReportInfo, ReportRating } from '../api';
import { Confirm, type ConfirmRequest } from '../git/Confirm';
import { HelpButton } from './HelpButton';

interface Props {
  reports: BalanceReportInfo[];
  selected: string;
  base: string;
  onSelect: (path: string) => void;
  onBase: (path: string) => void;
  onReload: () => void;
  onRate: (path: string, rating: ReportRating) => void;
  onDelete: (path: string) => void;
}

const RATINGS: { value: ReportRating; label: string }[] = [
  { value: '', label: 'Bewertung …' }, { value: 'gut', label: 'gut' }, { value: 'schlecht', label: 'schlecht' },
  { value: 'unklar', label: 'unklar' }, { value: 'referenz', label: 'Referenz' },
];

const HELP = `**Öffnen:** Klick auf den Namen zeigt den Bericht rechts (Ziele, Kennzahlen, Diagramme).

**Vergleichen:** „Vergleichsbasis“ macht einen Bericht zum Maßstab **A**. Danach zeigt jeder geöffnete Bericht **B** seine Werte neben denen von A: Abstand je Ziel und welche Ziele gegenüber A „gekippt“ sind (Pass ↔ Fail). Typisch: A = Lauf ohne Variante, B = Lauf mit Variante. Noch einmal klicken hebt die Basis auf.

**Bewerten:** gut, schlecht, unklar oder Referenz; die Bewertung steht in \`reports/ratings.json\` und bleibt erhalten.

**Löschen:** entfernt den Bericht (JSON und Markdown) nach Rückfrage; ein leerer Lauf-Ordner verschwindet mit.`;

const nameOf = (r: BalanceReportInfo) => r.run || r.path.split('/').pop() || r.path;

/** Berichtsliste (neueste zuerst): öffnen (B), Vergleichsbasis (A), bewerten, löschen (WZ28.5, WZ28.13). */
export function ReportList({ reports, selected, base, onSelect, onBase, onReload, onRate, onDelete }: Props) {
  const [confirm, setConfirm] = useState<ConfirmRequest | null>(null);
  const baseReport = reports.find((r) => r.path === base);
  const ask = (r: BalanceReportInfo) =>
    setConfirm({ title: 'Bericht löschen?', text: `${r.path} wird gelöscht (samt Markdown und Bewertung).`, action: 'Löschen', danger: true, run: () => onDelete(r.path) });
  return (
    <section className="bal-card" aria-label="Berichte">
      <div className="bal-run-head">
        <Text weight="bold">Berichte</Text>
        <HelpButton title="Berichte" source={HELP} />
        <Button size="1" variant="ghost" onClick={onReload}>Neu laden</Button>
      </div>
      {baseReport && (
        <Flex gap="2" align="center" wrap="wrap" className="bal-base-hint">
          <Text size="1">Vergleich: <strong>A</strong> = {nameOf(baseReport)} (Basis), <strong>B</strong> = {selected && selected !== base ? nameOf(reports.find((r) => r.path === selected) ?? baseReport) : 'Bericht öffnen'}</Text>
          <Button size="1" variant="ghost" onClick={() => onBase('')}>Basis aufheben</Button>
        </Flex>
      )}
      {reports.length === 0 && <Text color="gray" size="2">Keine Berichte unter reports/.</Text>}
      {reports.map((r) => (
        <div key={r.path} className="bal-report-row" data-on={r.path === selected ? 'true' : undefined} data-rating={r.rating || undefined}>
          <Button size="1" variant="ghost" onClick={() => onSelect(r.path)}>{nameOf(r)}</Button>
          <StatusBadge tone={r.pass ? 'ok' : 'error'}>{r.pass ? 'Pass' : 'Fail'}</StatusBadge>
          {r.kind === 'tuning' && <StatusBadge tone="info">Abstimmung</StatusBadge>}
          <Text size="1" color="gray">{r.kind === 'tuning' ? '' : `${r.seeds} Seeds · `}{new Date(r.modified).toLocaleString('de-DE')}{r.varianten?.length ? ` · ${r.varianten.length} Var.` : ''}</Text>
          <span className="bal-report-tools">
            <SelectField aria-label={`Bewertung ${nameOf(r)}`} size="1" value={r.rating ?? ''} options={RATINGS} onChange={(v) => onRate(r.path, v)} />
            {r.kind !== 'tuning' && (
              <Button size="1" variant={r.path === base ? 'solid' : 'soft'} onClick={() => onBase(r.path === base ? '' : r.path)}>
                {r.path === base ? 'Basis A ✓' : 'Vergleichsbasis'}
              </Button>
            )}
            <IconButton size="1" variant="ghost" color="red" aria-label={`Löschen ${nameOf(r)}`} onClick={() => ask(r)}>✕</IconButton>
          </span>
        </div>
      ))}
      <Confirm request={confirm} onClose={() => setConfirm(null)} />
    </section>
  );
}
