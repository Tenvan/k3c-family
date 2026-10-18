import { Button, Text } from '@radix-ui/themes';
import { CROSSFADE_SECONDS } from '@game/audio/crossfade';
import { candidatesFromSlots, fileFor, groupCandidates, keyAction, moveSelection, stepVolume, type Action, type Candidate, type Group } from '@game/tools/soundtestLogic';
import { useEffect, useMemo, useRef, useState, type KeyboardEvent } from 'react';
import { useAssets } from '../assets/useAssets';
import { NoticeCard } from '../ui/parts';
import { core, stopAll } from './audio';

// Reiter „Hörprobe“ der Seite Sound (B-459/AC-06, Inhalt von soundtest.html ohne Pad-Steuerung): ganze Dateien der
// Sound-Slots nach Musik-Zuständen und Ereignissen, hart oder mit Crossfade, Stopp, Lautstärke ±10 %, Tastatur wie
// dort (Pfeile, Enter/Leertaste, S, X, +/−). Die Logik kommt aus src/tools/soundtestLogic.ts (soundtest.html nutzt sie weiter).

const canPlay = (mime: string) => new Audio().canPlayType(mime) !== '';

/** Zustand und Aktionen der Hörprobe wie in soundtest.ts: eine Stimme zur Zeit, Musik in Schleife, Effekte einmal.
 *  `playing` steht zusätzlich in einem Ref, damit nach den await der aktuelle Stand gilt, nicht der des alten Renderns. */
function useHoerprobe(list: readonly Candidate[]) {
  const [selected, setSelected] = useState(0);
  const [playing, setPlayingState] = useState<Candidate>();
  const now = useRef<Candidate | undefined>(undefined);
  const [failed, setFailed] = useState<ReadonlySet<string>>(new Set());
  const [volume, setVolume] = useState(() => core().getVolume('music'));
  const setPlaying = (c: Candidate | undefined) => {
    now.current = c;
    setPlayingState(c);
  };
  const play = async (index: number, crossfade: boolean) => {
    const c = list[index];
    const url = c && fileFor(c, canPlay);
    const k = core();
    await k.onInput();
    if (!c || !k.running || now.current === c) return;
    if (!url || !(await k.preload(url))) return setFailed((f) => new Set(f).add(c.datei));
    const prev = now.current;
    if (prev && prev.bus !== c.bus) k.stop(prev.bus);
    const ok = await k.playFile(url, { bus: c.bus, fade: crossfade && prev ? CROSSFADE_SECONDS : 0, onEnded: () => { if (now.current === c) setPlaying(undefined); } });
    if (ok) setPlaying(c);
  };
  /** Wie soundtest.ts: erst entsperren, dann nur mit laufendem Kern; die Lautstärke kommt aus dem Mixer (gemerkt). */
  const act = async (a: Action) => {
    const k = core();
    await k.onInput();
    if (!k.running) return;
    if (a === 'up' || a === 'down') return setSelected((i) => moveSelection(i, list.length, a === 'up' ? -1 : 1));
    if (a === 'play' || a === 'fade') return play(selected, a === 'fade');
    if (a === 'stop') {
      stopAll(); // stoppt auch, was in Szenen oder am Slot gestartet wurde
      return setPlaying(undefined);
    }
    k.setVolume('music', stepVolume(k.getVolume('music'), a === 'volUp' ? 1 : -1));
    k.setVolume('sfx', k.getVolume('music'));
    setVolume(k.getVolume('music'));
  };
  return { selected, setSelected, playing, failed, volume, play, act };
}

/** Tastatur wie soundtest.html; Enter und Leertaste auf einem Knopf bleiben dessen eigener Klick. */
function onKey(e: KeyboardEvent, act: (a: Action) => Promise<unknown>) {
  const a = keyAction(e.code);
  if (!a || (a === 'play' && e.target instanceof HTMLButtonElement)) return;
  e.preventDefault();
  void act(a);
}

export function Hoerprobe() {
  const data = useAssets();
  const groups = useMemo(() => groupCandidates(candidatesFromSlots(data.view?.slots ?? [])), [data.view]);
  const list = useMemo(() => groups.flatMap((g) => g.items), [groups]);
  const h = useHoerprobe(list);
  const box = useRef<HTMLDivElement>(null);
  // Block statt Ausdruck: ein Rückgabewert von scrollIntoView würde React als Aufräumfunktion nehmen („destroy is not a function“).
  useEffect(() => { box.current?.querySelector('.hp-sel')?.scrollIntoView?.({ block: 'nearest' }); }, [h.selected]);
  if (!data.view) return data.error ? <NoticeCard title="Slot-Register nicht lesbar" tone="error">{data.error}</NoticeCard> : <Text color="gray">Lade Slot-Register …</Text>;
  return (
    <div ref={box} className="hp-page" tabIndex={0} onKeyDown={(e) => onKey(e, h.act)}>
      <div className="as-head">
        <Button variant="soft" color="gray" onClick={() => void h.act('stop')}>Stopp (S)</Button>
        <Button variant="soft" onClick={() => void h.act('fade')}>Überblenden (X)</Button>
        <Button variant="soft" onClick={() => void h.act('volDown')}>Leiser (−)</Button>
        <Button variant="soft" onClick={() => void h.act('volUp')}>Lauter (+)</Button>
        <Text size="2" color="gray" role="status">{h.playing ? `spielt: ${h.playing.name}` : 'Stille'} · Lautstärke {h.volume} %</Text>
      </div>
      <Text size="1" color="gray">Pfeile wählen, Enter oder Leertaste spielt, X überblendet; der erste Klick entsperrt den Ton.</Text>
      {list.length === 0 && <Text color="gray">Keine ganzen Sound-Dateien mit Bus music oder sfx im Slot-Register.</Text>}
      <Groups groups={groups} h={h} />
    </div>
  );
}

function Groups({ groups, h }: { groups: readonly Group[]; h: ReturnType<typeof useHoerprobe> }) {
  let i = 0;
  return groups.map((g) => (
    <section key={g.title}>
      <h3 className="sz-title">{g.title} <small className="sz-dim">{g.kind}</small></h3>
      {g.items.map((c) => {
        const idx = i++;
        const off = h.failed.has(c.datei);
        return (
          <button key={`${c.datei}:${idx}`} type="button" className={`hp-item${idx === h.selected ? ' hp-sel' : ''}${h.playing === c ? ' hp-on' : ''}${off ? ' hp-off' : ''}`}
            onFocus={() => h.setSelected(idx)} onClick={() => { h.setSelected(idx); void h.play(idx, false); }}>
            <span>{off ? `${c.name} – lädt nicht` : c.name}</span>
            <span className="sz-dim">{c.quelle} · {c.lizenz}</span>
          </button>
        );
      })}
    </section>
  ));
}
