// Atemübungen, solange Claude arbeitet (Idee: halluton/Mindful-Claude, MIT; eigene Umsetzung).
// Rein, ohne `$`: die Phasen-Rechnung für den Spinner und ein Svg, das die Atmung per SMIL selbst animiert.
// Das Svg ändert sich während eines Turns nicht, also zeichnet das Band es nicht neu (interaktive Svg flackern sonst).

export type BreathKey = 'hrv' | 'sigh' | 'box' | '478'
export type BreathPhase = 'in' | 'sip' | 'hold' | 'out'

export type Breath = { key: BreathKey; inMs: number; hold1Ms: number; outMs: number; hold2Ms: number; sip?: true }

export const BREATHS: readonly Breath[] = [
  { key: 'hrv', inMs: 5500, hold1Ms: 0, outMs: 5500, hold2Ms: 0 },
  // Physiologischer Seufzer: zweites, kurzes Nachatmen statt Halten, dann langes Ausatmen.
  { key: 'sigh', inMs: 4000, hold1Ms: 1000, outMs: 10000, hold2Ms: 0, sip: true },
  { key: 'box', inMs: 4000, hold1Ms: 4000, outMs: 4000, hold2Ms: 4000 },
  { key: '478', inMs: 4000, hold1Ms: 7000, outMs: 8000, hold2Ms: 0 },
]

export const isBreathKey = (v: unknown): v is BreathKey => BREATHS.some(b => b.key === v)
export const breathOf = (key: unknown): Breath => BREATHS.find(b => b.key === key) ?? (BREATHS[0] as Breath)

// Abschnitte eines Atemzugs: Phase, Dauer, Füllstand der Lunge am Ende (0..1). Abschnitte ohne Dauer fallen weg.
export function segments(b: Breath): { phase: BreathPhase; ms: number; to: number }[] {
  return [
    { phase: 'in' as const, ms: b.inMs, to: b.sip ? 0.85 : 1 },
    { phase: b.sip ? ('sip' as const) : ('hold' as const), ms: b.hold1Ms, to: 1 },
    { phase: 'out' as const, ms: b.outMs, to: 0 },
    { phase: 'hold' as const, ms: b.hold2Ms, to: 0 },
  ].filter(s => s.ms > 0)
}

export const cycleMs = (b: Breath): number => b.inMs + b.hold1Ms + b.outMs + b.hold2Ms

// Wo im Atemzug `elapsedMs` seit Beginn liegt, und wie viele Millisekunden die Phase noch dauert.
export function phaseAt(b: Breath, elapsedMs: number): { phase: BreathPhase; leftMs: number } {
  const cycle = cycleMs(b)
  let t = ((Math.floor(elapsedMs) % cycle) + cycle) % cycle
  for (const s of segments(b)) {
    if (t < s.ms) return { phase: s.phase, leftMs: s.ms - t }
    t -= s.ms
  }
  return { phase: 'in', leftMs: b.inMs }
}

// Füllstand der Lunge (0..1) zu `elapsedMs`: Ein-/Ausatmen sanft auslaufend, Halten konstant.
export function fillAt(b: Breath, elapsedMs: number): number {
  const cycle = cycleMs(b)
  let t = ((Math.floor(elapsedMs) % cycle) + cycle) % cycle
  let from = 0
  for (const s of segments(b)) {
    if (t < s.ms) {
      const x = t / s.ms
      return from + (s.to - from) * (s.phase === 'hold' ? 1 : 1 - (1 - x) * (1 - x))
    }
    t -= s.ms
    from = s.to
  }
  return 0
}

// Ein Einzelbild für den Turn: das Band zeichnet während der Arbeit ständig neu und startet SMIL dabei von vorn,
// deshalb rechnet hier der 250-ms-Takt den Kreis selbst. Ohne Animation, also auch als einfaches Bild stabil.
export function breathFrameSvg(b: Breath, elapsedMs: number, labels: Record<BreathPhase, string>, name: string, color: string, muted: string, width = 260, height = 56): string {
  const cy = height / 2
  const r = 5 + (cy - 3 - 5) * fillAt(b, elapsedMs)
  const ph = phaseAt(b, elapsedMs)
  return (
    `<svg xmlns="http://www.w3.org/2000/svg" width="${width}" height="${height}" viewBox="0 0 ${width} ${height}" font-family="system-ui, sans-serif">` +
    `<circle cx="${cy}" cy="${cy}" r="${cy - 3}" fill="none" stroke="${color}" stroke-opacity="0.25"/>` +
    `<circle cx="${cy}" cy="${cy}" r="${r3(r)}" fill="${color}" fill-opacity="0.55"/>` +
    `<text x="${height + 10}" y="${cy - 2}" fill="${color}" font-size="15" font-weight="600">${esc(labels[ph.phase])} ${Math.ceil(ph.leftMs / 1000)}s</text>` +
    `<text x="${height + 10}" y="${cy + 15}" fill="${muted}" font-size="11">${esc(name)}</text>` +
    `</svg>`
  )
}

const r3 = (n: number): string => String(Math.round(n * 1000) / 1000)
const esc = (s: string): string => s.replace(/&/g, '&amp;').replace(/</g, '&lt;').replace(/>/g, '&gt;')

// Ein Atemzug als SMIL: der Kreis wächst und schrumpft (Ein-/Ausatmen sanft auslaufend, Halten gerade),
// daneben die Phase als Wort, das genau in ihrem Zeitfenster sichtbar ist.
export function breathSvg(b: Breath, labels: Record<BreathPhase, string>, name: string, color: string, muted: string, width = 260, height = 56): string {
  const segs = segments(b)
  const cycle = cycleMs(b)
  const cy = height / 2
  const rMin = 5
  const rMax = cy - 3
  const dur = `${cycle / 1000}s`
  const times: number[] = [0]
  for (const s of segs) times.push((times[times.length - 1] ?? 0) + s.ms / cycle)
  times[times.length - 1] = 1
  const radius = (f: number) => r3(rMin + (rMax - rMin) * f)
  const values = [radius(0), ...segs.map(s => radius(s.to))].join(';')
  const splines = segs.map(s => (s.phase === 'hold' ? '0 0 1 1' : '0.2 0 0.4 1')).join(';')
  const grow = (attr: string) =>
    `<animate attributeName="${attr}" dur="${dur}" repeatCount="indefinite" values="${values}" keyTimes="${times.map(r3).join(';')}" calcMode="spline" keySplines="${splines}"/>`
  // Wort je Abschnitt: diskret sichtbar von seinem Start bis zu seinem Ende.
  const words = segs
    .map((s, i) => {
      const from = times[i] ?? 0
      const to = times[i + 1] ?? 1
      const vals = [from > 0 ? '0' : null, '1', to < 1 ? '0' : null].filter(v => v !== null).join(';')
      const keys = [from > 0 ? '0' : null, r3(from), to < 1 ? r3(to) : null].filter(v => v !== null).join(';')
      return `<text x="${height + 10}" y="${cy - 2}" fill="${color}" font-size="15" font-weight="600" opacity="0">${esc(labels[s.phase])}<animate attributeName="opacity" dur="${dur}" repeatCount="indefinite" calcMode="discrete" values="${vals}" keyTimes="${keys}"/></text>`
    })
    .join('')
  return (
    `<svg xmlns="http://www.w3.org/2000/svg" width="${width}" height="${height}" viewBox="0 0 ${width} ${height}" font-family="system-ui, sans-serif">` +
    `<circle cx="${cy}" cy="${cy}" r="${rMax}" fill="none" stroke="${color}" stroke-opacity="0.25"/>` +
    `<circle cx="${cy}" cy="${cy}" r="${rMin}" fill="${color}" fill-opacity="0.55">${grow('r')}</circle>` +
    words +
    `<text x="${height + 10}" y="${cy + 15}" fill="${muted}" font-size="11">${esc(name)}</text>` +
    `</svg>`
  )
}
