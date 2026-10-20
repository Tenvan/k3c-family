// Die GitHub-Pages-Seite (site/, B-183) ist rein statisch: kein Script, keine Anfrage an Server oder Fremddienste.
// Pages lädt nur site/ hoch (B-507): jedes eingebundene Bild und jeder lokale Link zeigt auf eine Datei in site/.
import { existsSync, readFileSync } from 'node:fs';
import { join, resolve } from 'node:path';
import { describe, expect, it } from 'vitest';

const ROOT = resolve(__dirname, '..');
const SITE = join(ROOT, 'site');
const html = readFileSync(join(SITE, 'index.html'), 'utf8');

/** Lokale Referenzen aus src/href/url(), ohne Anker, Fremdadressen und mailto. */
function localRefs(): string[] {
  const refs = [...html.matchAll(/(?:src|href)="([^"]+)"|url\(([^)]+)\)/g)].map((m) => (m[1] ?? m[2]).replace(/["']/g, ''));
  return refs.filter((ref) => !/^(?:#|https?:|mailto:|var\()/.test(ref));
}

describe('GitHub-Pages-Seite', () => {
  it('lädt kein Script und spricht keinen Server an', () => {
    expect(html).not.toMatch(/<script|\/api\/|["'(]\/?ws\b|fetch\(|new WebSocket|<form|<iframe/);
  });

  it('bindet nichts von fremden Adressen ein (Links sind erlaubt)', () => {
    const embeds = [...html.matchAll(/(?:src|url\()\s*["']?(https?:)?\/\//g)];
    const remoteLinkTags = [...html.matchAll(/<link[^>]+href="https?:/g)];
    expect([...embeds, ...remoteLinkTags]).toEqual([]);
  });

  it('der Deploy-Workflow lädt nur site/ hoch, ohne Node und ohne Build', () => {
    const workflow = readFileSync(join(ROOT, '.github/workflows/deploy-pages.yml'), 'utf8');
    expect(workflow).not.toMatch(/setup-node|setup-go|setup-task|npm |npx |task /);
    expect(workflow).toMatch(/path:\s*site\/\s*$/m);
  });

  it('jede lokale Referenz zeigt auf eine Datei in site/ (kein Spiel-Build unter app/)', () => {
    const refs = localRefs();
    expect(refs.some((ref) => ref.endsWith('.png'))).toBe(true);
    for (const ref of refs) {
      expect(ref.startsWith('/') || ref.includes('..'), ref).toBe(false);
      expect(existsSync(join(SITE, ref)), ref).toBe(true);
    }
  });
});
