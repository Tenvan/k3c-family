import { Fragment, type CSSProperties } from 'react';
import { Text, TextField } from '@webkit/atoms';
import type { WorldDetail, WorldIsland } from '../api';
import { MARK_ICON, bandText, gradientOf, groupEtappen, islandAnchor, marksOf, paletteOf, stageId, widthPct } from './crossModel';
import type { LevelModel } from './levelView';
import type { StageChoice } from './useWorld';

// Querschnitt des Welten-Editors (B-963): alle Inseln untereinander, je Insel eine Karte, jede Stufe ein Streifen
// (Breite ∝ Länge, Farben der Biom-Palette, Chunk-Raster) auf gemeinsamer Mittelachse; Etappen-Bänder über den Inseln.

interface StripProps {
  island: WorldIsland;
  stage: number;
  model: LevelModel;
  models: ReadonlyMap<string, LevelModel>;
  biomeInfo: WorldDetail['biomeInfo'];
  selected: boolean;
  onPick: (island: string, stage: number) => void;
}

function StageStrip({ island, stage, model, models, biomeInfo, selected, onPick }: StripProps) {
  const chunkW = model.chunks[0] ? model.chunks[0].to - model.chunks[0].from : 0;
  const portals = model.objects.filter((o) => o.kind === 'portal').length;
  const label = `Tiefe ${island.depths[stage]} · ${model.biomeId} · ≈${model.widthUnits} Units · ${portals} Portale · Chunks ${chunkW} u`;
  const pick = () => onPick(island.id, stage);
  const style: CSSProperties = { width: `${widthPct(model, models)}%`, background: gradientOf(paletteOf(biomeInfo, model.biomeId)) };
  return (
    <div className="wd-x-stage" data-stage={stage}>
      <Text as="div" size="1" color="gray">{label}</Text>
      <div role="button" tabIndex={0} aria-pressed={selected} aria-label={`Stufe ${stage} von ${island.name} bearbeiten`}
        className={`wd-x-bar${selected ? ' wd-x-bar-selected' : ''}`} style={style}
        onClick={pick} onKeyDown={(e) => (e.key === 'Enter' || e.key === ' ') && pick()}>
        {model.chunks.map((c) => <i key={c.index} className="wd-x-grid" style={{ left: `${(c.from / (model.widthUnits || 1)) * 100}%` }} />)}
        {marksOf(model, stage === island.depths.length - 1, biomeInfo?.[model.biomeId]?.portalHp).map((m, i) => (
          <span key={i} className={`wd-x-mark wd-x-${m.kind}`} title={m.title}
            style={{ left: `${m.left}%`, ...(m.width ? { width: `${m.width}%` } : {}) }}>{MARK_ICON[m.kind]}</span>
        ))}
      </div>
    </div>
  );
}

interface EtappeFieldProps {
  island: WorldIsland;
  index: number;
  onEtappe: (island: string, etappe: number | null) => void;
}

/** Eingabe der Etappe einer Insel (leer = Listenplatz + 1); gespeichert wird beim Verlassen des Feldes. */
function EtappeField({ island, index, onEtappe }: EtappeFieldProps) {
  const commit = (text: string) => {
    const t = text.trim();
    const n = t === '' ? null : Number(t);
    if (n !== null && (!Number.isInteger(n) || n < 1)) return;
    if (n !== (island.etappe ?? null)) onEtappe(island.id, n);
  };
  return (
    <label className="wd-x-etappe">
      <Text size="1" color="gray">Etappe</Text>
      <TextField.Root key={island.etappe ?? 'auto'} size="1" type="number" min={1} aria-label={`Etappe von ${island.name}`}
        placeholder={String(index + 1)} defaultValue={island.etappe ?? ''}
        onBlur={(e) => commit(e.target.value)} onKeyDown={(e) => e.key === 'Enter' && e.currentTarget.blur()} />
    </label>
  );
}

interface IslandProps {
  island: WorldIsland;
  index: number;
  models: ReadonlyMap<string, LevelModel>;
  biomeInfo: WorldDetail['biomeInfo'];
  onEtappe?: (island: string, etappe: number | null) => void;
  choice: StageChoice | null;
  hit: boolean;
  onPick: (island: string, stage: number) => void;
}

function IslandCard({ island, index, models, biomeInfo, onEtappe, choice, hit, onPick }: IslandProps) {
  return (
    <section className={`wd-card wd-x-island${hit ? ' wd-x-hit' : ''}`} id={islandAnchor(island.id)} data-island={island.id} aria-label={`Insel ${index + 1}: ${island.name}`}>
      <header className="wd-x-head">
        <Text as="div" size="3" weight="bold">Insel {index + 1}: {island.name}</Text>
        <Text size="1" color="orange" weight="bold">{island.biomes.join(' → ')}</Text>
        {onEtappe && <EtappeField island={island} index={index} onEtappe={onEtappe} />}
      </header>
      {island.depths.map((_, stage) => {
        const model = models.get(stageId(island.id, stage));
        return model
          ? <StageStrip key={stage} island={island} stage={stage} model={model} models={models} biomeInfo={biomeInfo}
            selected={choice?.island === island.id && choice.stage === stage} onPick={onPick} />
          : <Text key={stage} as="div" size="1" color="gray" className="wd-x-stage">Stufe {stage} lädt …</Text>;
      })}
    </section>
  );
}

interface Props {
  detail: WorldDetail;
  models: ReadonlyMap<string, LevelModel>;
  choice: StageChoice | null;
  /** Insel, die kurz hervorgehoben wird (Sprung der Seekarte) */
  hit: string | null;
  onPick: (island: string, stage: number) => void;
  /** Etappe einer Insel setzen (null = Listenplatz + 1); fehlt sie, bleibt das Feld aus */
  onEtappe?: (island: string, etappe: number | null) => void;
}

/** Etappen-Bänder und Insel-Karten der ganzen Welt. */
export function CrossSection({ detail, models, choice, hit, onPick, onEtappe }: Props) {
  const groups = groupEtappen(detail);
  return (
    <div className="wd-x" aria-label="Querschnitt">
      {groups.map((g, gi) => (
        <Fragment key={g.etappe}>
          <div className="wd-x-band" data-etappe={g.etappe}>{bandText(g, groups[gi - 1])}</div>
          {g.islands.map(({ island, index }) => (
            <IslandCard key={island.id} island={island} index={index} models={models} biomeInfo={detail.biomeInfo} onEtappe={onEtappe} choice={choice} hit={hit === island.id} onPick={onPick} />
          ))}
        </Fragment>
      ))}
      {groups.length === 0 && <Text size="1" color="gray">Die Welt hat keine Inseln.</Text>}
    </div>
  );
}
