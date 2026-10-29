import { Checkbox, SelectField, Slider, Text } from '@webkit/atoms';
import { CheckField } from '@webkit/molecules';
import { createPortal } from 'react-dom';
import { useEffect, useMemo, useRef, useState, type CSSProperties, type ReactNode, type RefObject } from 'react';
import type { World } from '@game/model/types';
import { backend, type SiteLook, type WorldPreviewView } from '../api';
import { useDebounced } from '@webkit/lib/useDebounced';
import type { Look, Marks, PreviewStage, SiteHit, StagePosition } from './PreviewStage';
import { stagePx } from './hudBar';
import { BuildingFilter } from './BuildingFilter';
import { BuildingInspector } from './BuildingInspector';
import { backgroundToggles, buildingToggles, type BuildingToggle, focusXs, hubLevelChoices, pickKeys, previewArgsOf, worldOf, type PlanSource, type RowFocus } from './stageLogic';
import { MAX_PROGRESS, TIMES_OF_DAY, brightnessOf, simTime, type TimeOfDayId } from './timeOfDay';

const NONE: ReadonlySet<string> = new Set();
type Looks = Readonly<Record<string, SiteLook>>;
const NO_LOOKS: Looks = {};
const NO_SEL: readonly string[] = [];
const TIME_OPTIONS = TIMES_OF_DAY.map((t) => ({ value: t.id, label: t.label }));

interface Answer { key: string; off: string; view: WorldPreviewView }

/** Vorschau-Welt zu Seed, Biom, Hub-Stufe und Tageszeit. `all` hat alle Gebäude (daraus die Schalter), `view` die Auswahl.
 *  Die Auswahl gehört zu Seed, Biom, Hub-Stufe und Planer-Stand (`ctx`): wechselt eines davon, gilt sie sofort als leer. Jede Antwort
 *  trägt ihren Schlüssel, eine veraltete wird nie gezeigt; bis die neue da ist, bleibt die letzte Welt stehen. */
function usePreviewWorld(seed: string, biome: string, hubLevel: number, time: { phase: 'day' | 'dusk' | 'night' | 'dawn'; progress: number }, plan: PlanSource | undefined, candidate: string) {
  const ctx = `${seed}|${biome}|${hubLevel}|${candidate}|${JSON.stringify(plan ?? null)}`; // ein anderer Planer-Stand oder Entwurf leert die Auswahl wie eine andere Stufe
  const key = `${ctx}|${time.phase}|${time.progress}`;
  const [chosen, setChosen] = useState<{ ctx: string; off: ReadonlySet<string>; looks: Looks; sel: readonly string[] }>({ ctx, off: NONE, looks: NO_LOOKS, sel: NO_SEL });
  const off = chosen.ctx === ctx ? chosen.off : NONE;
  const looks = chosen.ctx === ctx ? chosen.looks : NO_LOOKS;
  const sel = chosen.ctx === ctx ? chosen.sel : NO_SEL; // gewählte Bauplätze (Inspektor, B-666)
  const changed = off.size > 0 || Object.keys(looks).length > 0;
  const offKey = `${[...off].sort().join(',')}|${JSON.stringify(looks)}`;
  const [all, setAll] = useState<Answer | null>(null);
  const [part, setPart] = useState<Answer | null>(null);
  const [shown, setShown] = useState<WorldPreviewView | null>(null);

  useEffect(() => {
    let current = true;
    void backend.worldPreview(previewArgsOf(seed, biome, hubLevel, NONE, time, plan, candidate)).then((view) => { if (current) setAll({ key, off: '', view }); });
    return () => { current = false; };
  }, [seed, biome, hubLevel, time, key, plan, candidate]);

  useEffect(() => {
    if (!changed) return;
    let current = true;
    void backend.worldPreview(previewArgsOf(seed, biome, hubLevel, off, time, plan, candidate, looks)).then((view) => { if (current) setPart({ key, off: offKey, view }); });
    return () => { current = false; };
  }, [seed, biome, hubLevel, time, key, off, looks, changed, offKey, plan, candidate]);

  const fresh = !changed ? (all?.key === key ? all.view : null) : (part?.key === key && part.off === offKey ? part.view : null);
  useEffect(() => { if (fresh) setShown(fresh); }, [fresh]);

  const setMany = (names: readonly string[], on: boolean) => setChosen(() => {
    const next = new Set(off);
    for (const name of names) { if (on) next.delete(name); else next.add(name); }
    return { ctx, off: next, looks, sel };
  });
  /** Zustand eines Platzes setzen; ein leerer Zustand entfernt den Eintrag */
  const setLook = (name: string, look: SiteLook) => setChosen(() => {
    const next = { ...looks };
    if (look.level || look.hpPercent || look.unbuilt) next[name] = look; else delete next[name];
    return { ctx, off, looks: next, sel };
  });
  /** Zustand aller Plätze auf einmal setzen (Voreinstellungen, Auswahl, Zurücksetzen) */
  const setLooks = (next: Looks) => setChosen(() => ({ ctx, off, looks: next, sel }));
  const setSel = (keys: readonly string[]) => setChosen(() => ({ ctx, off, looks, sel: keys }));
  return { all: all?.view ?? null, view: shown, off, looks, sel, setMany, setLook, setLooks, setSel };
}

/** Phaser wird erst beim ersten Sichtbarwerden geladen (eigener Baustein, groß) und baut die Bühne im Element auf; danach
 *  zeichnet sie jede neue Welt mit der aktuellen Darstellung (`look`). Beim Verlassen wird sie abgebaut. Liefert Grafiken,
 *  die nicht geladen werden konnten. */
function useStage(host: RefObject<HTMLDivElement | null>, world: World | null, look: Look, position: StagePosition, marks: Marks, jump: { key: string; x: number } | null, onPan: (px: number) => void, onWheel: (e: WheelEvent) => void, onSite?: (hit: SiteHit | null) => void): string[] {
  const stage = useRef<PreviewStage | null>(null);
  const latest = useRef({ world, look, position, marks });
  latest.current = { world, look, position, marks };
  const pan = useRef(onPan);
  pan.current = onPan;
  const wheel = useRef(onWheel);
  wheel.current = onWheel;
  const site = useRef(onSite);
  site.current = onSite;
  const [failed, setFailed] = useState<string[]>([]);

  useEffect(() => {
    let cancelled = false;
    void import('./PreviewStage').then(({ PreviewStage: Stage }) => {
      if (cancelled || stage.current || !host.current) return;
      stage.current = new Stage(host.current, setFailed, (px) => pan.current(px), (e) => wheel.current(e), (hit) => site.current?.(hit));
      stage.current.setLook(latest.current.look);
      stage.current.setPosition(latest.current.position);
      stage.current.setMarks?.(latest.current.marks);
      stage.current.show(latest.current.world);
    });
    return () => { cancelled = true; };
  }, [host]);
  useEffect(() => () => { stage.current?.destroy(); stage.current = null; }, []);
  useEffect(() => { stage.current?.show(world); }, [world]);
  useEffect(() => { stage.current?.setLook(look); }, [look]);
  useEffect(() => { stage.current?.setPosition(position); }, [position]);
  useEffect(() => { stage.current?.setMarks?.(marks); }, [marks]);
  // Springt zum gewählten Platz, sobald die Auswahl oder sein Ort wechselt; Ziehen danach bleibt unberührt (der Schlüssel der Auswahl ist der Auslöser).
  useEffect(() => { if (jump) stage.current?.focusX?.(jump.x); }, [jump?.key]); // eslint-disable-line react-hooks/exhaustive-deps
  // Nach jedem Rendern (Zeilen, Meldungen, Objekte über der Bühne) Größe und Lage prüfen: Beobachter melden erst im
  // nächsten Bild-Takt (bei verdecktem Fenster nie); die Messung erzwingt das fertige Layout (WZ17.5). Optional, weil
  // Test-Attrappen der Bühne settle nicht kennen.
  useEffect(() => { stage.current?.settle?.(); });
  return failed;
}

/** Höhe der Bühne: Leiste des globalen HUDs plus Weltausschnitt wie im Spiel (stage/hudBar.ts), mit dem Zoom skaliert. Der Zoom
 *  verkleinert also die ganze Vorschau; sichtbar bleibt immer die volle Spielhöhe, nur die Breite wächst mit. */
export function stageStyle(scale: number): CSSProperties {
  return { flex: 'none', minHeight: 0, height: stagePx(scale) };
}

/** Tageszeit (vier Einträge, Fortschritt) der Live-Vorschau */
function TimeControls({ timeId, setTimeId, progress, setProgress }: { timeId: TimeOfDayId; setTimeId: (id: TimeOfDayId) => void; progress: number; setProgress: (p: number) => void }) {
  return (
    <>
      <Text size="2">Tageszeit</Text>
      <SelectField aria-label="Tageszeit" value={timeId} options={TIME_OPTIONS} onChange={setTimeId} />
      <Slider className="wd-progress" aria-label="Fortschritt" min={0} max={MAX_PROGRESS} step={0.01} value={[progress]} onValueChange={([v]) => v !== undefined && setProgress(v)} />
    </>
  );
}

/** Hintergrund-Schichten der Live-Vorschau (ein Schalter je Schicht) */
function LayerControls({ biome, hidden, toggleLayer }: { biome: string; hidden: ReadonlySet<string>; toggleLayer: (key: string, on: boolean) => void }) {
  const layers = useMemo(() => backgroundToggles(biome), [biome]);
  return (
    <>
      {layers.length === 0 && (
        <Text as="label" size="1" color="gray">
          <Checkbox disabled checked={false} aria-label="Hintergrund-Schichten" /> keine Hintergrund-Schichten in diesem Biom
        </Text>
      )}
      {layers.map((l) => (
        <CheckField size="1" key={l.key} title={`Scroll-Faktor ${l.scroll}`} label={l.label} checked={!hidden.has(l.key)} onCheckedChange={(c) => toggleLayer(l.key, c === true)} aria-label={l.key} />
      ))}
    </>
  );
}

/** Meldung der Vorschau: Fehler der Welt, sonst die Grafiken, die nicht geladen werden konnten */
const noteOf = (error: string | undefined, failed: readonly string[]) => error || (failed.length > 0 ? `Grafiken nicht geladen: ${failed.join(', ')}` : '');

/** Bedienelemente an ihrem Ort: ohne Zielelement (undefined) neben der Bühne, mit Ziel dort (ein noch fehlendes Ziel zeigt nichts) */
const into = (node: ReactNode, host: HTMLElement | null | undefined) => (host === undefined ? node : host && createPortal(node, host));

/** Ausgeblendete Hintergrund-Schichten; die Wahl gilt nur für das Biom, zu dem sie gehört */
function useHiddenLayers(biome: string) {
  const [hiddenFor, setHiddenFor] = useState<{ biome: string; hidden: ReadonlySet<string> }>({ biome, hidden: NONE });
  const hidden = hiddenFor.biome === biome ? hiddenFor.hidden : NONE;
  const toggleLayer = (key: string, on: boolean) => setHiddenFor(() => {
    const next = new Set(hidden);
    if (on) next.delete(key); else next.add(key);
    return { biome, hidden: next };
  });
  return { hidden, toggleLayer };
}

/** Auswahl der Bauplätze im Inspektor (B-666, B-667): x-Stellen der gewählten Plätze für den Rahmen, Sprung nach einem Klick auf
 *  einen Chip (`select`) und der Klick auf die Bühne (`onSite`, nur mit Inspektor); `onPickSite` meldet der Seite eine Wahl in der Bühne. */
function useSiteSelection(inspector: boolean, toggles: readonly BuildingToggle[], off: ReadonlySet<string>, sel: readonly string[], setSel: (keys: readonly string[]) => void, onPickSite?: () => void) {
  const [pickJump, setPickJump] = useState<{ key: string; x: number } | null>(null);
  const picked = useMemo(() => sel.map((k) => toggles.find((t) => t.key === k)?.x).filter((x): x is number => x !== undefined), [sel, toggles]);
  /** Klick auf einen Bauplatz-Chip: gewählt und die Vorschau springt hin; die Auswahl aus der Gruppenzeile springt nicht */
  const select = (keys: string[], jumpThere: boolean) => {
    setSel(keys);
    const x = toggles.find((t) => t.key === keys[0])?.x;
    if (jumpThere && x !== undefined) setPickJump({ key: `${keys[0]}|${Date.now()}`, x });
  };
  const onSite = inspector ? (hit: SiteHit | null) => { const keys = pickKeys(hit, sel, toggles, off); setSel(keys); if (keys.length > 0) onPickSite?.(); } : undefined;
  return { picked, pickJump, select, onSite };
}

/** Planer-Stand für die Live-Vorschau (Gebäude-Planer, B-462/AC-04): Kandidaten-hub.json, Anzahl ihrer Hub-Stufen und
 *  die gewählte Stufe, gesteuert von der Seite. */
export interface PlanerSource {
  candidate: string;
  levels: number;
  hubLevel: number;
  onHubLevel: (level: number) => void;
  /** gewählte Zeile der Matrix: Rahmen und Sprung in der Vorschau; Namen über allen Plätzen zeigt die Vorschau immer */
  focus?: RowFocus;
}

/** Wählbare Hub-Stufen und die gewählte: aus `data/hub.json`, mit Planer-Stand aus dessen Stufen und gesteuert von der Seite. */
function useHubLevel(planer: PlanerSource | undefined) {
  const levels = planer?.levels;
  const choices = useMemo(() => (levels === undefined ? hubLevelChoices() : hubLevelChoices(Array.from({ length: levels }))), [levels]);
  const [ownLevel, setOwnLevel] = useState(choices.at(-1) ?? 1);
  return { choices, hubLevel: planer?.hubLevel ?? ownLevel, setHubLevel: planer?.onHubLevel ?? setOwnLevel };
}

/** Markierungen und Sprungziel der gewählten Planer-Zeile: Namen über allen Plätzen mit Planer, Rahmen und Sprung zur Zeile;
 *  `picked` sind die x-Stellen der im Inspektor gewählten Bauplätze (Rahmen), `pickJump` der Sprung nach einem Klick auf ihren Chip. */
function usePlanerMarks(planer: PlanerSource | undefined, world: ReturnType<typeof worldOf>, picked: readonly number[], pickJump: { key: string; x: number } | null) {
  const selectedXs = useMemo(() => focusXs(planer?.focus, world?.hubX ?? 0, world?.sites ?? []), [planer?.focus, world]);
  const marks = useMemo<Marks>(() => ({ names: planer !== undefined, selected: [...selectedXs, ...picked] }), [planer, selectedXs, picked]);
  const jumpX = selectedXs[0];
  const jump = useMemo(() => (jumpX === undefined ? pickJump : { key: `${planer?.focus?.id}|${jumpX}`, x: jumpX }), [jumpX, planer?.focus?.id, pickJump]);
  return { marks, jump };
}

/** Live-Vorschau des Welt-Betrachters (B-460/AC-04, AC-05): die Welt des Spiels mit der Spielgrafik auf der Phaser-Bühne
 *  des Studios. Hub-Stufe aus `data/hub.json` › `levels` (mit `planer` aus dem Planer-Stand), Gebäude einzeln ein und aus,
 *  Tageszeit, Hintergrund-Schichten mit Parallaxe (Kamera scrollt). Position und Zoom kommen von außen (`position`, B-477,
 *  gleich wie im Chunk-Streifen); Ziehen meldet `onPan` (Pixel), das Mausrad geht an `onWheel`; `onSites` meldet die Bauplätze
 *  (für die Sprungpunkte). Mit `plan` zeigt sie den Entwurf der geplanten Chunks (B-461/AC-11). */
export function LivePreview({ seed, biome, position, onPan, onWheel, onSites, plan, planer, controlsHost, treeHost, layersHost, onPickSite }: {
  seed: string; biome: string; position: StagePosition; onPan: (px: number) => void; onWheel: (e: WheelEvent) => void;
  onSites?: (sites: readonly { kind: string; x: number }[]) => void; plan?: PlanSource; planer?: PlanerSource;
  /** Zielelemente der Seite: Hub-Stufe, Tageszeit und Schichten (`controlsHost`) und der Gebäude-Baum (`treeHost`) erscheinen dort statt um die Bühne */
  controlsHost?: HTMLElement | null; treeHost?: HTMLElement | null;
  /** Zielelement der Hintergrund-Schichten (Reiter Ansicht des Inspektors); ohne es stehen sie in der Steuerzeile */
  layersHost?: HTMLElement | null;
  /** Ein Klick wählte einen Bauplatz in der Bühne: die Seite öffnet den Reiter Gebäude (B-667) */
  onPickSite?: () => void;
}) {
  const { choices, hubLevel, setHubLevel } = useHubLevel(planer);
  const [timeId, setTimeId] = useState<TimeOfDayId>('day');
  const [progress, setProgress] = useState(0);
  const settled = useDebounced(progress, 120);
  const time = useMemo(() => simTime(timeId, settled), [timeId, settled]);
  const { hidden, toggleLayer } = useHiddenLayers(biome);
  const { all, view, off, looks, sel, setMany, setLook, setLooks, setSel } = usePreviewWorld(seed, biome, hubLevel, time, plan, planer?.candidate ?? '');
  const host = useRef<HTMLDivElement>(null);
  const world = useMemo(() => worldOf(view), [view]);
  const look = useMemo<Look>(() => ({ brightness: world ? brightnessOf(world.cycle) : 1, hiddenLayers: hidden }), [world, hidden]);
  const sites = all?.state?.sites;
  const toggles = useMemo(() => buildingToggles(sites ?? []), [sites]);
  const { picked, pickJump, select, onSite } = useSiteSelection(treeHost !== undefined, toggles, off, sel, setSel, onPickSite);
  const { marks, jump } = usePlanerMarks(planer, world, picked, pickJump);
  const failed = useStage(host, world, look, position, marks, jump, onPan, onWheel, onSite);
  useEffect(() => { onSites?.(sites ?? []); }, [sites, onSites]);
  const note = noteOf(view?.error, failed);

  const layers = <LayerControls biome={biome} hidden={hidden} toggleLayer={toggleLayer} />;
  const controls = (
    <div className="wd-bar">
      <Text size="2">Hub-Stufe</Text>
      <SelectField aria-label="Hub-Stufe" value={String(hubLevel)} options={choices.map((n) => ({ value: String(n), label: String(n) }))} onChange={(v) => setHubLevel(Number(v))} />
      <TimeControls timeId={timeId} setTimeId={setTimeId} progress={progress} setProgress={setProgress} />
      {layersHost === undefined && layers}
      {note && <Text size="2" color="red" role="status">{note}</Text>}
    </div>
  );
  // Im Inspektor (treeHost) die Bedienung mit Voreinstellungen und Auswahl, sonst der Filter mit Listen je Platz
  const tree = treeHost === undefined
    ? <BuildingFilter toggles={toggles} off={off} onSet={setMany} looks={looks} onLook={setLook} />
    : <BuildingInspector toggles={toggles} off={off} onSet={setMany} looks={looks} onLooks={setLooks} selected={sel} onSelect={select} />;
  return (
    <div className="wd-live-body">
      {into(controls, controlsHost)}
      <div className="wd-stage" ref={host} style={stageStyle(position.scale)} />
      {into(tree, treeHost)}
      {layersHost && createPortal(layers, layersHost)}
    </div>
  );
}
