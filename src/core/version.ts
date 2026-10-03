/** Versionen von Client und Server: Tag und Buildzeit (Landingpage, Lobby, Debug-Overlay). */

declare const __APP_VERSION__: string;
declare const __BUILD_TIME__: string;

export interface BuildInfo {
  /** Git-Tag bzw. `git describe`, sonst `dev` */
  version: string;
  /** Buildzeit als ISO 8601 (UTC); leer = unbekannt */
  built: string;
}

/** Eingebaut von `vite.config.ts` › `define`. */
export const CLIENT: BuildInfo = { version: __APP_VERSION__, built: __BUILD_TIME__ };

/** `v0.4.0 (03.10.2026, 14:38)`; ohne gültige Buildzeit nur die Version. `timeZone` nur für Tests. */
export function formatBuild(b: BuildInfo, timeZone?: string): string {
  const t = new Date(b.built);
  if (!b.built || Number.isNaN(t.getTime())) return b.version;
  const when = t.toLocaleString('de-DE', { day: '2-digit', month: '2-digit', year: 'numeric', hour: '2-digit', minute: '2-digit', timeZone });
  return `${b.version} (${when})`;
}

/** `Client … · Server …` (`sep` z. B. Zeilenumbruch für zwei Zeilen); `server` null = nicht erreichbar oder noch nicht geladen. */
export const versionLine = (client: BuildInfo, server: BuildInfo | null, sep = ' · ', timeZone?: string): string =>
  `Client ${formatBuild(client, timeZone)}${sep}Server ${server ? formatBuild(server, timeZone) : '–'}`;

/** `api/health` (relativ zur Seite) → Version und Buildzeit des Servers; kein Go-Server (z. B. GitHub Pages) → `null`, nie ein Fehler. */
export async function fetchServerBuild(fetchFn: typeof fetch = fetch, timeoutMs = 2000): Promise<BuildInfo | null> {
  try {
    const res = await fetchFn('api/health', { cache: 'no-store', signal: AbortSignal.timeout(timeoutMs) });
    if (!res.ok) return null;
    const body = (await res.json()) as { ok?: unknown; version?: unknown; built?: unknown };
    if (body.ok !== true) return null;
    return { version: typeof body.version === 'string' ? body.version : '?', built: typeof body.built === 'string' ? body.built : '' };
  } catch {
    return null;
  }
}
