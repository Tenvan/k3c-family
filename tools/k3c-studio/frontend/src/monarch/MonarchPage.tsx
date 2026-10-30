import { SelectField, Text, type SelectOption } from '@webkit/atoms';
import { useTicketPage } from '@webkit/lib/ticketContext';
import { useEffect, useRef, useState } from 'react';
import '../styles/figures.css';
import { FigureStage } from '../playback/FigureStage';
import { GapBadge } from '../playback/PlaybackControls';
import type { FigureInfo } from '../stage/PreviewStage';
import { MonarchToolbar } from './MonarchToolbar';
import { PLAYERS, type SlotOption } from './monarchLogic';
import { wheelZoom } from './monarchZoom';
import { SkillPanel } from './SkillPanel';
import { useMonarch, useSkillPanel, type Monarch } from './useMonarch';

/** Einträge der Grafik-Auswahl aus den Slots: Wert ist der Index der Präferenz. */
const sheetOptions = (options: SlotOption[]): SelectOption<string>[] =>
  options.map((o) => ({ value: String(o.index), label: `Präferenz ${o.index + 1}: ${o.sheet}` }));

/** Spieler als schmale Zeile unter der Bühne (B-671): Kennfarbe, Kurzname der Grafik, Präferenz als kleines Dropdown; dazu
 *  die Grafik des Reittiers. Ohne Slots keine Auswahl. */
function Players({ m }: { m: Monarch }) {
  return (
    <ul className="wd-colors wd-players" aria-label="Spielerfarben">
      {PLAYERS.map((p) => (
        <li key={p.index} data-index={p.index}>
          <span className="wd-badge" style={{ background: p.badge }} title={`Kennfarbe ${p.badge}`} />
          <Text size="1" title={p.spec.tint ? `tint ${p.spec.tint}` : undefined}>Spieler {p.index + 1}: {m.riders[p.index]?.sheet ?? p.spec.sheet}</Text>
          {m.riderOptions[p.index]!.length > 0 && (
            <SelectField size="1" aria-label={`Grafik Monarch ${p.index + 1}`} value={String(m.riders[p.index]?.index ?? 0)} options={sheetOptions(m.riderOptions[p.index]!)}
              onChange={(v) => m.pickRider(p.index, Number(v))} />
          )}
          {m.riders[p.index]?.gap && <GapBadge gap={m.riders[p.index]!.gap} />}
        </li>
      ))}
      {m.mountOptions.length > 0 && (
        <li data-mount={m.mount}>
          <Text size="1">Reittier {m.mount}</Text>
          <SelectField size="1" aria-label="Grafik Reittier" value={String(m.mountChoice?.index ?? 0)} options={sheetOptions(m.mountOptions)} onChange={(v) => m.pickMount(Number(v))} />
        </li>
      )}
    </ul>
  );
}

/** Zoom der Ansicht mit Alt + Mausrad: nativer Listener, React hängt Wheel-Handler passiv an, preventDefault (die Seite soll
 *  bei Alt + Rad nicht scrollen) braucht passive: false */
function useWheelZoom(box: React.RefObject<HTMLDivElement | null>, setZoom: (f: (z: number) => number) => void): void {
  useEffect(() => {
    const el = box.current;
    if (!el) return;
    const onWheel = (e: WheelEvent) => {
      if (!wheelZoom(1, e)) return;
      e.preventDefault();
      setZoom((z) => wheelZoom(z, e) ?? z);
    };
    el.addEventListener('wheel', onWheel, { passive: false });
    return () => el.removeEventListener('wheel', onWheel);
  }, [box, setZoom]);
}

/** Monarch-Seite des Asset-Studios (B-463, B-671): der Monarch immer beritten, mit den Animationen des Spiels. Eine
 *  Werkzeugleiste, darunter die Bühne über die Resthöhe mit den vier Spielerfarben im selben Takt, die Spieler als schmale
 *  Zeile; die Skills liegen in einem einklappbaren Seitenpanel und lassen sich wie im Spiel abspielen. Grafik-Auswahl aus
 *  den Slots `monarch:*` und `mount:*` als Vorschau (das Register bleibt unverändert). */
export function MonarchPage() {
  const m = useMonarch();
  useTicketPage('monarch', m.mount);
  const [infos, setInfos] = useState<FigureInfo[]>([]);
  const [missing, setMissing] = useState<string[]>([]);
  const [zoom, setZoom] = useState(1);
  const [panelOpen, setPanelOpen] = useSkillPanel();
  const stageBox = useRef<HTMLDivElement>(null);
  useWheelZoom(stageBox, setZoom);
  return (
    <div className="wd-root">
      <div className="wd-page wd-monarch-page">
        <MonarchToolbar m={m} info={infos[0]} zoom={zoom} onZoom={setZoom} panelOpen={panelOpen} onPanel={setPanelOpen} missing={missing} />
        <div className="wd-monarch" data-panel={panelOpen ? 'open' : 'closed'}>
          <div className="wd-monarch-main">
            <div className="wd-monarch-stage" ref={stageBox}>
              <FigureStage items={m.items} control={m.control} zoom={zoom} onInfo={setInfos} onLoadError={setMissing} />
            </div>
            <Players m={m} />
          </div>
          {panelOpen && <SkillPanel current={m.mode === 'skill' ? m.skillId : null} onPlay={m.playSkill} onClose={() => setPanelOpen(false)} />}
        </div>
      </div>
    </div>
  );
}
