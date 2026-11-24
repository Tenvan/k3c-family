import { useState } from 'react';
import { Button, Flex, SelectField, Text, TextField } from '@webkit/atoms';
import type { EndBossOption, WorldIsland } from '../api';

// Endboss je Insel und Skins der Welt (WE6.4, B-981, B-984). Die Engine prüft beim Speichern; die Tabelle prüft das
// Format vorab (<gruppe>/<asset>, beide Seiten dieselbe Gruppe), damit kein Fehlversuch unnötig schreibt.

const PAIR = /^[a-z0-9_-]+\/[a-z0-9_-]+$/;

/** Ein Skin-Paar ist gültig, wenn beide Seiten <gruppe>/<asset> sind und dieselbe Gruppe haben. */
export function skinProblem(from: string, to: string): string {
  if (!PAIR.test(from) || !PAIR.test(to)) return 'Form <gruppe>/<asset>, a-z 0-9 - _';
  if (from.split('/')[0] !== to.split('/')[0]) return 'beide Seiten in derselben Gruppe';
  return '';
}

export function EndBossSelect({ island, options, onChange }: { island: WorldIsland; options: readonly EndBossOption[]; onChange: (boss: string) => void }) {
  const current = island.endBoss ?? '';
  const known = options.some((o) => o.id === current);
  const choices = [{ value: '', label: 'Standard (tiefste Stufe)' }, ...options.map((o) => ({ value: o.id, label: o.name }))];
  if (current && !known) choices.push({ value: current, label: `${current} (unbekannt)` });
  return <SelectField size="1" aria-label={`Endboss von ${island.name || island.id}`} value={current} onChange={onChange} options={choices} />;
}

type Row = { from: string; to: string };

export function SkinTable({ skins, onSave }: { skins: Record<string, string>; onSave: (skins: Record<string, string>) => void }) {
  const [rows, setRows] = useState<Row[]>(() => Object.entries(skins).map(([from, to]) => ({ from, to })));
  const problems = rows.map((r) => skinProblem(r.from.trim(), r.to.trim()));
  const valid = problems.every((p) => p === '') && new Set(rows.map((r) => r.from.trim())).size === rows.length;
  const save = () => {
    const out: Record<string, string> = {};
    for (const r of rows) out[r.from.trim()] = r.to.trim();
    onSave(out);
  };
  return (
    <Flex direction="column" gap="2">
      <Text size="2" weight="bold">Skins</Text>
      {rows.map((r, i) => (
        <Flex key={i} gap="2" align="center">
          <TextField.Root size="1" aria-label={`Basis-Asset ${i + 1}`} value={r.from} onChange={(e) => setRows(rows.map((x, j) => (j === i ? { ...x, from: e.target.value } : x)))} />
          <Text size="1">→</Text>
          <TextField.Root size="1" aria-label={`Ersatz-Asset ${i + 1}`} value={r.to} onChange={(e) => setRows(rows.map((x, j) => (j === i ? { ...x, to: e.target.value } : x)))} />
          <Button size="1" variant="soft" aria-label={`Skin ${i + 1} entfernen`} onClick={() => setRows(rows.filter((_, j) => j !== i))}>Entfernen</Button>
          {problems[i] && <Text size="1" color="red">{problems[i]}</Text>}
        </Flex>
      ))}
      <Flex gap="2">
        <Button size="1" variant="soft" onClick={() => setRows([...rows, { from: '', to: '' }])}>Skin hinzufügen</Button>
        <Button size="1" disabled={!valid} onClick={save}>Skins speichern</Button>
      </Flex>
    </Flex>
  );
}

/** Abschnitt „Endboss und Skins“: je Insel die Endboss-Auswahl, darunter die Skin-Tabelle der Welt. */
export function BossPanel({ islands, endBosses, skins, onEndBoss, onSkins }: {
  islands: WorldIsland[]; endBosses: EndBossOption[]; skins: Record<string, string>;
  onEndBoss: (island: string, boss: string) => void; onSkins: (skins: Record<string, string>) => void;
}) {
  return (
    <Flex direction="column" gap="3">
      <Text size="2" weight="bold">Endboss je Insel</Text>
      {islands.map((isl) => (
        <Flex key={isl.id} gap="2" align="center">
          <Text size="1">{isl.name || isl.id}</Text>
          <EndBossSelect island={isl} options={endBosses} onChange={(b) => onEndBoss(isl.id, b)} />
        </Flex>
      ))}
      <SkinTable key={JSON.stringify(skins)} skins={skins} onSave={onSkins} />
    </Flex>
  );
}
