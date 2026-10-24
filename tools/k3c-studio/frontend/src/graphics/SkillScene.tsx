import { Button, Text } from '@webkit/atoms';
import { useEffect, useRef, useState } from 'react';
import { skillFrame } from '@game/scenes/effects/skillLooks';
import type { SkillLook } from '@game/scenes/effects/effects';
import { zeichneHintergrund, zeichneMonarch } from './backdrop';
import { pixelContext } from './pixel';
import { SKILL_SCALES, skillRows, type Licht, type SkillRow } from './gameScenesLogic';

// Szene „Skill-Effekte“ (B-598): je Skill die Animation des Effekts am Monarchen in Spielgröße und ×2/×3.
// Ring und Partikel kommen aus `skillFrame` (Spielcode, gleiche Werte wie die Tweens in effectsView), Rückfall ist markiert.

const W = 120;
const H = 100;
const GROUND = 84;
/** Höhe über dem Boden, in der der Effekt am Monarchen erscheint (`BODY_PX` in effects.ts) */
const BODY = 40;
const STILL = 0.3;

const hex = (color: number) => `#${color.toString(16).padStart(6, '0')}`;

function zeichne(c: HTMLCanvasElement, look: SkillLook, scale: number, p: number, biom: string, licht: Licht): void {
  const g = pixelContext(c, W * scale, H * scale);
  if (!g) return;
  g.scale(scale, scale);
  zeichneHintergrund(g, W, H, GROUND, biom, licht);
  zeichneMonarch(g, W / 2, GROUND);
  for (const s of skillFrame(look, p)) {
    g.globalAlpha = s.alpha;
    g.fillStyle = hex(look.color);
    g.beginPath();
    g.arc(W / 2 + s.x, GROUND - BODY + s.y, s.radius * s.scale, 0, Math.PI * 2);
    g.fill();
  }
  g.globalAlpha = 1;
}

function SkillCanvas({ look, scale, biom, licht, tick }: { look: SkillLook; scale: number; biom: string; licht: Licht; tick: number }) {
  const ref = useRef<HTMLCanvasElement>(null);
  useEffect(() => {
    const c = ref.current;
    if (!c) return;
    if (tick === 0) return zeichne(c, look, scale, STILL, biom, licht);
    const t0 = performance.now();
    let raf = 0;
    const step = (now: number) => {
      const p = (now - t0) / look.durationMs;
      zeichne(c, look, scale, Math.min(1, p), biom, licht);
      if (p < 1) raf = requestAnimationFrame(step);
      else zeichne(c, look, scale, STILL, biom, licht);
    };
    raf = requestAnimationFrame(step);
    return () => cancelAnimationFrame(raf);
  }, [look, scale, biom, licht, tick]);
  return <canvas ref={ref} className="sz-canvas" />;
}

function Zeile({ r, biom, licht, tick }: { r: SkillRow; biom: string; licht: Licht; tick: number }) {
  if (!r.look) return <tr><td>{r.id}</td><td>{r.line} · {r.kind}</td><td colSpan={4}><span className="sz-gap">Linie ohne Farbe: kein Effekt</span></td></tr>;
  const look = r.look;
  return (
    <tr>
      <td><b>{r.id}</b>{r.fallback && <span className="sz-nc"> Rückfall</span>}</td>
      <td>{r.line} · {r.kind} · Stufe {r.tier}</td>
      <td className="sz-dim">
        <i style={{ background: hex(look.color) }} className="sz-swatch" /> r {look.radius} px · {look.durationMs} ms · {look.particles} Partikel · {look.motion}
      </td>
      {SKILL_SCALES.map((s) => <td key={s}><SkillCanvas look={look} scale={s} biom={biom} licht={licht} tick={tick} /></td>)}
    </tr>
  );
}

export function SkillScene({ biom, licht }: { biom: string; licht: Licht }) {
  const [tick, setTick] = useState(0);
  const rows = skillRows();
  return (
    <>
      <div><Button size="1" onClick={() => setTick((t) => t + 1)}>Abspielen</Button></div>
      <Text size="2" color="gray">Effekt am Monarchen in Spielgröße ×1, ×2, ×3; „Rückfall“ = Ring der Linie ohne eigenen Look.</Text>
      <table className="sz-table">
        <thead><tr><th>Skill</th><th>Linie · Art</th><th>Farbe · Radius · Dauer</th>{SKILL_SCALES.map((s) => <th key={s}>×{s}</th>)}</tr></thead>
        <tbody>{rows.map((r) => <Zeile key={r.id} r={r} biom={biom} licht={licht} tick={tick} />)}</tbody>
      </table>
    </>
  );
}
