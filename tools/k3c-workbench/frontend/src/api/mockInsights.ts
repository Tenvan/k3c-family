import type { BalanceBackend, BalanceInsights, BalanceSummary, Light, TargetInsight } from './types';

// Mock der Auswertung (B-632): leitet aus einem Beispielbericht Ampeln und Texte ab, wie es die Go-Seite tut (vereinfacht).

const LIGHT: Record<string, Light> = { ok: 'green', knapp: 'yellow', verletzt: 'red' };

function target(e: BalanceSummary['targets'][number]): TargetInsight {
  const light = LIGHT[e.status] ?? 'grey';
  return {
    name: e.name, measure: e.measure, kind: e.kind, unit: e.kind === 'share' ? '%' : '', grade: e.grade || 'normal',
    scenario: `${e.players} Spieler · ${e.bot} · Tiefe ${e.depth} · ${e.days} Tage`, value: e.value, lower: e.lower, upper: e.upper,
    status: e.status, light, distance: light === 'grey' ? null : light === 'red' ? -25 : 20, trend: null,
    failSeeds: e.failSeeds ?? null, invalidSeeds: e.invalidSeeds ?? null, rule: e.rule,
    levers: light === 'green' ? [] : [{ path: 'buildings.json:wall.levels.0.hp', direction: '+', step: 0.2, label: 'Mauer-HP Stufe 1 erhöhen' }],
  };
}

/** Auswertung zu einem Beispielbericht. */
export function exampleInsights(path: string, s: BalanceSummary): BalanceInsights {
  const targets = s.targets.map(target);
  const counts: Record<Light, number> = { green: 0, yellow: 0, red: 0, grey: 0 };
  for (const t of targets) counts[t.light]++;
  const red = targets.filter((t) => t.light === 'red');
  return {
    title: path, run: path.split('/').slice(-2, -1)[0] || 'reports', generated: '25.10.2026 06:21', seeds: s.seeds, pass: s.pass, counts,
    summary: [`${counts.red} von ${targets.length} Zielen verletzt, ${counts.yellow} knapp, ${counts.green} im Korridor: der Lauf ${s.pass ? 'besteht' : 'besteht nicht'}.`,
      ...(red.length ? [`Am weitesten daneben: ${red[0].name}.`, 'Größter Hebel: Mauer-HP Stufe 1 erhöhen (buildings.json:wall.levels.0.hp, Schritt 20 %).'] : []),
      'Alle Kennzahlen liegen im oder nahe am Richtwert.'],
    strengths: targets.filter((t) => t.light === 'green').map((t) => `${t.name}: im Korridor.`),
    weaknesses: red.map((t) => `${t.name}: ${t.value} statt Korridor.`),
    targets,
    figures: { note: 'Kennzahlen aus 8 gültigen Läufen.', bands: {}, nights: s.kennzahlen?.nights ?? [],
      lights: [{ key: 'firstWallTick', name: 'Erste Mauer', unit: 'Ticks', value: 3400, light: 'red', text: 'Richtwert grün ≤ 1500, gelb ≤ 3000' },
        { key: 'idleShare', name: 'Leerlauf der Bauern', unit: '%', grade: 'normal', value: 18, light: 'green', text: 'Richtwert grün ≤ 25, gelb ≤ 35' }] },
    hints: red.map((t) => ({ target: t.name, light: 'red' as Light, text: 'zu hoch: Mauer-HP Stufe 1 erhöhen (+20 %) probieren.', why: `${t.value} bei Korridor.`, levers: t.levers })),
    next: { steps: red.length ? ['Seeds 1, 2 im Replay nachspielen.', 'Nach jeder Änderung den vollen Lauf wiederholen.'] : ['Alle Ziele grün: Baseline neu schreiben.'],
      seeds: red.length ? ['1', '2'] : [],
      commands: [{ label: 'Voller Lauf zum Vergleich', where: 'chat', text: `Starte sim_test {"action":"start","mode":"offline","seeds":${s.seeds}}.` },
        { label: 'Replays schreiben', where: 'shell', text: `task balance:targets -- --seeds ${s.seeds}` }] },
    baseline: { present: false, note: 'Keine Baseline: kein Trend.', changes: [] },
    varianten: s.varianten ?? [],
  };
}

/** Auswertung und „HTML-Bericht öffnen“ im Mock zu den Beispielberichten. */
export function mockInsights(summaries: Map<string, BalanceSummary>): Pick<BalanceBackend, 'balanceInsights' | 'balanceReportHTML'> {
  const get = (path: string) => {
    const s = summaries.get(path);
    if (!s) throw new Error(`bericht ${path} unbekannt`);
    return s;
  };
  return {
    balanceInsights: async (path) => exampleInsights(path, get(path)),
    balanceReportHTML: async (path) => {
      get(path);
      console.info('[mock] HTML-Bericht öffnen', path);
      return path.replace(/\.json$/, '.html');
    },
  };
}
