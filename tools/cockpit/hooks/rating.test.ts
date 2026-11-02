import { expect, test } from 'claude-code/testing'

import { bandRate as blastRate, paneView as blastView } from './blast'
import { bandRate as inspectRate } from './inspect'
import { METRICS, limitText, rate, ratingSection, worst, type Metric } from './rating'

test('rate: Grenzen gelten inklusive, „low“ aufwärts und „high“ abwärts', () => {
  expect([0, 10, 11, 20, 21, 35, 36].map(v => rate('overhead', v))).toEqual(['great', 'great', 'good', 'good', 'poor', 'poor', 'bad'])
  expect([100, 80, 79, 60, 59, 50, 49].map(v => rate('cache', v))).toEqual(['great', 'great', 'good', 'good', 'poor', 'poor', 'bad'])
  expect([0, 1, 2, 3, 5, 6].map(v => rate('unresolved', v))).toEqual(['great', 'good', 'good', 'poor', 'poor', 'bad'])
  expect([2_000, 3_000, 9_000, 20_000].map(v => rate('ttft', v))).toEqual(['great', 'great', 'poor', 'bad'])
})

test('Grenzen sind je Metrik geordnet; limitText nennt sie in der Richtung der Metrik', () => {
  for (const [name, m] of Object.entries(METRICS) as [Metric, (typeof METRICS)[Metric]][]) {
    const [a, b, c] = m.limits
    expect(m.dir === 'low' ? a <= b && b <= c : a >= b && b >= c).toBe(true)
    expect(rate(name, m.dir === 'low' ? c + 1 : c - 1)).toBe('bad')
  }
  expect(limitText('overhead')).toBe('≤10 / ≤20 / ≤35 %')
  expect(limitText('cache')).toBe('≥80 / ≥60 / ≥50 %')
  expect(limitText('ttft')).toBe('≤3 / ≤8 / ≤15 s')
  expect(limitText('affected')).toBe('≤5 / ≤15 / ≤40')
  expect(ratingSection('en', ['waste']).rows).toEqual([{ label: 'Waste', value: '≤2 / ≤5 / ≤15 %' }])
})

test('worst: schlechteste Stufe, undefined wird übergangen', () => {
  expect(worst('great', undefined, 'poor', 'good')).toBe('poor')
  expect(worst('bad', 'great')).toBe('bad')
  expect(worst(undefined)).toBe(undefined)
})

test('Blast: Ampel nach Reichweite, Testabdeckung und Unaufgelöstem', () => {
  const r = (affected: number, tests: number, unresolved = 0) => ({
    targets: ['a.ts'], affected: Array.from({ length: affected }, (_, i) => `f${i}.ts`), tests: Array.from({ length: tests }, (_, i) => `t${i}.test.ts`), unresolved: Array.from({ length: unresolved }, (_, i) => `u${i}`), incomplete: false,
  })
  expect(blastRate(null)).toBe(undefined)
  expect(blastRate(r(3, 3))).toBe('great')
  expect(blastRate(r(3, 0))).toBe('bad')
  expect(blastRate(r(10, 4))).toBe('good')
  expect(blastRate({ ...r(3, 3), incomplete: true })).toBe('poor')
  const rows = blastView(r(3, 0), [], 'de').flatMap(s => s.rows)
  // Bewertung (schlechteste zuerst) mit Sätzen, dann die Ampeln der Abschnitte.
  expect(rows.filter(x => x.rate !== undefined).map(x => x.rate)).toEqual(['bad', 'great', 'great', 'bad'])
})

test('Inspect: Kontextfüllung bestimmt die Band-Ampel, Verschwendung ebenso', () => {
  const sum = { waste: 0, liveEst: 10_000 } as Parameters<typeof inspectRate>[0]
  expect(inspectRate(sum, 40)).toBe('great')
  expect(inspectRate(sum, 90)).toBe('bad')
  expect(inspectRate({ ...sum, waste: 1_000 }, 40)).toBe('poor')
  expect(inspectRate(sum, undefined)).toBe('great')
})
