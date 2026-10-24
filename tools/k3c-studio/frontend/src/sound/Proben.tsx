import { ActionButton, Button, Checkbox, Flex, Heading, SelectField, Text } from '@webkit/atoms';
import { NoticeCard } from '@webkit/molecules';
import { useEffect, useMemo, useRef, useState } from 'react';
import type { AssetSlot } from '../api';
import { useAssets } from '../assets/useAssets';
import { core, playAtGain, playChoice, stopAll } from './audio';
import { combatMix, distanceSteps, duckSummary, LOUDNESS_GAP_MS, loudnessQueue, pickOf, playable, type Pick } from './probes';
import { playStatus } from './playback';
import { GLOBAL_SLOTS } from './soundRows';

// Reiter „Proben“ der Seite Sound (WZ30.2, B-597/AC-03): Mischprobe (Musik eines Zustands und ein Effekt zugleich, Ducking
// hörbar), Lautheitsvergleich (gewählte Töne nacheinander mit gleichem Pegel) und Abstandsprobe (nah und fern wie im Spiel).

const pause = (ms: number) => new Promise((r) => setTimeout(r, ms));

/** Zustand und Aktionen der Proben; `run` zählt jeden Stopp hoch, damit eine laufende Folge endet statt später doch zu spielen. */
function useProben(slots: readonly AssetSlot[]) {
  const music = useMemo(() => playable(slots, 'music'), [slots]);
  const fx = useMemo(() => playable(slots, 'sfx'), [slots]);
  const [musicId, setMusicId] = useState('');
  const [fxId, setFxId] = useState('');
  const [chosen, setChosen] = useState<string[]>([]);
  const [status, setStatus] = useState('');
  const run = useRef(0);
  useEffect(() => () => { run.current += 1; }, []);
  const m = pickOf(slots, musicId) ?? music[0];
  const e = pickOf(slots, fxId) ?? pickOf(slots, 'sfx:warn') ?? fx[0];
  const sound = (label: string, ok: boolean) => setStatus(ok ? label : `${label}: Ton nicht abspielbar (Datei fehlt oder Atlas nicht gebaut: task soundatlas)`);
  const startMusic = async () => m && sound(`Musik ${m.slot.id}`, await playChoice(m.slot, m.choice));
  const playFx = async () => {
    if (!e) return;
    if (GLOBAL_SLOTS.has(e.slot.id)) core().duck(); // wie im Spiel: eine Warnung senkt die Musik
    sound(`Effekt ${e.slot.id}`, await playChoice(e.slot, e.choice));
  };
  const combat = async () => {
    const mix = combatMix(slots);
    const ok = await Promise.all(mix.map((p) => playChoice(p.slot, p.choice)));
    setStatus(mix.length ? `Kampf-Mischung: ${mix.map((p) => p.slot.id).join(' + ')}${ok.every(Boolean) ? '' : ' (nicht alle abspielbar)'}` : 'Kampf-Mischung: keine der Slots hit, arrow, kill zugeordnet');
  };
  const toggle = (id: string) => setChosen((c) => (c.includes(id) ? c.filter((x) => x !== id) : [...c, id]));
  const loudness = async () => {
    const queue = loudnessQueue(slots, chosen);
    const mine = ++run.current;
    for (const [i, p] of queue.entries()) {
      if (run.current !== mine) return;
      setStatus(`Lautheit ${i + 1}/${queue.length}: ${playStatus(p.slot.beschreibung, p.choice, await playAtGain(p.choice, 1))}`);
      await pause(LOUDNESS_GAP_MS);
    }
  };
  const stop = () => { run.current += 1; stopAll(); setStatus(''); };
  return { music, fx, m, e, chosen, status, setMusicId, setFxId, sound, startMusic, playFx, combat, toggle, loudness, stop };
}
type State = ReturnType<typeof useProben>;

const options = (list: Pick[]) => list.map((p) => ({ value: p.slot.id, label: `${p.slot.id} · ${p.slot.beschreibung}` }));

export function Proben() {
  const data = useAssets();
  const p = useProben(data.view?.slots ?? []);
  if (!data.view) return data.error ? <NoticeCard title="Slot-Register nicht lesbar" tone="error">{data.error}</NoticeCard> : <Text color="gray">Lade Slot-Register …</Text>;
  return (
    <Flex direction="column" gap="3" className="sz-page">
      <div className="as-head">
        <Button variant="soft" color="gray" onClick={p.stop}>Stopp</Button>
        <Text size="2" color="gray" role="status">{p.status || 'Klick auf eine Probe entsperrt den Ton (Nutzergeste).'}</Text>
      </div>
      <MixSection p={p} />
      <LoudnessSection p={p} />
      <DistanceSection p={p} />
    </Flex>
  );
}

/** Mischprobe: Musik eines Zustands und ein Effekt zugleich, Ducking hörbar. */
function MixSection({ p }: { p: State }) {
  return (
    <section>
      <Heading size="3">Mischprobe</Heading>
      <Text size="1" color="gray">{duckSummary()} Der Effekt senkt die Musik nur, wenn er im Spiel eine Warnung ist (global).</Text>
      <Flex gap="2" align="center" wrap="wrap">
        {p.music.length === 0 ? <Text color="gray">Keine Musik mit Zuordnung im Register.</Text> : (
          <SelectField aria-label="Musik" value={p.m?.slot.id ?? ''} options={options(p.music)} onChange={p.setMusicId} />
        )}
        <ActionButton size="1" onClick={p.startMusic}>Musik starten</ActionButton>
        {p.fx.length === 0 ? <Text color="gray">Kein Effekt mit Zuordnung im Register.</Text> : (
          <SelectField aria-label="Effekt" value={p.e?.slot.id ?? ''} options={options(p.fx)} onChange={p.setFxId} />
        )}
        <ActionButton size="1" onClick={p.playFx}>Effekt dazu</ActionButton>
        <ActionButton size="1" variant="soft" onClick={p.combat}>Kampf-Mischung</ActionButton>
      </Flex>
    </section>
  );
}

/** Lautheitsvergleich: gewählte Töne nacheinander mit gleichem Pegel. */
function LoudnessSection({ p }: { p: State }) {
  return (
    <section>
      <Heading size="3">Lautheitsvergleich</Heading>
      <Text size="1" color="gray">Gewählte Töne nacheinander, alle mit Pegel 1 auf dem Bus Effekte, {LOUDNESS_GAP_MS / 1000} s Pause.</Text>
      <Flex gap="3" wrap="wrap">
        {p.fx.map((x) => (
          <Text key={x.slot.id} as="label" size="2">
            <Checkbox checked={p.chosen.includes(x.slot.id)} onCheckedChange={() => p.toggle(x.slot.id)} aria-label={x.slot.id} /> {x.slot.id}
          </Text>
        ))}
      </Flex>
      <ActionButton size="1" disabled={p.chosen.length === 0} onClick={p.loudness}>Nacheinander abspielen ({p.chosen.length})</ActionButton>
    </section>
  );
}

/** Abstandsprobe: der gewählte Effekt nah und fern, Dämpfung wie im Spiel. */
function DistanceSection({ p }: { p: State }) {
  const steps = distanceSteps();
  const e = p.e;
  return (
    <section>
      <Heading size="3">Abstandsprobe</Heading>
      <Text size="1" color="gray">Der gewählte Effekt (oben) in den Stufen des Spiels: Faktor aus `attenuation`, Bildausschnitt Mitte 20, Breite 20.</Text>
      <Flex gap="2" wrap="wrap">
        {steps.map((s) => (
          <ActionButton key={s.label} size="1" variant="soft" disabled={!e || s.gain <= 0} onClick={() => e && playAtGain(e.choice, s.gain).then((ok) => p.sound(`${e.slot.id} ${s.label}`, ok))}>
            {s.label} · {Math.round(s.gain * 100)} %
          </ActionButton>
        ))}
      </Flex>
      {steps.some((s) => s.gain <= 0) && <Text size="1" color="gray">Eine Bildschirmbreite daneben ist der Ton im Spiel stumm; die Stufe bleibt zum Vergleich gesperrt.</Text>}
    </section>
  );
}
