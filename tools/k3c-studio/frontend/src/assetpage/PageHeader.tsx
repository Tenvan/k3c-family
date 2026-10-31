import { Button, Flex, SegmentedControl, SelectField, Text, TextField } from '@webkit/atoms';
import { FocusBar, type FocusOption } from '@webkit/molecules';
import { useState } from 'react';
import { GRUPPE_LABEL, GRUPPEN, SICHT_LABEL, SICHTEN, type Gruppe, type Sicht } from './filter';

interface Props {
  chips: readonly FocusOption[];
  chip: string;
  onChip: (id: string) => void;
  onRemoveChip: (id: string) => void;
  /** Speichert die aktuelle Ansicht unter diesem Namen als eigenen Chip. */
  onSaveChip: (name: string) => void;
  suggestedName: string;
  query: string;
  onQuery: (q: string) => void;
  sicht: Sicht;
  onSicht: (s: Sicht) => void;
  gruppe: Gruppe;
  onGruppe: (g: Gruppe) => void;
  tools: boolean;
  onTools: () => void;
  onReload: () => void;
  notice: string;
}

/** Kopf der Assets-Seite (B-677): Fokus-Chips, darunter Suche, Sicht, Gruppierung, Neu laden und Werkzeuge. „Ansicht merken“ fragt den Namen in der Zeile. */
export function PageHeader(p: Props) {
  const [naming, setNaming] = useState<string | null>(null);
  const save = () => {
    const name = (naming ?? '').trim();
    if (name) p.onSaveChip(name);
    setNaming(null);
  };
  return (
    <Flex direction="column" gap="2" className="ap-head">
      <FocusBar label="Fokus" options={p.chips} value={p.chip} onChange={p.onChip} onRemove={p.onRemoveChip} onSave={() => setNaming(p.suggestedName)} />
      {naming !== null && (
        <Flex gap="2" align="center" role="group" aria-label="Ansicht merken">
          <TextField.Root size="1" autoFocus aria-label="Name der Ansicht" value={naming} onChange={(e) => setNaming(e.target.value)} onKeyDown={(e) => { if (e.key === 'Enter') save(); if (e.key === 'Escape') setNaming(null); }} />
          <Button size="1" onClick={save} disabled={!naming.trim()}>Speichern</Button>
          <Button size="1" variant="soft" color="gray" onClick={() => setNaming(null)}>Abbrechen</Button>
        </Flex>
      )}
      <Flex gap="2" align="center" wrap="wrap">
        <TextField.Root className="as-filter" placeholder="Slot, Beschreibung, Datei oder Quelle suchen" aria-label="Suche" value={p.query} onChange={(e) => p.onQuery(e.target.value)} />
        <SegmentedControl.Root size="1" value={p.sicht} onValueChange={(v) => p.onSicht(v as Sicht)} aria-label="Sicht">
          {SICHTEN.map((s) => <SegmentedControl.Item key={s} value={s}>{SICHT_LABEL[s]}</SegmentedControl.Item>)}
        </SegmentedControl.Root>
        {p.sicht !== 'szene' && (
          <SelectField value={p.gruppe} aria-label="Gruppieren nach" onChange={(v) => p.onGruppe(v as Gruppe)} options={GRUPPEN.map((g) => ({ value: g, label: `Gruppieren: ${GRUPPE_LABEL[g]}` }))} />
        )}
        <Button size="1" variant="soft" onClick={p.onReload} title="Slot-Register und Vorschläge neu lesen">Neu laden</Button>
        <Button size="1" variant={p.tools ? 'solid' : 'soft'} aria-pressed={p.tools} onClick={p.onTools}>Werkzeuge</Button>
      </Flex>
      {p.notice && <Text size="1" color="amber" role="status">{p.notice}</Text>}
    </Flex>
  );
}
