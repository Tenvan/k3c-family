// Auswahl: Welche Prüfungen lösen die geänderten Dateien eines Commits aus? Reine Logik ohne Git und Dateisystem, damit
// sie als Tabelle testbar bleibt (select.test.ts). Regeln: docs/arbeitsweise.md › Tempo im Lauf.
//
// Rust (Projekt RSU, RU1.2; Werkzeuge RU66.1): Es gibt zwei Cargo-Workspaces, das Spiel rust/ und die Werkzeuge
// rust/tools/ (eigenes [workspace], Entscheidung 014). Geänderte Dateien unter <workspace>/crates/<name>/ wählen fmt,
// clippy und Tests dieser Crate; Manifest, Lockfile oder clippy.toml direkt im Workspace-Ordner wählen den ganzen
// Workspace. rust/rust-toolchain.toml und rust/rustfmt.toml erbt rust/tools/, sie wählen beide Workspaces ganz. Ohne
// Rust-Datei läuft kein cargo.

import path from 'node:path';

export interface RustPlan {
  /** Manifest, Lockfile oder Toolchain geändert: ganzer Workspace */
  all: boolean;
  /** geänderte Crates (Ordnername = Paketname) */
  crates: Set<string>;
}

export interface Plan {
  /** Planungs-Tests (docs/, *.md) */
  docs: boolean;
  /** Regel-Test des Repos (alles außer Doku) */
  rules: boolean;
  /** Oxlint, tsc, Vitest --changed */
  client: boolean;
  /** Vitest der Workbench (ihr Frontend und tools/webkit) */
  wbVitest: boolean;
  /** Typecheck und Vitest des Studios (sein Frontend und tools/webkit, WZ63.3) */
  studioVitest: boolean;
  /** Workspace rust/; fehlt ohne Rust-Dateien */
  rust?: RustPlan;
  /** Workspace rust/tools/ der Rust-Werkzeuge; fehlt ohne deren Dateien */
  rustTools?: RustPlan;
}

/** Ordner, deren Dateien Lint, Typecheck und Vitest der Wurzel brauchen (der Browser-Client src/ ist entfernt, B-848). */
const CLIENT_ROOTS = ['tests/', 'public/', 'data/', 'tools/paper/', 'tools/checkdiff/', 'tools/cleanup/', 'tools/sync/', 'tools/rustpaket/',
    'tools/ruletable/', 'tools/soundatlas/'];
const CLIENT_EXTS = new Set(['.html', '.ts', '.js', '.mjs', '.json', '.lock', '.lockb']);
const RUST_ROOT = 'rust/';
/** Dateien direkt im Workspace-Ordner, die jede Crate dieses Workspace betreffen. */
const RUST_WORKSPACE_FILES = new Set(['Cargo.toml', 'Cargo.lock', 'clippy.toml']);
/** Dateien in rust/, die auch rust/tools/ erbt (Suche nach oben durch rustup und rustfmt). */
const RUST_INHERITED_FILES = new Set(['rust-toolchain.toml', 'rustfmt.toml']);

export function emptyPlan(): Plan {
  return { docs: false, rules: false, client: false, wbVitest: false, studioVitest: false };
}

const hasPrefix = (f: string, ...prefixes: string[]): boolean => prefixes.some((p) => f.startsWith(p));

const isDoc = (f: string): boolean => hasPrefix(f, 'docs/', 'reports/') || f.endsWith('.md');

/** Endung ab dem letzten Punkt des Dateinamens, auch bei Punkt-Dateien (wie path.Ext in Go). */
function ext(f: string): string {
  const base = f.slice(f.lastIndexOf('/') + 1);
  const dot = base.lastIndexOf('.');
  return dot < 0 ? '' : base.slice(dot);
}

/** Ordner des Clients oder eine Konfigurations- bzw. Seiten-Datei an der Repo-Wurzel. */
function isClient(f: string): boolean {
  if (hasPrefix(f, ...CLIENT_ROOTS)) return true;
  return !f.includes('/') && CLIENT_EXTS.has(ext(f));
}

/** Ordnet Dateien der Wails-Oberflächen zu; true = Datei ist damit erledigt. */
function frontend(p: Plan, f: string): boolean {
  if (f.startsWith('tools/webkit/')) {
    p.wbVitest = true;
    p.studioVitest = true;
  } else if (f.startsWith('tools/k3c-workbench/frontend/')) {
    p.wbVitest = true;
  } else if (f.startsWith('tools/k3c-studio/frontend/')) {
    p.studioVitest = true;
  } else {
    return false;
  }
  return true;
}

const newRustPlan = (): RustPlan => ({ all: false, crates: new Set() });
const rustOf = (p: Plan): RustPlan => (p.rust ??= newRustPlan());
const rustToolsOf = (p: Plan): RustPlan => (p.rustTools ??= newRustPlan());

/** Wählt zu einem Pfad relativ zum Workspace-Ordner die Crate oder den ganzen Workspace; `get` legt erst dann an. */
function pickRust(rel: string, get: () => RustPlan): void {
  if (rel.startsWith('crates/')) {
    const crate = rel.slice('crates/'.length);
    const slash = crate.indexOf('/');
    if (slash > 0) get().crates.add(crate.slice(0, slash));
    return;
  }
  if (RUST_WORKSPACE_FILES.has(rel)) get().all = true;
}

/** Ordnet eine Datei unter rust/ zu; true = Datei gehört zu rust/ (auch rust/tools/). */
function rustFile(p: Plan, f: string): boolean {
  if (!f.startsWith(RUST_ROOT)) return false;
  const rel = f.slice(RUST_ROOT.length);
  if (rel.startsWith('tools/')) {
    pickRust(rel.slice('tools/'.length), () => rustToolsOf(p));
  } else if (RUST_INHERITED_FILES.has(rel)) {
    rustOf(p).all = true;
    rustToolsOf(p).all = true;
  } else {
    pickRust(rel, () => rustOf(p));
  }
  return true;
}

/** classify bildet aus den geänderten Dateien (Pfade mit /, relativ zur Repo-Wurzel) die Auswahl. */
export function classify(files: string[]): Plan {
  const p = emptyPlan();
  for (const f of files) {
    if (isDoc(f)) {
      p.docs = true;
      continue;
    }
    p.rules = true;
    if (path.posix.basename(f) === 'Taskfile.yml' || f.startsWith('.github/') || frontend(p, f) || rustFile(p, f)) continue;
    // testdata/ an der Wurzel liest seit RU41 nur noch Rust (cargo test)
    if (f.startsWith('testdata/')) continue;
    p.client ||= isClient(f);
  }
  return p;
}
