import { ActionButton, Button, SegmentedControl, Text } from '@webkit/atoms';
import { CREDITS } from '@game/tools/credits';
import { useState } from 'react';
import { useAssets } from '../assets/useAssets';
import { licenseFlag, resolveSlot, SCENES, slotsOf, type Query, type Resolved, type SceneId } from '../graphics/scenesLogic';
import { loadPref, savePref } from '../lib/prefs';
import { NoticeCard } from '@webkit/molecules';
import { playChoice, stopAll } from './audio';
import { playStatus } from './playback';

// Reiter „Szenen“ der Seite Sound (B-459/AC-06, Sound-Teil der früheren Asset-Vorschau): Ereignis-SFX, Musik (Wechsel blendet
// mit CROSSFADE_SECONDS über, z. B. music:tag → music:nacht) und Umgebungsgeräusche (Schleife), Präferenz 1 oder
// zum Vergleich Präferenz n eines Slots; Stopp und Statuszeile.

const SOUND = SCENES.filter((s) => s.art === 'sound').map((s) => s.id);
const TITLE: Partial<Record<SceneId, string>> = { sfx: 'Ereignis-SFX', musik: 'Musik', ambient: 'Umgebung' };

export function SoundScenes() {
  const data = useAssets();
  const [szene, setSzene] = useState<SceneId>(() => loadPref('sound.szene', SOUND, 'sfx'));
  const [q, setQ] = useState<Query>({ wahl: 1 });
  const [status, setStatus] = useState('');
  if (!data.view) return data.error ? <NoticeCard title="Slot-Register nicht lesbar" tone="error">{data.error}</NoticeCard> : <Text color="gray">Lade Slot-Register …</Text>;
  const items = slotsOf(data.view.slots, szene).map((s) => resolveSlot(s, q));
  const pick = (v: string) => {
    const next = SOUND.find((s) => s === v) ?? 'sfx';
    setSzene(next);
    setQ({ wahl: 1 });
    savePref('sound.szene', next);
  };
  const play = async (r: Resolved, far: boolean) => {
    if (r.choice) setStatus(playStatus(r.slot.beschreibung, r.choice, await playChoice(r.slot, r.choice, far)));
  };
  return (
    <div className="sz-page">
      <div className="as-head">
        <SegmentedControl.Root value={szene} onValueChange={pick}>
          {SOUND.map((s) => <SegmentedControl.Item key={s} value={s}>{TITLE[s]}</SegmentedControl.Item>)}
        </SegmentedControl.Root>
        <Button variant="soft" color="gray" onClick={() => { stopAll(); setStatus(''); }}>Stopp</Button>
      </div>
      <Text size="2" color="gray" role="status">{status || 'Klick auf „Abspielen“ entsperrt den Ton (Nutzergeste).'}</Text>
      {items.length === 0 && <Text color="gray">Keine Asset-Slots in dieser Kategorie.</Text>}
      {items.map((r) => <SoundRow key={r.slot.id} r={r} q={q} onQ={setQ} onPlay={play} />)}
    </div>
  );
}

function SoundRow({ r, q, onQ, onPlay }: { r: Resolved; q: Query; onQ: (q: Query) => void; onPlay: (r: Resolved, far: boolean) => Promise<void> }) {
  const nc = r.choice && licenseFlag(r.choice.datei, CREDITS);
  const prefs = r.slot.wahl.length;
  return (
    <div className="sd-row">
      <span className="sd-cap">
        <b>{r.slot.id} · {r.slot.beschreibung}</b> {nc && <span className="sz-nc">NC: nur Lückenfüller</span>}
        <br />
        <span className="sz-dim">{r.choice ? `${r.choice.datei}${r.choice.sprite ? `#${r.choice.sprite}` : ''}${r.pref > 1 ? ` · Präferenz ${r.pref}` : ''}` : `Lücke: ${r.slot.id} hat keine Zuordnung`}</span>
      </span>
      {r.choice && <ActionButton size="1" onClick={() => onPlay(r, false)}>Abspielen</ActionButton>}
      {r.choice?.sprite && <ActionButton size="1" variant="soft" onClick={() => onPlay(r, true)}>fern</ActionButton>}
      {prefs > 1 && (
        <SegmentedControl.Root size="1" value={String(q.slot === r.slot.id ? q.wahl : 1)} onValueChange={(v) => onQ({ slot: r.slot.id, wahl: Number(v) })}>
          {r.slot.wahl.map((_, i) => <SegmentedControl.Item key={i} value={String(i + 1)}>Präf. {i + 1}</SegmentedControl.Item>)}
        </SegmentedControl.Root>
      )}
    </div>
  );
}
