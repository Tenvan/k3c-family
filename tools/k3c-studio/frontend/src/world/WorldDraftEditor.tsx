import { BIOMES } from '@game/model/biome';
import { Button, Text, TextField } from '@webkit/atoms';
import { useState } from 'react';
import type { DraftField, DraftScaling, IslandDraft, PlanIsland } from '../api';
import type { PlanState } from './usePlan';
import type { WorldDraftState } from './useWorldDraft';
import { addIsland, moveIsland, removeIsland, toggleDepth, updateIsland } from './worldDraft';

const FACTORS: { key: keyof DraftScaling; label: string }[] = [{ key: 'hp', label: 'HP' }, { key: 'damage', label: 'Schaden' }, { key: 'speed', label: 'Tempo' }];
const biomeOf = (depth: number) => BIOMES.find((b) => b.depth === depth);
const FIELDS: DraftField[] = ['name', 'id', 'depths', 'plan', 'depthScaling'];

/** Prüffehler am Feld (island -1 = Welt), sonst nichts. */
function FieldError({ world, island, field }: { world: WorldDraftState; island: number; field: DraftField }) {
  const { check } = world;
  if (check.ok || check.island !== island || check.field !== field) return null;
  return <Text as="p" size="1" color="red" role="alert" className="wd-error" data-island={island} data-field={field}>{check.error}</Text>;
}

/** Welt-Kopf: Name (die id bleibt) und Notizen. */
function WorldHead({ world }: { world: WorldDraftState }) {
  const d = world.draft!;
  return (
    <section className="wd-card" aria-label="Welt">
      <Text size="2" weight="bold">Welt</Text>
      <TextField.Root aria-label="Name der Welt" placeholder="Name der Welt" value={d.name} onChange={(e) => world.edit({ ...d, name: e.target.value })} />
      <TextField.Root aria-label="Notizen der Welt" placeholder="Notizen: Wofür ist diese Welt?" value={d.notes} onChange={(e) => world.edit({ ...d, notes: e.target.value })} />
      <FieldError world={world} island={-1} field="name" />
      <FieldError world={world} island={-1} field="islands" />
      <FieldError world={world} island={-1} field="" />
    </section>
  );
}

/** Eine Insel der Liste: wählbare Karte mit Name, id und den Biomen der Stufen als Farbmarken, dazu Reihenfolge und Entfernen. */
function IslandCard({ world, i, on, onPick, onMove, onRemove }: { world: WorldDraftState; i: number; on: boolean; onPick: () => void; onMove: (delta: number) => void; onRemove: () => void }) {
  const d = world.draft!;
  const isl = d.islands[i]!;
  const n = i + 1;
  return (
    <li className="wd-island" data-island={i} data-selected={on}>
      <div className="wd-bar">
        <Button variant={on ? 'soft' : 'ghost'} color="gray" highContrast className="wd-pick-island" aria-pressed={on} aria-label={`Insel ${n} wählen`} onClick={onPick}>
          <span className="wd-island-name">{n}. {isl.name}</span> <Text size="1" color="gray">{isl.id} · {isl.depths.length} Stufen</Text>
        </Button>
        <span className="wd-grow" />
        <Button variant="soft" aria-label={`Insel ${n} hoch`} disabled={i === 0} onClick={() => onMove(-1)}>↑</Button>
        <Button variant="soft" aria-label={`Insel ${n} runter`} disabled={i === d.islands.length - 1} onClick={() => onMove(1)}>↓</Button>
        <Button variant="outline" color="red" aria-label={`Insel ${n} entfernen`} disabled={d.islands.length < 2} onClick={onRemove}>×</Button>
      </div>
      <span className="wd-chips" aria-hidden>{isl.depths.map((depth) => <span key={depth} className="wd-chip" data-depth={depth} title={biomeOf(depth)?.name} />)}</span>
      {FIELDS.map((f) => <FieldError key={f} world={world} island={i} field={f} />)}
    </li>
  );
}

/** Karte einer Stufe der gewählten Insel: Biom, Größe und Planung der gespeicherten Datei, Sprung in den Modus Stufe bearbeiten. */
function StageCard({ depth, saved, onEdit }: { depth: number; saved?: PlanIsland; onEdit?: () => void }) {
  const biome = biomeOf(depth);
  const at = saved?.depths.indexOf(depth) ?? -1;
  const count = saved?.chunkCounts[String(depth)];
  const planned = saved?.planned[String(depth)]?.length ?? 0;
  return (
    <li className="wd-stage-card" data-depth={depth}>
      <Text size="1" color="gray">Stufe {at >= 0 ? at + 1 : '–'}</Text>
      <Text size="2" weight="bold">{biome?.name ?? `Tiefe ${depth}`}</Text>
      <Text size="1" color="gray">{saved ? `${count ? `${count} Abschnitte` : 'Biom-Länge'}${planned > 0 ? ` · ${planned} geplant` : ' · zufällig'}` : 'neu, erst nach dem Speichern planbar'}</Text>
      {onEdit && at >= 0 && <Button variant="ghost" size="1" aria-label={`Stufe ${at + 1} bearbeiten`} onClick={onEdit}>Stufe bearbeiten →</Button>}
    </li>
  );
}

/** Gewählte Insel: Name, id, Biome der Stufen als Schalter, Stufenkarten und Skalierung (HP, Schaden, Tempo). */
function IslandDetail({ world, i, saved, onEditStage }: { world: WorldDraftState; i: number; saved?: PlanIsland; onEditStage: (islandId: string, stage: number) => void }) {
  const d = world.draft!;
  const isl: IslandDraft = d.islands[i]!;
  const n = i + 1;
  const set = (patch: Parameters<typeof updateIsland>[2]) => world.edit(updateIsland(d, i, patch));
  return (
    <section className="wd-card" aria-label="Gewählte Insel">
      <div className="wd-bar">
        <Text size="2" weight="bold">{isl.name}</Text>
        <Text size="1" color="gray">Insel {n}</Text>
      </div>
      <div className="wd-bar">
        <TextField.Root className="wd-seed" aria-label={`Name Insel ${n}`} value={isl.name} onChange={(e) => set({ name: e.target.value })} />
        <TextField.Root className="wd-kind" aria-label={`ID Insel ${n}`} title="Insel-ID (a–z, 0–9, -); Spielstände verweisen darauf" value={isl.id} onChange={(e) => set({ id: e.target.value })} />
      </div>
      <Text size="2" weight="bold">Stufen</Text>
      <div className="wd-bar" role="group" aria-label={`Stufen Insel ${n}`}>
        {BIOMES.map((b) => {
          const on = isl.depths.includes(b.depth);
          return (
            <Button key={b.depth} size="1" variant={on ? 'solid' : 'soft'} color={on ? undefined : 'gray'} aria-pressed={on} aria-label={`Insel ${n} Tiefe ${b.depth}`}
              onClick={() => set({ depths: toggleDepth(isl.depths, b.depth) })}>{b.name}</Button>
          );
        })}
      </div>
      <ul className="wd-stage-cards" aria-label="Stufen der Insel">
        {isl.depths.map((depth) => <StageCard key={depth} depth={depth} saved={saved} onEdit={saved ? () => onEditStage(isl.id, isl.depths.indexOf(depth)) : undefined} />)}
      </ul>
      <Text size="2" weight="bold">Skalierung je Tiefe</Text>
      <div className="wd-bar" role="group" aria-label={`Skalierung Insel ${n}`}>
        {FACTORS.map((f) => (
          <Text as="label" size="2" key={f.key} className="wd-field">
            {f.label}
            <TextField.Root className="wd-slot" type="number" step="0.05" aria-label={`Insel ${n} ${f.label}`} value={isl.depthScaling[f.key]}
              onChange={(e) => { const v = Number(e.target.value); if (!Number.isNaN(v)) set({ depthScaling: { ...isl.depthScaling, [f.key]: v } }); }} />
          </Text>
        ))}
      </div>
    </section>
  );
}

/** Welt-Entwurf im Modus Welt-Struktur (WZ18.6, B-488/AC-05, B-664): Name und Notizen der Welt, links die Inseln als Liste
 *  (anlegen, umordnen, entfernen), rechts die gewählte Insel mit Stufen und Skalierung. Jede Änderung wird sofort geprüft
 *  (Fehler am Feld, Vorschau der Datei); Speichern, Verwerfen und die Meldung stehen im Kopf der Seite (`WorldHeader`). */
export function WorldDraftEditor({ world, plan, onEditStage }: { world: WorldDraftState; plan: PlanState; onEditStage: (islandId: string, stage: number) => void }) {
  const { draft, check, dirty } = world;
  const [picked, setPicked] = useState(0);
  if (!draft) return null;
  const sel = Math.min(picked, draft.islands.length - 1);
  const saved = plan.view?.islands.find((x) => x.id === draft.islands[sel]?.from);
  /** Die Auswahl folgt der Insel, nicht der Stelle in der Liste (B-664) */
  const move = (i: number, delta: number) => { world.edit(moveIsland(draft, i, delta)); if (sel === i) setPicked(i + delta); else if (sel === i + delta) setPicked(i); };
  const remove = (i: number) => { world.edit(removeIsland(draft, i)); if (i < sel) setPicked(sel - 1); };
  return (
    <div className="wd-columns wd-welt" aria-label="Welt-Entwurf">
      <section className="wd-struct" aria-label="Welt und Inseln">
        <WorldHead world={world} />
        <section className="wd-card" aria-label="Inseln der Welt">
          <div className="wd-bar">
            <Text size="2" weight="bold">Inseln</Text>
            <Text size="1" color="gray">Reihenfolge = Reihenfolge im Spiel</Text>
            <span className="wd-grow" />
            <Button variant="soft" title="Neue Insel als Kopie der letzten (Tiefen, Skalierung), ohne Planung" onClick={() => { world.edit(addIsland(draft)); setPicked(draft.islands.length); }}>Insel +</Button>
          </div>
          <ol className="wd-islands" aria-label="Inseln der Welt">
            {draft.islands.map((_, i) => <IslandCard key={i} world={world} i={i} on={i === sel} onPick={() => setPicked(i)} onMove={(delta) => move(i, delta)} onRemove={() => remove(i)} />)}
          </ol>
        </section>
      </section>
      <div className="wd-tree">
        <IslandDetail world={world} i={sel} saved={saved} onEditStage={onEditStage} />
        {dirty && check.preview && (
          <details>
            <summary>Vorschau der Welt-Datei</summary>
            <pre aria-label="Vorschau der Welt-Datei">{check.preview}</pre>
          </details>
        )}
      </div>
    </div>
  );
}
