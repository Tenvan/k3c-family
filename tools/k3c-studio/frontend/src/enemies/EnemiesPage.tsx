import { Button, SelectField } from '@webkit/atoms';
import { useTicketPage } from '@webkit/lib/ticketContext';
import { ChipSwitch } from '@webkit/molecules';
import { useMemo } from 'react';
import '../styles/figures.css';
import type { KampfWorld } from '@game/scenes/kampfAnzeige';
import { FigureStage } from '../playback/FigureStage';
import { GapChip, PlaybackControls } from '../playback/PlaybackControls';
import { useBossDisplay } from './BossDisplay';
import { EnemyInspector } from './EnemyInspector';
import { EnemyTable } from './EnemyTable';
import { ABILITY_MODE, ENEMY_MODES } from './enemyLogic';
import { cardGaps, useEnemies, type Enemies } from './useEnemies';

/** Alle Lücken der Gruppe: je Karte, dazu die der Aktion und der gespielten Fähigkeit */
const groupGaps = (e: Enemies) => [...e.cards.flatMap((c) => cardGaps(c, e.infoOf(c.id)?.gap)), ...(e.action?.gaps ?? []), ...(e.play?.gaps ?? [])];

/** Eine Werkzeugleiste (B-672): Gruppe, Modus als Segment (Animation, Aktion, Fähigkeit …), Chip Monarch, Wiedergabe, Chip Lücken */
function Toolbar({ e }: { e: Enemies }) {
  const modes = e.isBoss ? [...ENEMY_MODES, ABILITY_MODE] : ENEMY_MODES;
  const frames = e.infoOf(e.selected?.id ?? '')?.frames ?? 1;
  return (
    <div className="wd-figbar" role="toolbar" aria-label="Gegner">
      <SelectField size="1" aria-label="Gruppe" value={e.group.id} options={e.all.map((g) => ({ value: g.id, label: g.title }))} onChange={e.setGroupId} />
      <ChipSwitch label="Modus" options={modes} value={e.mode} onChange={e.setMode} />
      {e.mode === 'ability' && e.entries.length > 0 && (
        <SelectField size="1" aria-label="Fähigkeit" value={e.entry?.key ?? ''} options={e.entries.map((x) => ({ value: x.key, label: x.label }))}
          onChange={(k) => { const x = e.entries.find((y) => y.key === k); if (x) e.playAbility(x); }} />
      )}
      <Button size="1" variant={e.withMonarch ? 'solid' : 'soft'} aria-label="Monarch-Vergleich" aria-pressed={e.withMonarch} onClick={() => e.setWithMonarch(!e.withMonarch)}>Monarch</Button>
      <div className="wd-figbar-end">
        <PlaybackControls control={e.control} onChange={e.setControl} frames={frames} />
        <GapChip gaps={groupGaps(e)} />
      </div>
    </div>
  );
}

/** Ohne den Warnkreis des Beispielzustands: spielt eine Fähigkeit, zeichnet sie ihren eigenen (sonst stünden zwei da). */
const withoutWarn = (w: KampfWorld | null): KampfWorld | null => (w ? { ...w, enemies: (w.enemies ?? []).map((x) => ({ ...x, warn: undefined })) } : null);

/** Gegner-Seite des Asset-Studios (B-464, B-672): alle Gegnertypen nach Biom und Tiefe (Portal vor Nacht, dazu „ohne Biom“)
 *  und die Bosse mit den Animationen des Spiels. Eine Werkzeugleiste, die Bühne über die Resthöhe, darunter die Gegner als
 *  Tabelle und der gewählte im Inspektor; Klick auf Zeile oder Figur wählt. Angriffe und Boss-Fähigkeiten spielen die
 *  Darstellung des Spiels oder zeigen ihre Lücke; Grafik-Auswahl je Gegner-ID als Vorschau (das Register bleibt unverändert). */
export function EnemiesPage() {
  const e = useEnemies();
  useTicketPage('gegner', e.group.id);
  // Boss-Anzeigen des Spiels (B-469) nur in der Gruppe der Bosse; die Mitte des Warnkreises ist die Lage des Boss-Bildes
  const boss = useBossDisplay(e.isBoss, (id) => e.items.find((i) => i.id === `enemy-${id}`)?.x ?? 0);
  const kampf = useMemo(() => (e.mode === 'ability' ? withoutWarn(boss.world) : boss.world), [e.mode, boss.world]);
  const onFigure = (id: string | null) => { if (id?.startsWith('enemy-')) e.setSelectedId(id.slice('enemy-'.length)); };
  return (
    <div className="wd-root">
      <div className="wd-page wd-gegner">
        <Toolbar e={e} />
        {boss.bar}
        <div className="wd-gegner-stage">
          <FigureStage items={e.items} control={e.control} kampf={kampf} selected={e.selected ? `enemy-${e.selected.id}` : null} onFigure={onFigure} onInfo={e.setInfos} />
        </div>
        <div className="wd-gegner-lower">
          <EnemyTable e={e} />
          <EnemyInspector e={e} />
        </div>
      </div>
    </div>
  );
}
