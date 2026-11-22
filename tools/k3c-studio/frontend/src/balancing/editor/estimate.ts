import type { BalanceRun } from '../../api';
import type { BalanceForm } from '../spec';

// Zusammenfassung und Dauer-Schätzung der Fußleiste (B-661): Dauer aus früheren Läufen gleicher Welt und Tage,
// sonst die Faustregel aus help.ts („40 Tage ≈ 1–4 min je Seed“, nur für eine Welt).

/** „Alle“ Seeds = Seeds der Ziele (data/balance-targets.json, heute 100). */
export const ALL_SEEDS = 100;

const MIN_PER_SEED: [number, number] = [1, 4];
const WORLD_DAYS = 40;

export function seedCount(form: BalanceForm): number {
  const list = form.seedList.split(',').map((s) => s.trim()).filter(Boolean);
  if (list.length > 0) return list.length;
  return Number(form.seeds) > 0 ? Number(form.seeds) : ALL_SEEDS;
}

export interface Work {
  /** Gleich große Auswertungen über alle Seeds: Läufe, Rasterpunkte oder Budget der Suche. */
  count: number;
  /** Zusatz für die Zusammenfassung, z. B. „bis zu 40 Auswertungen“; leer bei einem Lauf. */
  text: string;
}

/** Umfang der Arbeit: Suche bis zum Budget (40), Sweep Schritte^Parameter (5), „je Variante“ n + 1 Läufe. */
export function workOf(form: BalanceForm, counts: { fest: number; bereich: number }, perVariant: boolean): Work {
  if (counts.bereich > 0) {
    if (form.tune.mode === 'sweep') {
      const n = (Number(form.tune.steps) || 5) ** Math.min(counts.bereich, 2);
      return { count: n, text: `${n} Rasterpunkte` };
    }
    const n = Number(form.tune.budget) || 40;
    return { count: n, text: `bis zu ${n} Auswertungen` };
  }
  if (perVariant && counts.fest > 0) return { count: counts.fest + 1, text: `${counts.fest + 1} Läufe` };
  return { count: 1, text: '' };
}

const seedsOf = (r: BalanceRun): number => r.spec?.seedList?.length || r.spec?.seeds || ALL_SEEDS;

/** Minuten je Seed aus fertigen Läufen gleicher Welt und Tage (Median); undefined ohne Treffer. */
export function minutesPerSeed(runs: readonly BalanceRun[], form: BalanceForm): number | undefined {
  const per = runs
    .filter((r) => r.state === 'fertig' && r.ended && r.spec && !r.spec.tune && (r.spec.world ?? '') === form.world && (r.spec.days ?? 0) === (Number(form.days) || 0))
    .map((r) => (Date.parse(r.ended!) - Date.parse(r.started)) / 60000 / seedsOf(r))
    .filter((m) => Number.isFinite(m) && m > 0)
    .sort((a, b) => a - b);
  return per.length === 0 ? undefined : per[Math.floor(per.length / 2)];
}

export function formatMinutes(min: number): string {
  if (min < 1) return 'unter 1 min';
  if (min < 60) return `${Math.round(min)} min`;
  const h = Math.floor(min / 60);
  const m = Math.round(min - h * 60);
  return m === 0 ? `${h} h` : `${h} h ${m} min`;
}

/** Text der geschätzten Dauer mit Quelle; „unbekannt“ für eine Einzelstufe ohne frühere Läufe. */
export function durationText(runs: readonly BalanceRun[], form: BalanceForm, work: Work): string {
  const seeds = seedCount(form) * work.count;
  const own = minutesPerSeed(runs, form);
  if (own !== undefined) return `etwa ${formatMinutes(own * seeds)} (nach früheren Läufen)`;
  if (form.world === '') return 'Dauer unbekannt';
  const scale = (Number(form.days) || WORLD_DAYS) / WORLD_DAYS;
  const [lo, hi] = MIN_PER_SEED.map((m) => m * seeds * scale);
  const [a, b] = [formatMinutes(lo), formatMinutes(hi)];
  return `etwa ${a === b ? a : `${a} bis ${b}`} (Faustregel)`;
}

/** Fußleiste: „8 Seeds × 1 Spieler × 40 Tage · bis zu 40 Auswertungen · etwa …“. */
export function summaryText(runs: readonly BalanceRun[], form: BalanceForm, work: Work): string {
  const seeds = seedCount(form);
  const seedText = form.seeds === '' && form.seedList.trim() === '' ? 'alle Seeds' : `${seeds} ${seeds === 1 ? 'Seed' : 'Seeds'}`;
  const players = Number(form.players) || 2;
  const days = form.days.trim() ? `${form.days.trim()} Tage` : 'Tage je Ziel';
  const parts = [`${seedText} × ${players} Spieler × ${days}`, work.text, durationText(runs, form, work)];
  return parts.filter(Boolean).join(' · ');
}
