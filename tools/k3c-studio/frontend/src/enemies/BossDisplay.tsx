import { SelectField, Text, plainOptions, type SelectOption } from '@webkit/atoms';
import { useEffect, useMemo, useState } from 'react';
import type { KampfWorld } from '@game/scenes/kampfAnzeige';
import { BOSS_DATA, abilityIn, bossWorld, phaseCount, phaseShare, phaseThreshold } from './bossLogic';

/** HP-Anteile zur Wahl (Prozent); die Phase des Endbosses setzt einen passenden Wert, er bleibt änderbar */
const HP_CHOICES = [100, 75, 50, 25, 10];

/** Beispielzustand für die Boss-Anzeigen des Spiels (B-469): Boss, beim Endboss die Phase, HP-Anteil. Daraus entsteht der
 *  Zustand, den `KampfHud` und `WarnView` des Spiels zeichnen; die Seite zeichnet nichts selbst. `x` ist die Lage des Bosses
 *  auf der Bühne (Units, für die Mitte des Warnkreises). Liefert `null`, solange die Anzeige aus ist. */
export function useBossDisplay(active: boolean, xOf: (bossId: string) => number): { world: KampfWorld | null; bar: React.ReactNode } {
  const [bossId, setBossId] = useState(BOSS_DATA[0]!.id);
  const [phase, setPhase] = useState(1);
  const [percent, setPercent] = useState(100);
  const boss = BOSS_DATA.find((b) => b.id === bossId) ?? BOSS_DATA[0]!;
  const phases = phaseCount(boss);
  useEffect(() => { setPhase(1); setPercent(100); }, [bossId]);
  const x = xOf(boss.id);
  const world = useMemo(
    () => (active ? bossWorld(boss, { ...(phases > 0 ? { phase } : {}), share: percent / 100, x }) : null),
    [active, boss, phases, phase, percent, x],
  );
  const threshold = phases > 0 ? phaseThreshold(boss, phase) : undefined;
  const ability = abilityIn(boss, phases > 0 ? phase : undefined);
  const choices = useMemo(() => HP_CHOICES.map((p): SelectOption<string> => ({ value: String(p), label: `${p} % HP` })), []);
  const bar = !active ? null : (
    <div className="wd-bar" role="group" aria-label="Boss-Anzeige">
      <SelectField aria-label="Boss-Anzeige" value={boss.id} options={plainOptions(BOSS_DATA.map((b) => b.id))} onChange={setBossId} />
      {phases > 0 && (
        <SelectField aria-label="Phase" value={String(phase)} onChange={(v) => { setPhase(Number(v)); setPercent(Math.round(phaseShare(boss, Number(v)) * 100)); }}
          options={Array.from({ length: phases }, (_, i): SelectOption<string> => ({ value: String(i + 1), label: `Phase ${i + 1}` }))} />
      )}
      <SelectField aria-label="HP" value={String(percent)} options={choices.some((c) => c.value === String(percent)) ? choices : [...choices, { value: String(percent), label: `${percent} % HP` }]}
        onChange={(v) => setPercent(Number(v))} />
      <Text size="1" color="gray" aria-label="Phasen-Schwelle">
        {threshold === undefined ? 'Schwelle: ab Beginn' : `Schwelle: unter ${threshold} % HP`}{ability ? ` · ${ability.type}${ability.radius ? `, Radius ${ability.radius}` : ''}` : ''}
      </Text>
    </div>
  );
  return { world, bar };
}
