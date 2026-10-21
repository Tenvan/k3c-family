import { BIOMES } from '@game/model/biome';
import { Button, Text, TextField } from '@webkit/atoms';
import { CheckField } from '@webkit/molecules';
import type { DraftField, DraftScaling } from '../api';
import type { WorldDraftState } from './useWorldDraft';
import { addIsland, moveIsland, removeIsland, toggleDepth, updateIsland } from './worldDraft';

const FACTORS: { key: keyof DraftScaling; label: string }[] = [{ key: 'hp', label: 'HP' }, { key: 'damage', label: 'Schaden' }, { key: 'speed', label: 'Tempo' }];

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
    <>
      <div className="wd-bar">
        <TextField.Root className="wd-seed" aria-label="Name der Welt" placeholder="Name der Welt" value={d.name} onChange={(e) => world.edit({ ...d, name: e.target.value })} />
        <TextField.Root className="wd-seed" aria-label="Notizen der Welt" placeholder="Notizen" value={d.notes} onChange={(e) => world.edit({ ...d, notes: e.target.value })} />
      </div>
      <FieldError world={world} island={-1} field="name" />
      <FieldError world={world} island={-1} field="islands" />
      <FieldError world={world} island={-1} field="" />
    </>
  );
}

/** Eine Insel im Entwurf: Name, id, Reihenfolge, Tiefen (Biome) und Skalierung, Fehler am Feld. */
function IslandRow({ world, i }: { world: WorldDraftState; i: number }) {
  const d = world.draft!;
  const isl = d.islands[i]!;
  const n = i + 1;
  const set = (patch: Parameters<typeof updateIsland>[2]) => world.edit(updateIsland(d, i, patch));
  return (
    <li className="wd-island" data-island={i}>
      <div className="wd-bar">
        <TextField.Root className="wd-seed" aria-label={`Name Insel ${n}`} value={isl.name} onChange={(e) => set({ name: e.target.value })} />
        <TextField.Root className="wd-kind" aria-label={`ID Insel ${n}`} title="Insel-ID (a–z, 0–9, -); Spielstände verweisen darauf" value={isl.id} onChange={(e) => set({ id: e.target.value })} />
        <Button variant="soft" aria-label={`Insel ${n} hoch`} disabled={i === 0} onClick={() => world.edit(moveIsland(d, i, -1))}>↑</Button>
        <Button variant="soft" aria-label={`Insel ${n} runter`} disabled={i === d.islands.length - 1} onClick={() => world.edit(moveIsland(d, i, 1))}>↓</Button>
        <Button variant="outline" color="red" aria-label={`Insel ${n} entfernen`} disabled={d.islands.length < 2} onClick={() => world.edit(removeIsland(d, i))}>×</Button>
      </div>
      <FieldError world={world} island={i} field="name" />
      <FieldError world={world} island={i} field="id" />
      <div className="wd-bar" role="group" aria-label={`Stufen Insel ${n}`}>
        {BIOMES.map((b) => (
          <CheckField key={b.depth} label={b.name} aria-label={`Insel ${n} Tiefe ${b.depth}`} checked={isl.depths.includes(b.depth)}
            onCheckedChange={() => set({ depths: toggleDepth(isl.depths, b.depth) })} />
        ))}
      </div>
      <FieldError world={world} island={i} field="depths" />
      <FieldError world={world} island={i} field="plan" />
      <div className="wd-bar" role="group" aria-label={`Skalierung Insel ${n}`}>
        {FACTORS.map((f) => (
          <Text as="label" size="2" key={f.key} className="wd-field">
            {f.label}
            <TextField.Root className="wd-slot" type="number" step="0.05" aria-label={`Insel ${n} ${f.label}`} value={isl.depthScaling[f.key]}
              onChange={(e) => { const v = Number(e.target.value); if (!Number.isNaN(v)) set({ depthScaling: { ...isl.depthScaling, [f.key]: v } }); }} />
          </Text>
        ))}
      </div>
      <FieldError world={world} island={i} field="depthScaling" />
    </li>
  );
}

/** Welt-Entwurf im Welten Editor (WZ18.6, B-488/AC-05): Name und Notizen der Welt, Inseln anlegen, umordnen, entfernen
 *  und bearbeiten. Jede Änderung wird sofort geprüft (Fehler am Feld, Vorschau der Datei); Speichern nur gültig, auf dem
 *  Stand der geladenen Datei (Stale: der Entwurf bleibt), danach der REG-Hinweis. */
export function WorldDraftEditor({ world }: { world: WorldDraftState }) {
  const { draft, check, notice, dirty } = world;
  if (!draft) return null;
  return (
    <section className="wd-plan" aria-label="Welt-Entwurf">
      <div className="wd-bar">
        <Text size="2" weight="bold">Welt-Entwurf</Text>
        <span className="wd-grow" />
        <Button variant="soft" title="Neue Insel als Kopie der letzten (Tiefen, Skalierung), ohne Planung" onClick={() => world.edit(addIsland(draft))}>Insel +</Button>
        <Button disabled={!dirty || !check.ok || world.lock !== undefined} title={world.lock} onClick={() => void world.save()}>Welt speichern</Button>
        <Button variant="outline" disabled={!dirty} onClick={world.discard}>Entwurf verwerfen</Button>
      </div>
      {notice && <Text as="p" size="2" role="status" color={notice.kind === 'hint' ? 'green' : 'red'} className="wd-notice">{notice.text}{notice.kind === 'stale' ? ' (Entwurf bleibt)' : ''}</Text>}
      <WorldHead world={world} />
      <ol className="wd-islands" aria-label="Inseln der Welt">
        {draft.islands.map((_, i) => <IslandRow key={i} world={world} i={i} />)}
      </ol>
      {dirty && check.preview && (
        <details>
          <summary>Vorschau der Welt-Datei</summary>
          <pre aria-label="Vorschau der Welt-Datei">{check.preview}</pre>
        </details>
      )}
    </section>
  );
}
