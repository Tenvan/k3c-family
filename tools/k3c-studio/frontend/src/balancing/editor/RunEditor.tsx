import { useMemo, useRef, useState } from 'react';
import { Button, Flex, Heading } from '@webkit/atoms';
import { errorText } from '@webkit/lib/errors';
import type { BalanceBaseline, BalanceRun, BalanceSpec, BalanceDataFileValue } from '../../api';
import { savePreset, type BalanceForm, type Preset } from '../spec';
import { DataBrowser } from './DataBrowser';
import { EditorFooter } from './EditorFooter';
import { OriginPicker } from './OriginPicker';
import { Card, CompareSection, PlayersSection, ScenarioSection, ScopeSection } from './Sections';
import { VariantTable } from './VariantTable';
import { summaryText, workOf } from './estimate';
import { checkRows, keyOf } from './variantCheck';
import { buildSpec, canPerVariant, formWithRows, type EditorForm, type Origin } from './variantForm';
import { newRow, countKinds, type VariantKind } from './variants';
import { useEditorState } from './useEditorState';
import { useDataValues } from './useDataValues';

export interface RunEditorProps {
  /** Felder, mit denen der Editor öffnet (Bericht, Basis-Lauf, Beispiel …); `perVariant` nur, wenn der Auftrag sie hatte. */
  initial: EditorForm;
  /** Woher `initial` kommt; ohne Angabe „Leer“. */
  origin?: Origin;
  baselines: BalanceBaseline[];
  /** Läufe der Warteschlange: Grundlage der Dauer-Schätzung. */
  runs: BalanceRun[];
  presets: Preset[];
  onPresets: (p: Preset[]) => void;
  /** Startet den Lauf; der Editor schließt danach mit `onClose`. Eine Ablehnung erscheint in der Fußleiste. */
  onStart: (spec: BalanceSpec) => Promise<void>;
  /** Zurück zur Liste. */
  onClose: () => void;
}

const LEER: Origin = { kind: 'leer', label: '' };

function startLabel(bereich: number, perVariant: boolean, fest: number): string {
  if (bereich > 0) return 'Abstimmung starten';
  return perVariant ? `${fest + 1} Läufe starten` : 'Lauf starten';
}

function variantsSummary(fest: number, bereich: number): string {
  if (fest + bereich === 0) return 'keine Varianten';
  return [fest > 0 && `${fest} fest`, bereich > 0 && `${bereich} ${bereich === 1 ? 'Bereich' : 'Bereiche'}`].filter(Boolean).join(' · ');
}

/** Lauf-Editor (B-661): Ausgangspunkt, fünf Abschnitte, Varianten-Tabelle mit Daten-Browser daneben, feste Fußleiste.
 *  Braucht eine feste Höhe vom Elternelement (`height: 100%`); nur der Inhalt scrollt, die Fußleiste bleibt stehen. */
export function RunEditor({ initial, origin = LEER, baselines, runs, presets, onPresets, onStart, onClose }: RunEditorProps) {
  const { state, patch, setRows, setPerVariant, setListMode, load } = useEditorState(initial);
  const { form, rows, perVariant, listMode } = state;
  const data = useDataValues();
  const start = useRef<Partial<EditorForm> | undefined>(initial);
  const [error, setError] = useState('');
  const checks = useMemo(() => checkRows(rows, data.index), [rows, data.index]);
  const used = useMemo(() => new Set(rows.map((r) => keyOf(r.file, r.path))), [rows]);
  const counts = countKinds(rows);
  const on = perVariant && canPerVariant(rows);
  const full = formWithRows(form, rows);
  const work = workOf(full, counts, on);
  const bad = checks.findIndex((c) => c.error);
  const problem = bad >= 0 ? `Variante ${bad + 1}: ${checks[bad].error}` : buildSpec(form, rows, on).error ?? '';
  const add = (v: BalanceDataFileValue, kind: VariantKind) => setRows([...rows, newRow(v.file, v.path, v.value, kind)]);
  const loadFrom = (ef: Partial<EditorForm> | undefined) => { start.current = ef; setError(''); load(ef); };
  const run = async () => {
    const { spec, error: err } = buildSpec(form, rows, on);
    setError(err ?? '');
    if (!spec) return;
    try {
      await onStart(spec);
      onClose();
    } catch (e) {
      setError(errorText(e));
    }
  };
  return (
    <div className="bal-ed" aria-label="Lauf-Editor">
      <div className="bal-ed-body">
        <section className="bal-ed-main">
          <Flex align="center" gap="3" className="bal-ed-top">
            <Button size="1" variant="ghost" color="gray" aria-label="Zurück zur Liste" onClick={onClose}>‹ Läufe und Berichte</Button>
            <Heading size="4">{origin.kind === 'leer' ? 'Neuer Lauf' : 'Lauf bearbeiten'}</Heading>
          </Flex>
          <div className="bal-ed-scroll">
            <OriginPicker origin={origin} report={initial} baselines={baselines} presets={presets} onPresets={onPresets} onLoad={loadFrom} />
            <ScenarioSection form={form} patch={patch} />
            <PlayersSection form={form} patch={patch} />
            <ScopeSection form={form} patch={patch} listMode={listMode} onListMode={setListMode} />
            <Card title="4 · Varianten" summary={variantsSummary(counts.fest, counts.bereich)}>
              <VariantTable rows={rows} checks={checks} onRows={setRows} tune={form.tune} onTune={(tune) => patch({ tune })} perVariant={perVariant} onPerVariant={setPerVariant} />
            </Card>
            <CompareSection form={form} patch={patch} baselines={baselines} />
          </div>
          <EditorFooter
            summary={summaryText(runs, full, work)} startLabel={startLabel(counts.bereich, on, counts.fest)} problem={problem} error={error}
            presetName={form.label} onReset={() => loadFrom(start.current)} onStart={run}
            onSavePreset={(name) => onPresets(savePreset(presets, name, { ...full, perVariant: on } as BalanceForm))}
          />
        </section>
        <DataBrowser values={data.values} error={data.error} used={used} onAdd={add} />
      </div>
    </div>
  );
}
