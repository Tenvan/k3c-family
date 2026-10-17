import { Checkbox, Select, Text } from '@radix-ui/themes';
import { useEffect, useMemo, useRef, useState, type RefObject } from 'react';
import type { World } from '@game/model/types';
import { backend, type WorldPreviewView } from '../api';
import type { PreviewStage } from './PreviewStage';
import { buildingToggles, hubLevelChoices, previewArgsOf, worldOf } from './stageLogic';

const NONE: ReadonlySet<string> = new Set();

interface Answer { ctx: string; off: string; view: WorldPreviewView }

/** Vorschau-Welt zu Seed, Biom und Hub-Stufe. `all` hat alle Gebäude (daraus die Schalter), `view` die Auswahl. Die
 *  Auswahl gehört zu Seed, Biom und Hub-Stufe (`ctx`): wechselt eines davon, gilt sie sofort als leer. Jede Antwort
 *  trägt ihren Kontext, eine veraltete wird nie gezeigt; bis die neue da ist, bleibt die letzte Welt stehen. */
function usePreviewWorld(seed: string, biome: string, hubLevel: number) {
  const ctx = `${seed}|${biome}|${hubLevel}`;
  const [chosen, setChosen] = useState<{ ctx: string; off: ReadonlySet<string> }>({ ctx, off: NONE });
  const off = chosen.ctx === ctx ? chosen.off : NONE;
  const offKey = [...off].sort().join(',');
  const [all, setAll] = useState<Answer | null>(null);
  const [part, setPart] = useState<Answer | null>(null);
  const [shown, setShown] = useState<WorldPreviewView | null>(null);

  useEffect(() => {
    let current = true;
    void backend.worldPreview(previewArgsOf(seed, biome, hubLevel, NONE)).then((view) => { if (current) setAll({ ctx, off: '', view }); });
    return () => { current = false; };
  }, [seed, biome, hubLevel, ctx]);

  useEffect(() => {
    if (off.size === 0) return;
    let current = true;
    void backend.worldPreview(previewArgsOf(seed, biome, hubLevel, off)).then((view) => { if (current) setPart({ ctx, off: offKey, view }); });
    return () => { current = false; };
  }, [seed, biome, hubLevel, ctx, off, offKey]);

  const fresh = off.size === 0 ? (all?.ctx === ctx ? all.view : null) : (part?.ctx === ctx && part.off === offKey ? part.view : null);
  useEffect(() => { if (fresh) setShown(fresh); }, [fresh]);

  const toggle = (key: string, on: boolean) => setChosen(() => {
    const next = new Set(off);
    if (on) next.delete(key); else next.add(key);
    return { ctx, off: next };
  });
  return { all: all?.view ?? null, view: shown, off, toggle };
}

/** Phaser wird erst beim ersten Sichtbarwerden geladen (eigener Baustein, groß) und baut die Bühne im Element auf; danach
 *  zeichnet sie jede neue Welt. Beim Verlassen wird sie abgebaut. Liefert Grafiken, die nicht geladen werden konnten. */
function useStage(host: RefObject<HTMLDivElement | null>, active: boolean, world: World | null): string[] {
  const stage = useRef<PreviewStage | null>(null);
  const worldRef = useRef(world);
  worldRef.current = world;
  const [failed, setFailed] = useState<string[]>([]);

  useEffect(() => {
    if (!active) return;
    if (stage.current) { stage.current.refresh(); return; }
    let cancelled = false;
    void import('./PreviewStage').then(({ PreviewStage: Stage }) => {
      if (cancelled || stage.current || !host.current) return;
      stage.current = new Stage(host.current, setFailed);
      stage.current.show(worldRef.current);
    });
    return () => { cancelled = true; };
  }, [active, host]);
  useEffect(() => () => { stage.current?.destroy(); stage.current = null; }, []);
  useEffect(() => { stage.current?.show(world); }, [world]);
  return failed;
}

/** Live-Vorschau des Welt-Betrachters (B-460/AC-04): die Welt des Spiels mit der Spielgrafik auf der Phaser-Bühne des
 *  Studios. Hub-Stufe aus `data/hub.json` › `levels`, Gebäude einzeln ein und aus. `active` = sichtbar. */
export function LivePreview({ seed, biome, active }: { seed: string; biome: string; active: boolean }) {
  const choices = useMemo(() => hubLevelChoices(), []);
  const [hubLevel, setHubLevel] = useState(choices.at(-1) ?? 1);
  const { all, view, off, toggle } = usePreviewWorld(seed, biome, hubLevel);
  const host = useRef<HTMLDivElement>(null);
  const world = useMemo(() => worldOf(view), [view]);
  const failed = useStage(host, active, world);
  const toggles = useMemo(() => buildingToggles(all?.state?.sites ?? []), [all]);
  const note = view?.error || (failed.length > 0 ? `Grafiken nicht geladen: ${failed.join(', ')}` : '');

  return (
    <div className="wd-live-body">
      <div className="wd-bar">
        <Text size="2">Hub-Stufe</Text>
        <Select.Root value={String(hubLevel)} onValueChange={(v) => setHubLevel(Number(v))}>
          <Select.Trigger aria-label="Hub-Stufe" />
          <Select.Content>{choices.map((n) => <Select.Item key={n} value={String(n)}>{n}</Select.Item>)}</Select.Content>
        </Select.Root>
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
