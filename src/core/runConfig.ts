/**
 * Start-Parameter des Clients an einer Stelle (B-768, BD11.3): Auswahl im Menü vor URL vor Standardwerten.
 * Kein Modul außer diesem liest `location.search`; alle fragen `runSearch()` (die wirksame Abfrage als Text, damit die reinen
 * Parser wie `parseStartParams` oder `debugEnabled` unverändert bleiben) oder `runParam(key)`.
 *
 * - Beim Laden gilt die URL der Seite (Deep-Link für Tests, CI und Bots, einmal gelesen).
 * - `setRun(params)` ersetzt alles durch genau diese Werte (ein Menü-Eintrag startet ein Szenario, ohne dass etwas von der URL
 *   mitschwingt); `patchRun(params)` ändert einzelne Werte (`null` entfernt), z. B. das Dev-Overlay.
 * - Ein Szenario startet damit im selben Dokument, ohne Neuladen der Seite.
 */

export type RunParams = Readonly<Record<string, string | null>>;

const urlSearch = (): string => (typeof location === 'undefined' ? '' : location.search);

let params = new URLSearchParams(urlSearch());

const asSearch = (p: URLSearchParams): string => {
  const text = p.toString();
  return text ? `?${text}` : '';
};

/** Die wirksame Abfrage, z. B. `?dev=1&autostart=1`; leer ohne Parameter */
export const runSearch = (): string => asSearch(params);

/** Wert eines Parameters der wirksamen Abfrage, `null` wenn er fehlt */
export const runParam = (key: string): string | null => params.get(key);

function apply(target: URLSearchParams, values: RunParams): void {
  for (const [key, value] of Object.entries(values)) {
    if (value === null) target.delete(key);
    else target.set(key, value);
  }
}

/** Ersetzt die ganze Konfiguration durch genau diese Werte (Eintrag mit `null` zählt als fehlend) */
export function setRun(values: RunParams): void {
  params = new URLSearchParams();
  apply(params, values);
}

/** Ändert einzelne Werte, der Rest bleibt; `null` entfernt den Parameter */
export function patchRun(values: RunParams): void {
  apply(params, values);
}

/** Zurück auf die URL der Seite (Tests, Neuladen der Konfiguration) */
export function resetRun(): void {
  params = new URLSearchParams(urlSearch());
}
