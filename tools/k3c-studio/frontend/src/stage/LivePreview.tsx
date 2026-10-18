import { Checkbox, Select, Text } from '@radix-ui/themes';
import { useEffect, useMemo, useRef, useState, type RefObject } from 'react';
import type { World } from '@game/model/types';
import { backend, type WorldPreviewView } from '../api';
import { useDebounced } from '../lib/useDebounced';
import type { Look, PreviewStage } from './PreviewStage';
import { backgroundToggles, buildingToggles, hubLevelChoices, previewArgsOf, worldOf } from './stageLogic';
import { MAX_PROGRESS, TIMES_OF_DAY, brightnessOf, simTime, type TimeOfDayId } from './timeOfDay';

const NONE: ReadonlySet<string> = new Set();

interface Answer { key: string; off: string; view: WorldPreviewView }

/** Vorschau-Welt zu Seed, Biom, Hub-Stufe und Tageszeit. `all` hat alle Gebäude (daraus die Schalter), `view` die Auswahl.
 *  Die Auswahl gehört zu Seed, Biom, Hub-Stufe und Planer-Stand (`ctx`): wechselt eines davon, gilt sie sofort als leer. Jede Antwort
 *  trägt ihren Schlüssel, eine veraltete wird nie gezeigt; bis die neue da ist, bleibt die letzte Welt stehen. */
function usePreviewWorld(seed: string, biome: string, hubLevel: number, time: { phase: 'day' | 'dusk' | 'night'; progress: number }, candidate: string) {
  const ctx = `${seed}|${biome}|${hubLevel}|${candidate}`; // ein anderer Planer-Stand leert die Auswahl wie eine andere Stufe
  const key = `${ctx}|${time.phase}|${time.progress}`;
  const [chosen, setChosen] = useState<{ ctx: string; off: ReadonlySet<string> }>({ ctx, off: NONE });
  const off = chosen.ctx === ctx ? chosen.off : NONE;
  const offKey = [...off].sort().join(',');
  const [all, setAll] = useState<Answer | null>(null);
  const [part, setPart] = useState<Answer | null>(null);
  const [shown, setShown] = useState<WorldPreviewView | null>(null);

  useEffect(() => {
    let current = true;
    void backend.worldPreview(previewArgsOf(seed, biome, hubLevel, NONE, time, candidate)).then((view) => { if (current) setAll({ key, off: '', view }); });
    return () => { current = false; };
  }, [seed, biome, hubLevel, time, candidate, key]);

  useEffect(() => {
    if (off.size === 0) return;
    let current = true;
    void backend.worldPreview(previewArgsOf(seed, biome, hubLevel, off, time, candidate)).then((view) => { if (current) setPart({ key, off: offKey, view }); });
    return () => { current = false; };
  }, [seed, biome, hubLevel, time, candidate, key, off, offKey]);

  const fresh = off.size === 0 ? (all?.key === key ? all.view : null) : (part?.key === key && part.off === offKey ? part.view : null);
  useEffect(() => { if (fresh) setShown(fresh); }, [fresh]);

  const toggle = (name: string, on: boolean) => setChosen(() => {
    const next = new Set(off);
    if (on) next.delete(name); else next.add(name);
    return { ctx, off: next };
  });
  return { all: all?.view ?? null, view: shown, off, toggle };
}

/** Phaser wird erst beim ersten Sichtbarwerden geladen (eigener Baustein, groß) und baut die Bühne im Element auf; danach
 *  zeichnet sie jede neue Welt mit der aktuellen Darstellung (`look`). Beim Verlassen wird sie abgebaut. Liefert Grafiken,
 *  die nicht geladen werden konnten. */
function useStage(host: RefObject<HTMLDivElement | null>, active: boolean, world: World | null, look: Look): string[] {
  const stage = useRef<PreviewStage | null>(null);
  const latest = useRef({ world, look });
  latest.current = { world, look };
  const [failed, setFailed] = useState<string[]>([]);

  useEffect(() => {
    if (!active) return;
    if (stage.current) { stage.current.refresh(); return; }
    let cancelled = false;
    void import('./PreviewStage').then(({ PreviewStage: Stage }) => {
      if (cancelled || stage.current || !host.current) return;
      stage.current = new Stage(host.current, setFailed);
      stage.current.setLook(latest.current.look);
      stage.current.show(latest.current.world);
    });
    return () => { cancelled = true; };
  }, [active, host]);
  useEffect(() => () => { stage.current?.destroy(); stage.current = null; }, []);
  useEffect(() => { stage.current?.show(world); }, [world]);
  useEffect(() => { stage.current?.setLook(look); }, [look]);
  return failed;
}

/** Tageszeit (vier Einträge, Fortschritt) und Hintergrund-Schichten der Live-Vorschau */
function LookControls({ biome, timeId, setTimeId, progress, setProgress, hidden, toggleLayer }: {
  biome: string; timeId: TimeOfDayId; setTimeId: (id: TimeOfDayId) => void; progress: number; setProgress: (p: number) => void;
  hidden: ReadonlySet<string>; toggleLayer: (key: string, on: boolean) => void;
}) {
  const layers = useMemo(() => backgroundToggles(biome), [biome]);
  return (
    <>
      <Text size="2">Tageszeit</Text>
      <select className="wd-select" aria-label="Tageszeit" value={timeId} onChange={(e) => setTimeId(e.target.value as TimeOfDayId)}>
        {TIMES_OF_DAY.map((t) => <option key={t.id} value={t.id}>{t.label}</option>)}
      </select>
      <input type="range" aria-label="Fortschritt" min={0} max={MAX_PROGRESS} step={0.01} value={progress} onChange={(e) => setProgress(Number(e.target.value))} />
      {layers.length === 0 && (
        <Text as="label" size="1" color="gray">
          <Checkbox disabled checked={false} aria-label="Hintergrund-Schichten" /> keine Hintergrund-Schichten in diesem Biom
        </Text>
      )}
      {layers.map((l) => (
        <Text as="label" size="1" key={l.key} title={`Scroll-Faktor ${l.scroll}`}>
          <Checkbox checked={!hidden.has(l.key)} onCheckedChange={(c) => toggleLayer(l.key, c === true)} aria-label={l.key} /> {l.label}
        </Text>
      ))}
    </>
  );
}

/** Planer-Stand für die Live-Vorschau (Gebäude-Planer, B-462/AC-04): Kandidaten-hub.json, Anzahl ihrer Hub-Stufen und
 *  die gewählte Stufe, gesteuert von der Seite. */
export interface PlanerSource {
  candidate: string;
  levels: number;
  hubLevel: number;
  onHubLevel: (level: number) => void;
}

/** Live-Vorschau des Welt-Betrachters (B-460/AC-04, AC-05): die Welt des Spiels mit der Spielgrafik auf der Phaser-Bühne
 *  des Studios. Hub-Stufe aus `data/hub.json` › `levels` (mit `planer` aus dem Planer-Stand), Gebäude einzeln ein und
 *  aus, Tageszeit, Hintergrund-Schichten mit Parallaxe (Kamera scrollt). `active` = sichtbar. */
export function LivePreview({ seed, biome, active, planer }: { seed: string; biome: string; active: boolean; planer?: PlanerSource }) {
  const levels = planer?.levels;
  const choices = useMemo(() => (levels === undefined ? hubLevelChoices() : hubLevelChoices(Array.from({ length: levels }))), [levels]);
  const [ownLevel, setOwnLevel] = useState(choices.at(-1) ?? 1);
  const hubLevel = planer?.hubLevel ?? ownLevel;
  const setHubLevel = planer?.onHubLevel ?? setOwnLevel;
  const [timeId, setTimeId] = useState<TimeOfDayId>('day');
  const [progress, setProgress] = useState(0);
  const settled = useDebounced(progress, 120);
  const time = useMemo(() => simTime(timeId, settled), [timeId, settled]);
  const [hiddenFor, setHiddenFor] = useState<{ biome: string; hidden: ReadonlySet<string> }>({ biome, hidden: NONE });
  const hidden = hiddenFor.biome === biome ? hiddenFor.hidden : NONE;
  const { all, view, off, toggle } = usePreviewWorld(seed, biome, hubLevel, time, planer?.candidate ?? '');
  const host = useRef<HTMLDivElement>(null);
  const world = useMemo(() => worldOf(view), [view]);
  const look = useMemo<Look>(() => ({ brightness: world ? brightnessOf(world.cycle) : 1, hiddenLayers: hidden }), [world, hidden]);
  const failed = useStage(host, active, world, look);
  const toggles = useMemo(() => buildingToggles(all?.state?.sites ?? []), [all]);
  const note = view?.error || (failed.length > 0 ? `Grafiken nicht geladen: ${failed.join(', ')}` : '');
  const toggleLayer = (key: string, on: boolean) => setHiddenFor(() => {
    const next = new Set(hidden);
    if (on) next.delete(key); else next.add(key);
    return { biome, hidden: next };
  });

  return (
    <div className="wd-live-body">
      <div className="wd-bar">
        <Text size="2">Hub-Stufe</Text>
        <Select.Root value={String(hubLevel)} onValueChange={(v) => setHubLevel(Number(v))}>
          <Select.Trigger aria-label="Hub-Stufe" />
          <Select.Content>{choices.map((n) => <Select.Item key={n} value={String(n)}>{n}</Select.Item>)}</Select.Content>
        </Select.Root>
        <LookControls biome={biome} timeId={timeId} setTimeId={setTimeId} progress={progress} setProgress={setProgress} hidden={hidden} toggleLayer={toggleLayer} />
        {note && <Text size="2" color="red" role="status">{note}</Text>}
      </div>
      <div className="wd-stage" ref={host} />
      <ul className="wd-buildings" aria-label="Gebäude">
        {toggles.map((b) => (
          <li key={b.key}>
            <Text as="label" size="1">
              <Checkbox checked={!off.has(b.key)} onCheckedChange={(c) => toggle(b.key, c === true)} aria-label={b.key} /> {b.kind} {b.x}
            </Text>
          </li>
        ))}
      </ul>
    </div>
  );
}
