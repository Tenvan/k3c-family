import type { BalanceBackend, BalanceBaseline, BalanceReportInfo, BalanceRun, ReportRating } from './types';

// Berichte verwalten im Mock (WZ28.13, B-662): Bewerten und Löschen einzeln und gesammelt, Baseline sichern.
// Wie Go prüfen Sammel-Aufrufe erst alle Pfade; ein unbekannter lehnt alles ab.

/** Ereignisse des Balancing-Mocks, wie Go sie schickt (balance:runs, balance:reports). */
export interface BalanceEvents {
  run(run: BalanceRun): void;
  reports(): void;
}

export function mockManageReports(
  reports: BalanceReportInfo[],
  events: BalanceEvents,
): Pick<BalanceBackend, 'balanceReportRate' | 'balanceReportDelete' | 'balanceReportsRate' | 'balanceReportsDelete'> {
  const pick = (paths: string[]) => {
    if (paths.length === 0) throw new Error('keine Berichte gewählt');
    return paths.map((path) => {
      const r = reports.find((x) => x.path === path);
      if (!r) throw new Error(`bericht ${path} unbekannt`);
      return r;
    });
  };
  const rate = async (paths: string[], rating: ReportRating) => {
    for (const r of pick(paths)) r.rating = rating || undefined;
    events.reports();
  };
  const remove = async (paths: string[]) => {
    for (const r of pick(paths)) {
      const i = reports.indexOf(r);
      if (i >= 0) reports.splice(i, 1);
    }
    events.reports();
  };
  return {
    balanceReportRate: (path, rating) => rate([path], rating),
    balanceReportDelete: (path) => remove([path]),
    balanceReportsRate: rate,
    balanceReportsDelete: remove,
  };
}

const BASELINE_NAME = /^[A-Za-z0-9_-]{1,40}$/;

/** Gesicherte Basis-Läufe (B-640) und „Als Baseline sichern“ (B-662) mit den Ablehnungen von Go. */
export function mockBaselines(
  baselines: BalanceBaseline[],
  reports: BalanceReportInfo[],
  events: BalanceEvents,
): Pick<BalanceBackend, 'balanceBaselines' | 'balanceSaveBaseline'> {
  return {
    balanceBaselines: async () => structuredClone(baselines),
    balanceSaveBaseline: async (path, name) => {
      const r = reports.find((x) => x.path === path);
      if (!r) throw new Error(`bericht "${path}" gibt es nicht`);
      if (r.kind !== 'balance' || !r.run) throw new Error(`bericht "${path}": nur balance-*.json im Ordner eines Laufs (reports/<lauf>/)`);
      if (!BASELINE_NAME.test(name)) throw new Error(`baseline "${name}": Buchstaben, Ziffern, _ und -, höchstens 40 Zeichen`);
      if (!r.pass) throw new Error(`baseline ${name}: der Lauf hat sein Ziel nicht erreicht (Pass fehlt)`);
      if (baselines.some((b) => b.name === name)) throw new Error(`baseline ${name} gibt es schon (reports/baseline/${name})`);
      const b: BalanceBaseline = {
        name, saved: new Date().toISOString(), run: r.run, seeds: r.seeds, seedMode: 'dev', bots: r.spec?.bot ? [r.spec.bot] : [],
        worlds: r.spec?.world ? [r.spec.world] : [], pass: r.pass, frozen: [], spec: r.spec,
      };
      baselines.push(b);
      baselines.sort((x, y) => x.name.localeCompare(y.name));
      events.reports();
      return structuredClone(b);
    },
  };
}
