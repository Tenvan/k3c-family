// Ampel für die Kennzahlen der Diagnose-Module: vier Stufen (sehr gut, gut, schlecht, sehr schlecht) je Messgröße. Rein, ohne `$`.
// Alle Schwellen stehen in METRICS und werden daraus auch als Text ins Panel geschrieben, damit Anzeige und Erklärung nicht
// auseinanderlaufen. Bewertet wird nur, wo eine Aussage trägt (Anteile, Zeiten, Mengen); Beschreibendes bleibt ohne Ampel.

import type { UiLang } from '../types'

export type Rate = 'great' | 'good' | 'poor' | 'bad'
export type Metric = 'overhead' | 'approval' | 'ttft' | 'cache' | 'tool' | 'context' | 'waste' | 'affected' | 'tests' | 'unresolved'

export const RATE_SYMBOL = '●'

// `low`: kleiner ist besser (Wert ≤ Grenze), `high`: größer ist besser (Wert ≥ Grenze). Grenzen: sehr gut, gut, schlecht;
// was darüber (bzw. darunter) liegt, ist sehr schlecht. Einheiten: pct = Prozent, s = Sekunden (Wert in ms), n = Anzahl.
export const METRICS: Record<Metric, { limits: readonly [number, number, number]; dir: 'low' | 'high'; unit: 'pct' | 's' | 'n' }> = {
  overhead: { limits: [10, 20, 35], dir: 'low', unit: 'pct' },
  approval: { limits: [5, 10, 20], dir: 'low', unit: 'pct' },
  ttft: { limits: [3_000, 8_000, 15_000], dir: 'low', unit: 's' },
  cache: { limits: [80, 60, 50], dir: 'high', unit: 'pct' },
  tool: { limits: [10_000, 60_000, 120_000], dir: 'low', unit: 's' },
  context: { limits: [50, 70, 85], dir: 'low', unit: 'pct' },
  waste: { limits: [2, 5, 15], dir: 'low', unit: 'pct' },
  affected: { limits: [5, 15, 40], dir: 'low', unit: 'n' },
  tests: { limits: [50, 25, 1], dir: 'high', unit: 'pct' },
  unresolved: { limits: [0, 2, 5], dir: 'low', unit: 'n' },
}

export function rate(metric: Metric, value: number): Rate {
  const { limits, dir } = METRICS[metric]
  const [a, b, c] = limits
  if (dir === 'low') return value <= a ? 'great' : value <= b ? 'good' : value <= c ? 'poor' : 'bad'
  return value >= a ? 'great' : value >= b ? 'good' : value >= c ? 'poor' : 'bad'
}

const ORDER: Record<Rate, number> = { great: 0, good: 1, poor: 2, bad: 3 }

// Die schlechteste von mehreren Stufen (für Kurzinfos, die mehrere Werte zusammenfassen); ohne Stufe: undefined.
export function worst(...rates: readonly (Rate | undefined)[]): Rate | undefined {
  let out: Rate | undefined
  for (const r of rates) if (r !== undefined && (out === undefined || ORDER[r] > ORDER[out])) out = r
  return out
}

const DE = {
  'rate.great': 'sehr gut',
  'rate.good': 'gut',
  'rate.poor': 'schlecht',
  'rate.bad': 'sehr schlecht',
  'rate.legend': 'Ampel',
  'rate.sec': 'Ampel-Schwellen',
  'rate.note': 'Reihenfolge: sehr gut / gut / schlecht; darüber (bei „≥“ darunter) sehr schlecht.',
  'rate.metric.overhead': 'Engine-Overhead',
  'rate.metric.approval': 'Freigabe-Wartezeit',
  'rate.metric.ttft': 'TTFT Ø',
  'rate.metric.cache': 'Cache-Trefferquote',
  'rate.metric.tool': 'Tool-Dauer',
  'rate.metric.context': 'Kontextfüllung (Band)',
  'rate.metric.waste': 'Verschwendung',
  'rate.metric.affected': 'Betroffene Dateien',
  'rate.metric.tests': 'Testabdeckung',
  'rate.metric.unresolved': 'Unaufgelöste Importe',
} as const

const EN: Record<keyof typeof DE, string> = {
  'rate.great': 'very good',
  'rate.good': 'good',
  'rate.poor': 'poor',
  'rate.bad': 'very poor',
  'rate.legend': 'Rating',
  'rate.sec': 'Rating thresholds',
  'rate.note': 'Order: very good / good / poor; beyond that (below for “≥”) very poor.',
  'rate.metric.overhead': 'Engine overhead',
  'rate.metric.approval': 'Approval wait',
  'rate.metric.ttft': 'TTFT avg',
  'rate.metric.cache': 'Cache hit rate',
  'rate.metric.tool': 'Tool duration',
  'rate.metric.context': 'Context fill (band)',
  'rate.metric.waste': 'Waste',
  'rate.metric.affected': 'Affected files',
  'rate.metric.tests': 'Test coverage',
  'rate.metric.unresolved': 'Unresolved imports',
}

const TEXT = { de: DE, en: EN } as const
export type RateKey = keyof typeof DE
export const rt = (L: UiLang, key: RateKey): string => TEXT[L][key]
export const rateLabel = (L: UiLang, r: Rate): string => rt(L, `rate.${r}`)

// „≤10 / ≤20 / ≤35 %“, „≥80 / ≥60 / ≥50 %“, „≤3 / ≤8 / ≤15 s“, „≤5 / ≤15 / ≤40“.
export function limitText(metric: Metric): string {
  const { limits, dir, unit } = METRICS[metric]
  const sign = dir === 'low' ? '≤' : '≥'
  const shown = limits.map(v => (unit === 's' ? v / 1000 : v))
  return `${shown.map(v => `${sign}${v}`).join(' / ')}${unit === 'pct' ? ' %' : unit === 's' ? ' s' : ''}`
}

// Abschnitt „Ampel-Schwellen“ für die Metriken, die ein Modul bewertet.
export function ratingSection(L: UiLang, metrics: readonly Metric[]): { title: string; rows: { label: string; value: string }[]; note: string } {
  return {
    title: rt(L, 'rate.sec'),
    rows: metrics.map(m => ({ label: rt(L, `rate.metric.${m}`), value: limitText(m) })),
    note: rt(L, 'rate.note'),
  }
}

// Stufe als Klartext-Anhang für Berichte im Chat: „ [gut]“.
export const rateTag = (L: UiLang, r: Rate | undefined): string => (r === undefined ? '' : ` [${rateLabel(L, r)}]`)
