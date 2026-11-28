// Lizenzdatei LIZENZEN.txt (B-1121): Kopf aus `k3c-game licenses`, darunter alle gelinkten Rust-Crates aus
// `cargo metadata` samt den Lizenztexten aus den Paketen. Texte laufen als Bytes durch (nur ohne \r).

import { spawnSync } from 'node:child_process';
import { readdirSync, readFileSync } from 'node:fs';
import path from 'node:path';
import { notFound, resolveExe } from '../checkdiff/exe';

/** Metadata ist der Teil von `cargo metadata --format-version 1`, den die Lizenzliste braucht. */
export interface Metadata {
  packages: Pkg[];
  resolve: { nodes: Node[] };
}

export interface Pkg {
  id: string;
  name: string;
  version: string;
  license?: string | null;
  repository?: string | null;
  source?: string | null;
  manifest_path: string;
}

export interface Node {
  id: string;
  deps: { pkg: string; dep_kinds: { kind: string | null }[] }[];
}

export interface Text {
  name: string;
  body: Buffer;
}

export type Write = (chunk: string | Buffer) => void;

const compare = (a: string, b: string): number => (a < b ? -1 : a > b ? 1 : 0);
const byNameVersion = (a: Pkg, b: Pkg): number => compare(a.name, b.name) || compare(a.version, b.version);
const message = (e: unknown): string => (e instanceof Error ? e.message : String(e));

function normalDeps(m: Metadata): Map<string, string[]> {
  const deps = new Map<string, string[]>();
  for (const node of m.resolve?.nodes ?? []) {
    const normal = (node.deps ?? []).filter((d) => d.dep_kinds?.some((k) => k.kind == null));
    deps.set(node.id, [...(deps.get(node.id) ?? []), ...normal.map((d) => d.pkg)]);
  }
  return deps;
}

/**
 * linkedPackages liefert die fremden Pakete (mit Quelle, also keine Workspace-Mitglieder), die vom Paket root über
 * normale Abhängigkeiten erreichbar sind; Build- und Dev-Abhängigkeiten gelangen nicht in die EXE. Nach Name, Version.
 */
export function linkedPackages(m: Metadata, root: string): Pkg[] {
  const packages = m.packages ?? [];
  const byId = new Map(packages.map((p) => [p.id, p]));
  const start = packages.find((p) => p.name === root && p.source == null);
  if (!start) throw new Error(`paket ${JSON.stringify(root)} fehlt in cargo metadata`);
  const deps = normalDeps(m);
  const seen = new Set([start.id]);
  const queue = [start.id];
  const res: Pkg[] = [];
  for (let i = 0; i < queue.length; i++) {
    for (const dep of deps.get(queue[i]) ?? []) {
      if (seen.has(dep)) continue;
      seen.add(dep);
      queue.push(dep);
      const p = byId.get(dep);
      if (p && p.source != null) res.push(p);
    }
  }
  return res.sort(byNameVersion);
}

const PREFIXES = ['LICENSE', 'LICENCE', 'COPYING', 'UNLICENSE', 'NOTICE'];

/** isLicenseName meldet, ob ein Dateiname eine Lizenz- oder Hinweisdatei benennt (Groß- und Kleinschreibung egal). */
export function isLicenseName(name: string): boolean {
  const up = name.toUpperCase();
  return PREFIXES.some((prefix) => up.startsWith(prefix));
}

/** licenseFiles liest die Lizenz- und Hinweisdateien im Ordner dir (ohne Unterordner), nach Name sortiert. */
export function licenseFiles(dir: string): Text[] {
  return readdirSync(dir, { withFileTypes: true })
    .filter((e) => !e.isDirectory() && isLicenseName(e.name))
    .map((e) => e.name)
    .sort(compare)
    .map((name) => ({ name, body: Buffer.from(readFileSync(path.join(dir, name)).filter((byte) => byte !== 0x0d)) }));
}

/** spdxUrls nennt zu einem SPDX-Ausdruck („MIT OR Apache-2.0“, „Apache-2.0/MIT“) die Adressen der Standardtexte. */
export function spdxUrls(expr: string): string[] {
  return expr
    .split(/[ /()]+/)
    .filter((id) => id !== '' && id !== 'OR' && id !== 'AND' && id !== 'WITH')
    .map((id) => `https://spdx.org/licenses/${id}.html`);
}

function textsOf(p: Pkg, read: (dir: string) => Text[]): Text[] {
  let texts: Text[];
  try {
    texts = read(path.dirname(p.manifest_path));
  } catch (e) {
    throw new Error(`${p.name} ${p.version}: ${message(e)}`);
  }
  if (texts.length === 0 && !p.license) {
    throw new Error(`weder Lizenzdatei noch SPDX-Ausdruck im Crate ${p.name} ${p.version}`);
  }
  return texts;
}

function renderPackage(write: Write, p: Pkg, texts: Text[]): void {
  const spdx = p.license ?? '?';
  write(`\n\n==== ${p.name} ${p.version} (${spdx}) ====\n`);
  if (texts.length === 0) {
    write(`\nDas Crate enthält keine Lizenzdatei. Lizenz: ${spdx}\n`);
    if (p.repository != null) write(`Quelle: ${p.repository}\n`);
    for (const url of spdxUrls(spdx)) write(`Text: ${url}\n`);
  }
  for (const text of texts) {
    write(`\n-- ${text.name} --\n`);
    write(text.body);
  }
}

/**
 * render schreibt den Kopf und je Paket Name, Version, SPDX-Ausdruck und die Lizenztexte. Liegt dem Paket keine
 * Lizenzdatei bei (einige Crates veröffentlichen sie nicht), stehen SPDX-Ausdruck, Quelle und die Adresse der
 * Standardtexte da; fehlt auch der SPDX-Ausdruck, bricht der Aufruf ab.
 */
export function render(write: Write, head: string, pkgs: Pkg[], read: (dir: string) => Text[]): void {
  write(head);
  write(
    `\n\nRust-Crates im Spiel (${pkgs.length}), Lizenztexte aus den Paketen. Für die gelinkten Bibliotheken ist diese Liste maßgeblich. · Rust crates linked into the game; this list is authoritative for the linked libraries.\n`,
  );
  for (const p of pkgs) renderPackage(write, p, textsOf(p, read));
}

/** mergePackages vereinigt zwei Listen ohne doppelte Name-Version-Paare, sortiert nach Name, Version. */
export function mergePackages(a: Pkg[], b: Pkg[]): Pkg[] {
  const byKey = new Map<string, Pkg>();
  for (const p of [...a, ...b]) {
    const key = `${p.name} ${p.version}`;
    if (!byKey.has(key)) byKey.set(key, p);
  }
  return [...byKey.values()].sort(byNameVersion);
}

/** Startet ein Programm ohne Shell und liefert stdout; wirft mit Exit-Code und, wenn gewünscht, stderr. */
function output(label: string, file: string, args: string[], cwd?: string, withStderr = false): Buffer {
  const res = spawnSync(file, args, { cwd, maxBuffer: 256 * 1024 * 1024, windowsHide: true });
  if (res.error) throw new Error(`${label}: ${res.error.message}`);
  if (res.status !== 0) {
    const detail = withStderr ? `: ${res.stderr.toString('utf8').trim()}` : '';
    throw new Error(`${label}: exit status ${res.status ?? res.signal}${detail}`);
  }
  return res.stdout;
}

/** cargoMetadata ruft `cargo metadata` im Rust-Ordner auf (nur das Windows-Ziel, Cargo.lock bleibt unverändert). */
export function cargoMetadata(rustDir: string, target: string): Metadata {
  const cargo = resolveExe('cargo');
  if (!cargo) throw new Error(`cargo metadata: ${notFound('cargo')}`);
  const args = ['metadata', '--format-version', '1', '--locked', '--filter-platform', target];
  const out = output('cargo metadata', cargo, args, rustDir, true);
  try {
    return JSON.parse(out.toString('utf8')) as Metadata;
  } catch (e) {
    throw new Error(`cargo metadata: ${message(e)}`);
  }
}

/** gameExe macht aus der Angabe einen eindeutigen Pfad: ein bloßer Name kommt nie aus dem Arbeitsordner. */
function gameExe(exe: string): string {
  if (/[\\/]/.test(exe)) return path.resolve(exe);
  const found = resolveExe(exe.replace(/\.exe$/i, ''));
  if (!found) throw new Error(`${exe} licenses: ${notFound(exe)}`);
  return found;
}

export interface LizenzOptions {
  exe: string;
  rust: string;
  target: string;
}

/** lizenzen: der Kopf kommt von `k3c-game licenses` (Liste der Lizenz-Seite im Menü), darunter die Crates. */
export function lizenzen(options: LizenzOptions, write: Write): void {
  const head = output(`${options.exe} licenses`, gameExe(options.exe), ['licenses']);
  const meta = cargoMetadata(options.rust, options.target);
  // Spiel und Cheat-Fenster (RU56, B-1135) liegen im Paket; das Fenster bindet das `windows`-Crate
  const pkgs = mergePackages(linkedPackages(meta, 'k3c-game'), linkedPackages(meta, 'k3c-devwin'));
  render(write, head.toString('utf8').replaceAll('\r', '').replace(/\n+$/, ''), pkgs, licenseFiles);
}
