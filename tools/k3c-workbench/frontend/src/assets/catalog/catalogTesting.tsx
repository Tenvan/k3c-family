// Test-Helfer des Katalogs (B-837): warten auf eine Bedingung statt auf feste Runden oder Zeiten. Die Mocks laden ihre
// Fixtures per dynamischem Import über den Modul-Runner von Vitest; unter Last (parallele check:dev-Läufe) dauert das
// länger als flush() (acht Runden setTimeout 0), dann prüften die Tests vor dem ersten Render.
import { buttons, click, mount, wait } from '../testing';
import { CatalogPage } from './CatalogPage';

/** Höchste Wartezeit einer Bedingung; unter Last reichen Millisekunden, das Limit fängt nur echte Fehler. */
const LIMIT_MS = 5000;

/** Wartet, bis `cond` einen Wert liefert, und gibt ihn zurück; nach `ms` scheitert der Test mit `what`. */
export async function until<T>(what: string, cond: () => T | null | undefined | false, ms = LIMIT_MS): Promise<T> {
  const end = Date.now() + ms;
  for (;;) {
    const v = cond();
    if (v) return v;
    if (Date.now() > end) throw new Error(`nicht erreicht nach ${ms} ms: ${what}`);
    await wait(5);
  }
}

/** Katalog einhängen und warten, bis die Liste steht und die Prüfung durch ist. */
export async function mountCatalog(): Promise<HTMLElement> {
  const host = await mount(<CatalogPage />);
  await until('Liste geladen und geprüft', () => host.querySelector('[data-asset]') && !host.querySelector('[data-checking]'));
  return host;
}

/** Asset wählen und warten, bis die Vorschau das erste Bild zeigt (Spec und Frame geladen). */
export async function select(host: HTMLElement, id: string): Promise<void> {
  await click(host.querySelector(`[data-asset="${id}"]`)!);
  await until(`Vorschau von ${id}`, () => host.querySelector(`.as-stage img[alt^="${id} "]`) && host.querySelector('[data-part="count"]'));
}

/** Gezeigter Frame (0-basiert) und Anzahl laut Zähler des Frame-Players. */
function counter(host: ParentNode): [number, number] {
  const m = /(\d+)\/(\d+)/.exec(host.querySelector('[data-part="count"]')?.textContent ?? '');
  if (!m) throw new Error('Frame-Player fehlt');
  return [Number(m[1]) - 1, Number(m[2])];
}

/** Spielt ab, bis der Zähler `steps` Frames weiter steht (über den Rundenwechsel gezählt), und hält an. Ersetzt feste
 *  Wartezeiten: unter Last wartet das Abspielen auf ladende Frames, eine Runde dauert dann länger als 1000 / fps ms. */
export async function playFrames(host: HTMLElement, steps: number): Promise<void> {
  const [start, frames] = counter(host);
  let prev = start;
  let walked = 0;
  await click(buttons(host, 'Abspielen')[0]);
  await until(`${steps} Frames abgespielt`, () => {
    const [nr] = counter(host);
    walked += (nr - prev + frames) % frames;
    prev = nr;
    return walked >= steps;
  });
  await click(buttons(host, 'Anhalten')[0]);
}
