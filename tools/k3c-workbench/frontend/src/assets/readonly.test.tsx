// @vitest-environment happy-dom
import { afterEach, beforeEach, describe, expect, it, vi, type MockInstance } from 'vitest';
import { Tabs } from '@webkit/organisms';
import { backend } from '../api';
import { AssetsArea } from './AssetsArea';
import { click, mount, unmount } from './testing';

// WB1/AC-02 (005 › 5, 7): Der Bereich Assets hat kein schreibendes Control. Jede Seite (außer review/, RV1) wird mit
// allen Buttons durchgeklickt; dabei darf nur die Lese-Allowlist aufgerufen werden. Neue Seiten in PAGES eintragen
// (Wert: hat die Seite Buttons?), neue Lesemethoden in READ.

const PAGES: Record<string, boolean> = { katalog: true, styleguide: false, sound: true, quellen: true };

/** Lesen, Ansehen, Kopieren und „In Claude öffnen“ (B-701 D-12); alles andere gilt als schreibend. */
const READ = new Set([
  'mock', 'on', 'info', 'openUrl', 'openInClaude',
  'assetsList', 'assetsGet', 'assetsStyle', 'assetsPrompts', 'assetsReview', 'assetsFrame', 'assetsPixelDiff',
  'studioSounds', 'studioSoundData', 'studioSources', 'studioGameOpen',
]);

/** Erlaubte Eingabefelder: nur Filter und Suche. */
const INPUTS = new Set(['Filter']);

let spies: [string, MockInstance][] = [];

beforeEach(() => {
  const b = backend as unknown as Record<string, unknown>;
  spies = Object.keys(b)
    .filter((k) => typeof b[k] === 'function' && !READ.has(k))
    .map((k) => [k, vi.spyOn(b as Record<string, () => unknown>, k).mockImplementation(() => Promise.reject(new Error(`schreibend: ${k}`)))]);
  Object.defineProperty(navigator, 'clipboard', { value: { writeText: vi.fn(async () => undefined) }, configurable: true });
  vi.spyOn(window, 'open').mockImplementation(() => null);
});

afterEach(() => {
  unmount();
  vi.restoreAllMocks();
});

/** Klickt jeden sichtbaren Button, auch die, die erst nach einem Klick erscheinen (höchstens drei Runden). */
async function clickAll(host: HTMLElement) {
  const seen = new Set<Element>();
  for (let round = 0; round < 3; round++) {
    const fresh = [...host.querySelectorAll('button, [role="button"], [data-asset]')].filter((b) => !seen.has(b));
    if (!fresh.length) return seen.size;
    for (const b of fresh) {
      seen.add(b);
      if (b.isConnected) await click(b);
    }
  }
  return seen.size;
}

describe('Bereich Assets ohne schreibende Controls (WB1/AC-02)', () => {
  for (const [page, hasButtons] of Object.entries(PAGES)) {
    it(`${page}: alle Buttons rufen nur Lesemethoden`, async () => {
      const host = await mount(<Tabs.Root value={page}><AssetsArea /></Tabs.Root>);
      const clicked = await clickAll(host);
      if (hasButtons) expect(clicked, `${page}: Buttons gefunden`).toBeGreaterThan(0);
      for (const [name, spy] of spies) expect(spy, name).not.toHaveBeenCalled();
    });

    it(`${page}: Eingabefelder nur zum Filtern`, async () => {
      const host = await mount(<Tabs.Root value={page}><AssetsArea /></Tabs.Root>);
      await clickAll(host);
      const fields = [...host.querySelectorAll('input, textarea, select')].filter(
        (f) => f.getAttribute('aria-hidden') !== 'true' && (f as HTMLInputElement).type !== 'hidden' && (f as HTMLElement).style.display !== 'none',
      );
      for (const f of fields) expect(INPUTS.has(f.getAttribute('aria-label') ?? ''), f.outerHTML.slice(0, 120)).toBe(true);
    });
  }
});
