// Die GitHub-Pages-Seite (site/, B-183) ist rein statisch: kein Script, keine Anfrage an Server oder Fremddienste,
// und jedes eingebundene Bild existiert (app/ = Spiel-Build, Bilder aus public/).
import { existsSync, readFileSync } from 'node:fs';
import { join, resolve } from 'node:path';
import { describe, expect, it } from 'vitest';

const ROOT = resolve(__dirname, '..');
const html = readFileSync(join(ROOT, 'site/index.html'), 'utf8');

describe('GitHub-Pages-Seite', () => {
  it('lädt kein Script und spricht keinen Server an', () => {
    expect(html).not.toMatch(/<script|\/api\/|["'(]\/?ws\b|fetch\(|new WebSocket|<form|<iframe/);
  });

  it('bindet nichts von fremden Adressen ein (Links sind erlaubt)', () => {
    const embeds = [...html.matchAll(/(?:src|url\()\s*["']?(https?:)?\/\//g)];
    const remoteLinkTags = [...html.matchAll(/<link[^>]+href="https?:/g)];
    expect([...embeds, ...remoteLinkTags]).toEqual([]);
  });

  it('jede Datei unter app/ gibt es im Build', () => {
    const refs = [...html.matchAll(/app\/([\w./-]+\.(?:png|html))/g)].map((m) => m[1]);
    expect(refs.length).toBeGreaterThan(0);
    for (const ref of refs) {
      const source = ref.endsWith('.html') ? ref : join('public', ref);
      expect(existsSync(join(ROOT, source)), ref).toBe(true);
    }
  });
});
