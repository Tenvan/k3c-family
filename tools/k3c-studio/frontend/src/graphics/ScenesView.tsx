import { SegmentedControl, Select, SelectField, Text } from '@webkit/atoms';
import { useEffect, useMemo, useState } from 'react';
import type { ListFilterConfig } from '@webkit/lib/listFilter';
import { useAssets } from '../assets/useAssets';
import { loadJSON, loadPref, saveJSON, savePref } from '../lib/prefs';
import { ListFilter, NoticeCard } from '@webkit/molecules';
import { BIOMES } from '@game/model/biome';
import { CREDITS } from '@game/tools/credits';
import { BuildingStrip } from './BuildingStrip';
import { HubGrowthStrip } from './HubGrowthStrip';
import { Backgrounds } from './Backgrounds';
import { GameScene } from './GameScene';
import { GAME_SCENES, isGameScene, LICHT, type Licht } from './gameScenesLogic';
import { FigurenCards, ImageCard } from './SlotCards';
import { resolveSlot, sceneFilter, SCENES, slotsOf, type AssetSlot, type GebaeudeEintrag, type Query, type Resolved, type SceneId } from './scenesLogic';

// Reiter „Szenen“ der Seite Grafik (B-459/AC-06, Inhalt der früheren Asset-Vorschau): je Kategorie eine Mini-Szene mit den
// aktiven Zuordnungen aus AssetsList; lädt bei assets:changed neu. Vergleich: Präferenz n eines Slots statt Präferenz 1.

const SLOT_SCENES = SCENES.filter((s) => s.art === 'grafik').map((s) => s.id);
const GRAFIK: readonly string[] = [...SLOT_SCENES, ...GAME_SCENES];
const TITLE: Record<string, string> = { figuren: 'Figuren', gebaeude: 'Gebäude', umgebung: 'Umgebung', objekte: 'Objekte', hud: 'HUD', skills: 'Skill-Effekte', schilder: 'Schilder & Gerüste', gegner: 'Gegner je Biom', reittiere: 'Reittiere', bosse: 'Bosse', kampf: 'Kampf-Anzeigen' };
const NONE = 'keiner';
const BIOM_IDS = BIOMES.map((b) => b.id);

/** Gebäude-Index (schreibt `task gen:gebaeude`, GEB1); null = lädt noch, [] = fehlt */
function useGebaeudeIndex(): GebaeudeEintrag[] | null {
  const [index, setIndex] = useState<GebaeudeEintrag[] | null>(null);
  useEffect(() => {
    let live = true;
    fetch('grafik/gebaeude/index.json')
      .then((r) => (r.ok ? (r.json() as Promise<GebaeudeEintrag[]>) : []))
      .then((list) => live && setIndex(list), () => live && setIndex([]));
    return () => {
      live = false;
    };
  }, []);
  return index;
}

export function ScenesView() {
  const data = useAssets();
  const index = useGebaeudeIndex();
  const [szene, setSzene] = useState<string>(() => loadPref('grafik.szene', GRAFIK, 'figuren'));
  const [biom, setBiom] = useState<string>(() => loadPref('grafik.biom', BIOM_IDS, BIOM_IDS[0]!));
  const [licht, setLicht] = useState<Licht>(() => loadPref('grafik.licht', LICHT, 'tag'));
  const [q, setQ] = useState<Query>({ wahl: 1 });
  const slots = useMemo(() => (data.view ? slotsOf(data.view.slots, szene as SceneId) : []), [data.view, szene]);
  const items = useMemo(() => slots.map((s) => resolveSlot(s, q)), [slots, q]);
  const filter = useMemo(() => sceneFilter(items, CREDITS), [items]);
  if (!data.view) return data.error ? <NoticeCard title="Slot-Register nicht lesbar" tone="error">{data.error}</NoticeCard> : <Text color="gray">Lade Slot-Register …</Text>;
  const pick = (v: string) => {
    const next = GRAFIK.find((s) => s === v) ?? 'figuren';
    setSzene(next);
    setQ({ wahl: 1 });
    savePref('grafik.szene', next);
  };
  return (
    <div className="sz-page">
      <div className="as-head">
        <SegmentedControl.Root value={szene} onValueChange={pick}>
          {GRAFIK.map((s) => <SegmentedControl.Item key={s} value={s}>{TITLE[s]}</SegmentedControl.Item>)}
        </SegmentedControl.Root>
        <Vergleich slots={slots} q={q} onQ={setQ} />
        {isGameScene(szene) && <Umgebung biom={biom} licht={licht} onBiom={(b) => { setBiom(b); savePref('grafik.biom', b); }} onLicht={(l) => { setLicht(l); savePref('grafik.licht', l); }} />}
      </div>
      <Text size="2" color="gray">Aktive Zuordnungen im Spielmaßstab ×2; 1 Unit = 32 px. Lücken und fehlende Dateien sind markiert.</Text>
      {isGameScene(szene) ? <GameScene szene={szene} biom={biom} licht={licht} index={index ?? []} /> : <Scene szene={szene} items={items} filter={filter} index={index ?? []} />}
    </div>
  );
}

function Scene({ szene, items, filter, index }: { szene: string; items: Resolved[]; filter: ListFilterConfig<Resolved>; index: GebaeudeEintrag[] }) {
  return (
    <>
      {szene === 'gebaeude' && index.length > 0 && <><h3 className="sz-title">Neue Gebäude (GEB)</h3><BuildingStrip index={index} /></>}
      {szene === 'gebaeude' && index.length > 0 && <><h3 className="sz-title">Hub wächst (GEB11)</h3><HubGrowthStrip index={index} /></>}
      <ListFilter
        key={szene} items={items} {...filter} storageKey={`grafik.${szene}.filter`} prefs={{ loadJSON, saveJSON }}
        emptyHint="Keine Asset-Slots in dieser Kategorie." renderList={(list) => <Bilder szene={szene} items={list} />}
      />
      {szene === 'umgebung' && <Backgrounds index={index} />}
    </>
  );
}

function Bilder({ szene, items }: { szene: string; items: Resolved[] }) {
  const bilder = szene === 'figuren' ? items.filter((r) => !r.choice?.sheet) : items;
  return (
    <>
      {szene === 'figuren' && <FigurenCards title={TITLE.figuren!} items={items} />}
      {bilder.length > 0 && <div className={`sz-strip sz-${szene}`}>{bilder.map((r) => <ImageCard key={r.slot.id} r={r} />)}</div>}
    </>
  );
}

/** Vergleich (früher `?slot=<id>&wahl=<n>`): ein Slot mit mehreren Präferenzen zeigt Präferenz n. */
function Vergleich({ slots, q, onQ }: { slots: AssetSlot[]; q: Query; onQ: (q: Query) => void }) {
  const multi = slots.filter((s) => s.wahl.length > 1);
  if (multi.length === 0) return null;
  const slot = multi.find((s) => s.id === q.slot);
  return (
    <>
      <Select.Root value={slot?.id ?? NONE} onValueChange={(v) => onQ({ slot: v === NONE ? undefined : v, wahl: 1 })}>
        <Select.Trigger aria-label="Vergleich" />
        <Select.Content>
          <Select.Item value={NONE}>Vergleich: keiner</Select.Item>
          {multi.map((s) => <Select.Item key={s.id} value={s.id}>Vergleich: {s.id}</Select.Item>)}
        </Select.Content>
      </Select.Root>
      {slot && (
        <SegmentedControl.Root value={String(q.wahl)} onValueChange={(v) => onQ({ slot: slot.id, wahl: Number(v) })}>
          {slot.wahl.map((_, i) => <SegmentedControl.Item key={i} value={String(i + 1)}>Präf. {i + 1}</SegmentedControl.Item>)}
        </SegmentedControl.Root>
      )}
    </>
  );
}

/** Biom (Himmel, Boden) und Tageslicht der Spiel-Szenen (B-598/AC-03) */
function Umgebung({ biom, licht, onBiom, onLicht }: { biom: string; licht: Licht; onBiom: (b: string) => void; onLicht: (l: Licht) => void }) {
  return (
    <>
      <SelectField value={biom} aria-label="Biom" onChange={onBiom} options={BIOMES.map((b) => ({ value: b.id, label: `Biom: ${b.name}` }))} />
      <SegmentedControl.Root value={licht} onValueChange={(v) => onLicht(v === 'nacht' ? 'nacht' : 'tag')}>
        <SegmentedControl.Item value="tag">Tag</SegmentedControl.Item>
        <SegmentedControl.Item value="nacht">Nacht</SegmentedControl.Item>
      </SegmentedControl.Root>
    </>
  );
}
