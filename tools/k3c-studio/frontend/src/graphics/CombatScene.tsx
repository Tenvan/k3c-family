import { Button, Flex, Slider, Text } from '@webkit/atoms';
import { SelectField } from '@webkit/atoms';
import { CheckField } from '@webkit/molecules';
import { useEffect, useMemo, useRef, useState } from 'react';
import { zeichneKampf, KAMPF_H, KAMPF_W } from './combatDraw';
import { ABLAUF_SEKUNDEN, ablauf, BOSSES, EVENTS, kampfBild, MIN_SCHRIFT_PX, MOCK_START, schriftPx, ZELLE_LABEL, ZELLEN, zellenZoom, type KampfEvent, type KampfMock, type Zelle } from './combatLogic';
import { pixelContext } from './pixel';
import type { Licht } from './gameScenesLogic';

// Szene „Kampf-Anzeigen“ (B-602): Boss-Leiste, Phasen-Text, Warnkreis, Event-Banner und Mondanzeige aus Mock-Daten, gezeichnet mit
// den Funktionen des Spiels (`combatDraw.ts`); der Ablauf läuft von „Abspielen“ (Auftritt, Phasenwechsel, Warnkreis, Banner).

const EVENT_LABEL: Record<KampfEvent, string> = { fullMoon: 'Vollmond', bloodMoon: 'Blutmond', merchantRaid: 'Händler-Überfall' };

export function CombatScene({ biom, licht }: { biom: string; licht: Licht }) {
  const [mock, setMock] = useState<KampfMock>(MOCK_START);
  const [t, setT] = useState<number | null>(null);
  const [zelle, setZelle] = useState<Zelle>('voll');
  const zoom = zellenZoom(zelle);
  const ref = useRef<HTMLCanvasElement>(null);
  const spiel = t === null ? { mock, leiste: 1, banner: 1 } : ablauf(mock, t);
  const bild = useMemo(() => kampfBild(spiel.mock), [JSON.stringify(spiel.mock)]); // eslint-disable-line react-hooks/exhaustive-deps
  const set = (p: Partial<KampfMock>) => setMock((m) => ({ ...m, ...p }));
  useEffect(() => {
    const c = ref.current;
    const g = c && pixelContext(c, KAMPF_W, KAMPF_H);
    if (g) zeichneKampf(g, bild, { biom, licht, spieler: mock.spieler, leiste: spiel.leiste, banner: spiel.banner, zoom });
  }, [bild, biom, licht, mock.spieler, spiel.leiste, spiel.banner, zoom]);
  useEffect(() => {
    if (t === null) return;
    const t0 = performance.now() - t * 1000;
    let raf = 0;
    const step = (now: number) => {
      const s = (now - t0) / 1000;
      if (s >= ABLAUF_SEKUNDEN) return setT(null);
      setT(s);
      raf = requestAnimationFrame(step);
    };
    raf = requestAnimationFrame(step);
    return () => cancelAnimationFrame(raf);
    // der Ablauf startet mit t = 0 und läuft von selbst; maßgeblich ist nur, ob er läuft
  }, [t === null]); // eslint-disable-line react-hooks/exhaustive-deps
  return (
    <>
      <Flex gap="3" wrap="wrap" align="center">
        <SelectField value={mock.boss} aria-label="Boss" onChange={(boss) => set({ boss })} options={BOSSES.map((b) => ({ value: b.id, label: `${b.id} (${b.kind === 'end' ? 'Endboss' : 'Miniboss'})` }))} />
        <SelectField value={String(mock.spieler)} aria-label="Spieler" onChange={(v) => set({ spieler: v === '2' ? 2 : 1 })} options={[{ value: '1', label: '1 Spieler' }, { value: '2', label: '2 Spieler' }]} />
        <SelectField value={zelle} aria-label="Zellengröße" onChange={(v) => setZelle(ZELLEN.find((z) => z === v) ?? 'voll')} options={ZELLEN.map((z) => ({ value: z, label: `${ZELLE_LABEL[z]} · Welt ×${zellenZoom(z).toFixed(2)}` }))} />
        <Text size="2">Leben {Math.round(spiel.mock.hpPercent)} %</Text>
        <Slider aria-label="Lebensanteil" style={{ width: 160 }} min={0} max={100} value={[mock.hpPercent]} disabled={t !== null} onValueChange={([v]) => set({ hpPercent: v ?? 0 })} />
        <CheckField label="Warnkreis" checked={mock.warn !== null} onCheckedChange={(on) => set({ warn: on === true ? { radius: 5, seconds: 1.5 } : null })} />
        {mock.warn && (
          <>
            <Text size="2">Radius {mock.warn.radius} · Rest {spiel.mock.warn?.seconds.toFixed(1)} s</Text>
            <Slider aria-label="Warnkreis-Radius" style={{ width: 100 }} min={1} max={8} value={[mock.warn.radius]} onValueChange={([v]) => set({ warn: { ...mock.warn!, radius: v ?? 1 } })} />
            <Slider aria-label="Warnkreis-Restzeit" style={{ width: 100 }} min={0} max={4} step={0.1} value={[mock.warn.seconds]} disabled={t !== null} onValueChange={([v]) => set({ warn: { ...mock.warn!, seconds: v ?? 0 } })} />
          </>
        )}
        {EVENTS.map((e) => (
          <CheckField key={e} label={EVENT_LABEL[e]} checked={mock.events.includes(e)} onCheckedChange={(on) => set({ events: on === true ? [...mock.events, e] : mock.events.filter((x) => x !== e) })} />
        ))}
        <Button size="1" onClick={() => setT(0)}>Abspielen</Button>
      </Flex>
      <Text size="2" color="gray">Mindest-Schrift {MIN_SCHRIFT_PX} px (Boss-Leiste {schriftPx('bossBar')} px, Banner {schriftPx('banner')} px); rote Linie = oben 70 px frei für den Home-Button. Mondanzeige: Vollmond und Blutmond im Banner.</Text>
      <canvas ref={ref} className="sz-canvas sz-kampf" />
    </>
  );
}
