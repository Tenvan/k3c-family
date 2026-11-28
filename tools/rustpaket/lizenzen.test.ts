// Lizenzliste (aus lizenzen_test.go), mit den Fixtures unter testdata/.

import { fileURLToPath } from 'node:url';
import { describe, expect, it } from 'vitest';
import { licenseFiles, linkedPackages, mergePackages, render, spdxUrls, type Metadata, type Pkg } from './lizenzen';

const fixture = (name: string) => fileURLToPath(new URL(`testdata/${name}`, import.meta.url));
const ok = `${fixture('crate-ok')}/Cargo.toml`;
const empty = `${fixture('crate-leer')}/Cargo.toml`;
const reg = 'registry+https://github.com/rust-lang/crates.io-index';

const pkg = (id: string, name: string, version: string, extra: Partial<Pkg> = {}): Pkg => ({
  id,
  name,
  version,
  license: 'MIT',
  source: reg,
  manifest_path: ok,
  ...extra,
});

const dep = (id: string, kind: string | null = null) => ({ pkg: id, dep_kinds: [{ kind }] });

// k3c-game -> a (normal) -> b; k3c-game -> k3c-room (Workspace, ohne Quelle) -> c; d nur als Build-Abhängigkeit.
function testMeta(): Metadata {
  return {
    packages: [
      { id: 'game', name: 'k3c-game', version: '', manifest_path: '' },
      { id: 'room', name: 'k3c-room', version: '', manifest_path: '' },
      pkg('a', 'a', '1.0.0'),
      pkg('b2', 'b', '2.0.0', { license: 'MIT OR Apache-2.0' }),
      pkg('b1', 'b', '1.0.0'),
      pkg('c', 'c', '0.1.0', { license: 'Apache-2.0/MIT', manifest_path: empty, repository: 'https://example.org/c' }),
      pkg('d', 'd', '3.0.0'),
    ],
    resolve: {
      nodes: [
        { id: 'game', deps: [dep('a'), dep('room'), dep('d', 'build')] },
        { id: 'a', deps: [dep('b1'), dep('b2')] },
        { id: 'room', deps: [dep('c')] },
      ],
    },
  };
}

const names = (pkgs: Pkg[]) => pkgs.map((p) => `${p.name} ${p.version}`).join(',');

function rendered(head: string, pkgs: Pkg[]): string {
  const chunks: Buffer[] = [];
  render((chunk) => void chunks.push(Buffer.from(chunk)), head, pkgs, licenseFiles);
  return Buffer.concat(chunks).toString('utf8');
}

describe('lizenzen', () => {
  it('nimmt nur gelinkte fremde Pakete, nach Name und Version', () => {
    // Workspace (k3c-room) und Build-Abhängigkeit (d) entfallen, zwei Versionen von b bleiben
    expect(names(linkedPackages(testMeta(), 'k3c-game'))).toBe('a 1.0.0,b 1.0.0,b 2.0.0,c 0.1.0');
    expect(() => linkedPackages(testMeta(), 'gibt-es-nicht')).toThrow('gibt-es-nicht');
  });

  it('findet nur Lizenzdateien', () => {
    expect(licenseFiles(fixture('crate-ok')).map((t) => t.name)).toEqual(['LICENSE-MIT', 'NOTICE']);
    expect(licenseFiles(fixture('crate-leer'))).toEqual([]); // README.md ist keine Lizenzdatei
  });

  it('rendert mit Text und ohne Lizenzdatei, ohne \\r', () => {
    const out = rendered('KOPF', linkedPackages(testMeta(), 'k3c-game'));
    for (const want of [
      'KOPF', 'Rust-Crates im Spiel (4)',
      '==== a 1.0.0 (MIT) ====', 'MIT-Testtext (Fixture)', 'Hinweis-Testtext (Fixture)',
      '==== c 0.1.0 (Apache-2.0/MIT) ====', 'Quelle: https://example.org/c',
      'Text: https://spdx.org/licenses/Apache-2.0.html', 'Text: https://spdx.org/licenses/MIT.html',
    ]) {
      expect(out).toContain(want);
    }
    expect(out).not.toContain('\r');
  });

  it('bricht ohne Datei und ohne SPDX-Ausdruck mit Crate-Namen ab', () => {
    const p = pkg('x', 'x', '1.0.0', { license: null, manifest_path: empty });
    expect(() => rendered('', [p])).toThrow('x 1.0.0');
  });

  it('nennt die Adressen zum SPDX-Ausdruck', () => {
    expect(spdxUrls('(MIT OR Apache-2.0) AND BSD-3-Clause')).toEqual([
      'https://spdx.org/licenses/MIT.html',
      'https://spdx.org/licenses/Apache-2.0.html',
      'https://spdx.org/licenses/BSD-3-Clause.html',
    ]);
  });

  it('vereinigt zwei Listen ohne Doppelte', () => {
    const a = [pkg('1', 'b', '1'), pkg('2', 'c', '1')];
    const b = [pkg('3', 'a', '2'), pkg('4', 'b', '1')];
    expect(names(mergePackages(a, b))).toBe('a 2,b 1,c 1');
  });
});
