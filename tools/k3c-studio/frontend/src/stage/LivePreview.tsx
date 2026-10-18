import { Checkbox, Select, Text } from '@radix-ui/themes';
import { useEffect, useMemo, useRef, useState, type RefObject } from 'react';
import type { World } from '@game/model/types';
import { backend, type WorldPreviewView } from '../api';
import { useDebounced } from '../lib/useDebounced';
import type { Look, PreviewStage, StagePosition } from './PreviewStage';
import { backgroundToggles, buildingToggles, hubLevelChoices, previewArgsOf, worldOf } from './stageLogic';
import { MAX_PROGRESS, TIMES_OF_DAY, brightnessOf, simTime, type TimeOfDayId } from './timeOfDay';

const NONE: ReadonlySet<string> = new Set();

interface Answer { key: string; off: string; view: WorldPreviewView }

/** Vorschau-Welt zu Seed, Biom, Hub-Stufe und Tageszeit. `all` hat alle Gebäude (daraus die Schalter), `view` die Auswahl.
 *  Die Auswahl gehört zu Seed, Biom und Hub-Stufe (`ctx`): wechselt eines davon, gilt sie sofort als leer. Jede Antwort
 *  trägt ihren Schlüssel, eine veraltete wird nie gezeigt; bis die neue da ist, bleibt die letzte Welt stehen. */
function usePreviewWorld(seed: string, biome: string, hubLevel: number, time: { phase: 'day' | 'dusk' | 'night'; progress: number }) {
  const ctx = `${seed}|${biome}|${hubLevel}`;
  const key = `${ctx}|${time.phase}|${time.progress}`;
  const [chosen, setChosen] = useState<{ ctx: string; off: ReadonlySet<string> }>({ ctx, off: NONE });
  const off = chosen.ctx === ctx ? chosen.off : NONE;
  const offKey = [...off].sort().join(',');
  const [all, setAll] = useState<Answer | null>(null);
  const [part, setPart] = useState<Answer | null>(null);
  const [shown, setShown] = useState<WorldPreviewView | null>(null);

  useEffect(() => {
    let current = true;
    void backend.worldPreview(previewArgsOf(seed, biome, hubLevel, NONE, time)).then((view) => { if (current) setAll({ key, off: '', view }); });
    return () => { current = false; };
  }, [seed, biome, hubLevel, time, key]);

  useEffect(() => {
    if (off.size === 0) return;
    let current = true;
    void backend.worldPreview(previewArgsOf(seed, biome, hubLevel, off, time)).then((view) => { if (current) setPart({ key, off: offKey, view }); });
    return () => { current = false; };
  }, [seed, biome, hubLevel, time, key, off, offKey]);

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
function useStage(host: RefObject<HTMLDivElement | null>, world: World | null, look: Look, position: StagePosition, onPan: (px: number) => void): string[] {
  const stage = useRef<PreviewStage | null>(null);
  const latest = useRef({ world, look, position });
  latest.current = { world, look, position };
  const pan = useRef(onPan);
  pan.current = onPan;
  const [failed, setFailed] = useState<string[]>([]);

  useEffect(() => {
    let cancelled = false;
    void import('./PreviewStage').then(({ PreviewStage: Stage }) => {
      if (cancelled || stage.current || !host.current) return;
      stage.current = new Stage(host.current, setFailed, (px) => pan.current(px));
      stage.current.setLook(latest.current.look);
      stage.current.setPosition(latest.current.position);
      stage.current.show(latest.current.world);
    });
    return () => { cancelled = true; };
  }, [host]);
  useEffect(() => () => { stage.current?.destroy(); stage.current = null; }, []);
  useEffect(() => { stage.current?.show(world); }, [world]);
  useEffect(() => { stage.current?.setLook(look); }, [look]);
  useEffect(() => { stage.current?.setPosition(position); }, [position]);
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

/** Live-Vorschau des Welt-Betrachters (B-460/AC-04, AC-05): die Welt des Spiels mit der Spielgrafik auf der Phaser-Bühne
 *  des Studios. Hub-Stufe aus `data/hub.json` › `levels`, Gebäude einzeln ein und aus, Tageszeit, Hintergrund-Schichten
 *  mit Parallaxe (Kamera scrollt). Position und Zoom kommen von außen (`position`, B-477, gleich wie im Chunk-Streifen);
 *  Ziehen und Mausrad melden `onPan` (Pixel); `onSites` meldet die Bauplätze (für die Sprungpunkte). */
export function LivePreview({ seed, biome, position, onPan, onSites }: {
  seed: string; biome: string; position: StagePosition; onPan: (px: number) => void; onSites?: (sites: readonly { kind: string; x: number }[]) => void;
}) {
  const choices = useMemo(() => hubLevelChoices(), []);
  const [hubLevel, setHubLevel] = useState(choices.at(-1) ?? 1);
  const [timeId, setTimeId] = useState<TimeOfDayId>('day');
  const [progress, setProgress] = useState(0);
  const settled = useDebounced(progress, 120);
  const time = useMemo(() => simTime(timeId, settled), [timeId, settled]);
  const [hiddenFor, setHiddenFor] = useState<{ biome: string; hidden: ReadonlySet<string> }>({ biome, hidden: NONE });
  const hidden = hiddenFor.biome === biome ? hiddenFor.hidden : NONE;
  const { all, view, off, toggle } = usePreviewWorld(seed, biome, hubLevel, time);
  const host = useRef<HTMLDivElement>(null);
  const world = useMemo(() => worldOf(view), [view]);
  const look = useMemo<Look>(() => ({ brightness: world ? brightnessOf(world.cycle) : 1, hiddenLayers: hidden }), [world, hidden]);
  const failed = useStage(host, world, look, position, onPan);
  const sites = all?.state?.sites;
  const toggles = useMemo(() => buildingToggles(sites ?? []), [sites]);
  useEffect(() => { onSites?.(sites ?? []); }, [sites, onSites]);
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
