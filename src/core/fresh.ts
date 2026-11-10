/**
 * Frische Fassung des Clients (B-767, BD11.6): Das Spiel bleibt auf der Xbox tagelang offen und kann eine alte Fassung des Clients
 * festhalten (Cache, Heimnetz-Server mit neuem Image). Meldet der Server eine andere Version als der Client, lädt das Spiel
 * einmal je Version mit `?v=<Version>` neu (Hash der Adresse bleibt erhalten); die Werkzeug-Panels bekommen dieselbe Version als
 * Parameter, damit kein Zwischenspeicher eine alte Seite ausliefert. Die Entscheidung ist rein, `reloadIfStale` führt sie aus.
 */
import type { BuildInfo } from './version';

/** Sitzungsmarke: für diese Server-Version wurde schon frisch geladen (verhindert eine Schleife bei hartnäckigem Cache) */
export const FRESH_KEY = 'k3c-fresh';

/** Muss die Landingpage frisch laden? Nur wenn der Server erreichbar ist, die Version abweicht und für sie noch kein Versuch lief. */
export function shouldRefresh(client: BuildInfo, server: BuildInfo | null, tried: string | null): boolean {
  return server !== null && server.version !== client.version && tried !== server.version;
}

/** Adresse der Landingpage mit `v=<version>`; Abfrage und Hash (geöffnete Seite) bleiben, ein altes `v` wird ersetzt */
export function freshHref(pathname: string, search: string, hash: string, version: string): string {
  const p = new URLSearchParams(search);
  p.set('v', version);
  return `${pathname}?${p.toString()}${hash}`;
}

/** Seite für das iframe mit `v=<version>`; ein vorhandenes `v` bleibt (z. B. von Hand gesetzt) */
/**
 * Lädt das Spiel frisch, wenn der Server eine andere Version meldet und für diese noch kein Versuch lief (Sitzungsmarke gegen
 * Schleifen bei hartnäckigem Cache). Ohne Sitzungsspeicher bleibt es bei der Anzeige der Abweichung. `true` = Neuladen ausgelöst.
 */
export function reloadIfStale(client: BuildInfo, server: BuildInfo | null): boolean {
  let tried: string | null = null;
  try {
    tried = sessionStorage.getItem(FRESH_KEY);
  } catch {
    return false;
  }
  if (!server || !shouldRefresh(client, server, tried)) return false;
  try {
    sessionStorage.setItem(FRESH_KEY, server.version);
  } catch {
    return false;
  }
  location.replace(freshHref(location.pathname, location.search, location.hash, server.version));
  return true;
}

export function withVersion(href: string, version: string): string {
  const hashAt = href.indexOf('#');
  const hash = hashAt >= 0 ? href.slice(hashAt) : '';
  const base = hashAt >= 0 ? href.slice(0, hashAt) : href;
  const q = base.indexOf('?');
  const path = q >= 0 ? base.slice(0, q) : base;
  const p = new URLSearchParams(q >= 0 ? base.slice(q + 1) : '');
  if (!p.has('v')) p.set('v', version);
  return `${path}?${p.toString()}${hash}`;
}
