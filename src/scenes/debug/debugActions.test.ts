import { describe, expect, it } from 'vitest';
import devPause from '../../../testdata/protocol/c2s-dev-pause.json';
import devGrade from '../../../testdata/protocol/c2s-dev-grade.json';
import devBoss from '../../../testdata/protocol/c2s-dev-boss.json';
import devBossPhase from '../../../testdata/protocol/c2s-dev-bossphase.json';
import devEvent from '../../../testdata/protocol/c2s-dev-event.json';
import devNight from '../../../testdata/protocol/c2s-dev-night.json';
import eventsJson from '../../../data/events.json';
import { BOSSES, DEV_ACTIONS, DEV_GROUPS, EVENTS, GRADES, NO_TOGGLES, actionLabel, actionVisible, devMessage, isCurrentGrade, nextToggles, pauseMessage, roomDevMode } from './debugActions';

describe('devMessage (B-179/AC-01)', () => {
  it('Beispiele aus testdata/protocol/: Gold und Zeitfaktor wörtlich', () => {
    // c2s-dev-gold.json
    expect(devMessage('gold:50', 0)).toEqual({ t: 'dev', action: 'gold', slot: 0, amount: 50 });
    // c2s-dev-timescale.json
    expect(devMessage('timescale:4', 0)).toEqual({ t: 'dev', action: 'timescale', factor: 4 });
    // c2s-dev-material.json mit der Menge der Liste (50 statt 100)
    expect(devMessage('material:wood', 0)).toEqual({ t: 'dev', action: 'material', slot: 0, resource: 'wood', amount: 50 });
  });

  it('jede Aktion der Liste ergibt die erwartete Nachricht', () => {
    const expected = [
      { t: 'dev', action: 'gold', slot: 1, amount: 10 },
      { t: 'dev', action: 'gold', slot: 1, amount: 50 },
      { t: 'dev', action: 'gold', slot: 1, amount: 100 },
      { t: 'dev', action: 'material', slot: 1, resource: 'wood', amount: 50 },
      { t: 'dev', action: 'material', slot: 1, resource: 'stone', amount: 50 },
      { t: 'dev', action: 'material', slot: 1, resource: 'copper', amount: 50 },
      { t: 'dev', action: 'material', slot: 1, resource: 'iron', amount: 50 },
      { t: 'dev', action: 'material', slot: 1, resource: 'crystal', amount: 50 },
      { t: 'dev', action: 'maxStock', amount: 100 },
      { t: 'dev', action: 'fillStock' },
      { t: 'dev', action: 'cheapBuild', on: true },
      { t: 'dev', action: 'buildAll', level: 1 },
      { t: 'dev', action: 'buildAll', level: 2 },
      { t: 'dev', action: 'buildAll', level: 3 },
      { t: 'dev', action: 'buildAll', level: 4 },
      { t: 'dev', action: 'buildAll', level: 5 },
      { t: 'dev', action: 'castleFall', slot: 1 },
      { t: 'dev', action: 'timescale', factor: 1 },
      { t: 'dev', action: 'timescale', factor: 2 },
      { t: 'dev', action: 'timescale', factor: 4 },
      { t: 'dev', action: 'timescale', factor: 8 },
      { t: 'dev', action: 'timescaleAuto', on: true },
      { t: 'dev', action: 'nextPhase' },
      ...['dev', 'easy', 'normal', 'hard', 'ultra'].map((grade) => ({ t: 'dev', action: 'grade', grade })),
      ...['goblinLeader', 'trollKing', 'ratQueen', 'lavaGolem', 'shardTitan', 'crystalHeartWarden'].map((boss) => ({ t: 'dev', action: 'boss', slot: 1, boss })),
      { t: 'dev', action: 'bossPhase' },
      ...['fullMoon', 'bloodMoon', 'merchantRaid'].map((event) => ({ t: 'dev', action: 'event', event })),
      { t: 'dev', action: 'night' },
    ];
    expect(DEV_ACTIONS.map((a) => devMessage(a.key, 1))).toEqual(expected);
  });

  it('Beschriftungen wie im Ticket', () => {
    expect(DEV_ACTIONS.map((a) => a.label)).toEqual([
      'Gold 10', 'Gold 50', 'Gold 100',
      'Holz 50', 'Stein 50', 'Kupfer 50', 'Eisen 50', 'Kristall 50',
      'Max +100', 'Alle Ressourcen füllen', 'Baukosten 1',
      'Alles ausbauen: Stufe 1', 'Alles ausbauen: Stufe 2', 'Alles ausbauen: Stufe 3', 'Alles ausbauen: Stufe 4', 'Alles ausbauen: Stufe 5', 'Burgfall',
      'Zeit 1×', 'Zeit 2×', 'Zeit 4×', 'Zeit 8×',
      'Zeitraffer → 1× bei Tageszeitwechsel', 'Nächste Tageszeit',
      'Grad Dev', 'Grad Leicht', 'Grad Normal', 'Grad Hart', 'Grad Ultra',
      'Boss Goblin-Anführer', 'Boss Troll-König', 'Boss Ratten-Königin', 'Boss Lava-Golem', 'Boss Splitter-Titan', 'Boss Kristallherz-Wächter',
      'Boss: nächste Phase', 'Event Vollmond', 'Event Blutmond', 'Event Händler-Überfall', 'Nacht sofort',
    ]);
  });

  it('unbekannte Auswahl, Menge oder Faktor → null', () => {
    expect(devMessage('gold:7', 0)).toBeNull();
    expect(devMessage('timescale:3', 0)).toBeNull();
    expect(devMessage('material:gold', 0)).toBeNull();
    expect(devMessage('restart', 0)).toBeNull();
    expect(devMessage('', 0)).toBeNull();
    expect(devMessage('upgrade:6', 0)).toBeNull();
  });

  it('Umschalter senden den umgekehrten Zustand und merken ihn', () => {
    const on = nextToggles('buildcost1', NO_TOGGLES);
    expect(on).toEqual({ cheapBuild: true, timescaleAuto: false });
    expect(devMessage('buildcost1', 0, on)).toEqual({ t: 'dev', action: 'cheapBuild', on: false });
    expect(nextToggles('autoslow', on)).toEqual({ cheapBuild: true, timescaleAuto: true });
    expect(nextToggles('maxplus', on)).toBe(on);
    const toggle = DEV_ACTIONS.find((a) => a.key === 'autoslow')!;
    expect(actionLabel(toggle, on)).toBe('Zeitraffer → 1× bei Tageszeitwechsel: aus');
    expect(actionLabel(toggle, { ...on, timescaleAuto: true })).toBe('Zeitraffer → 1× bei Tageszeitwechsel: an');
  });

  it('Liste steht in der Reihenfolge der Gruppen (Auf/Ab folgt der Anzeige)', () => {
    const order = DEV_ACTIONS.map((a) => DEV_GROUPS.indexOf(a.group));
    expect(order).toEqual([...order].sort((x, y) => x - y));
    expect(order).not.toContain(-1);
  });
});

describe('roomDevMode und pauseMessage (B-231)', () => {
  it('pause hält an und lässt weiterlaufen (c2s-dev-pause.json)', () => {
    expect(pauseMessage(true)).toEqual(devPause);
    expect(pauseMessage(false)).toEqual({ t: 'dev', action: 'pause', paused: false });
  });

  it('Dev-Mode am Feld devTimescale (s2c-snapshot-delta-timescale.json), auch bei Faktor 1', () => {
    expect(roomDevMode({ devTimescale: 4 })).toBe(true);
    expect(roomDevMode({ devTimescale: 1 })).toBe(true);
    expect(roomDevMode({})).toBe(false);
    expect(roomDevMode(null)).toBe(false);
  });
});

describe('Gruppe Grad (K5.3, B-107/AC-01, AC-02)', () => {
  it('jede Grad-Zeile sendet die Dev-Aktion grade wie c2s-dev-grade.json; unbekannter Grad → null', () => {
    expect(devMessage('grade:hard', 0)).toEqual(devGrade);
    expect(GRADES.map(([g]) => devMessage(`grade:${g}`, 0))).toEqual(GRADES.map(([g]) => ({ t: 'dev', action: 'grade', grade: g })));
    expect(devMessage('grade:gibtsnicht', 0)).toBeNull();
  });

  it('nur im Dev-Mode des Raums sichtbar, der aktuelle Grad ist markiert', () => {
    const grades = DEV_ACTIONS.filter((a) => a.group === 'grad');
    expect(grades.some((a) => actionVisible(a, false))).toBe(false);
    expect(grades.every((a) => actionVisible(a, true))).toBe(true);
    expect(DEV_ACTIONS.filter((a) => a.group !== 'grad').every((a) => actionVisible(a, false))).toBe(true);
    expect(grades.filter((a) => isCurrentGrade(a, 'normal')).map((a) => a.key)).toEqual(['grade:normal']);
    expect(grades.some((a) => isCurrentGrade(a, undefined))).toBe(false);
  });
});

describe('Gruppe Kampf (KB4.2, B-603/AC-03)', () => {
  it('Boss, Phase, Event und Nacht senden die Nachrichten wie c2s-dev-boss/-bossphase/-event/-night.json', () => {
    expect(devMessage('boss:goblinLeader', 0)).toEqual(devBoss);
    expect(devMessage('bossphase', 0)).toEqual(devBossPhase);
    expect(devMessage('event:bloodMoon', 0)).toEqual(devEvent);
    expect(devMessage('night', 0)).toEqual(devNight);
    expect(devMessage('boss:goblinLeader', 1)).toEqual({ ...devBoss, slot: 1 });
  });

  it('Event-Auswahl aus den bekannten Ids (data/events.json), unbekannte Boss- oder Event-Id → null', () => {
    expect(EVENTS.map(([id]) => id)).toEqual(eventsJson.events.map((e) => e.id));
    expect(EVENTS.map(([, name]) => name)).toEqual(['Vollmond', 'Blutmond', 'Händler-Überfall']);
    expect(BOSSES.length).toBeGreaterThan(0);
    expect(devMessage('event:neumond', 0)).toBeNull();
    expect(devMessage('boss:drache', 0)).toBeNull();
  });

  it('eigene Gruppe am Ende, ohne Dev-Mode sichtbar (der Server lehnt ab)', () => {
    expect(DEV_GROUPS.at(-1)).toBe('kampf');
    const kampf = DEV_ACTIONS.filter((a) => a.group === 'kampf');
    expect(kampf).toHaveLength(BOSSES.length + EVENTS.length + 2);
    expect(kampf.every((a) => actionVisible(a, false))).toBe(true);
  });
});
