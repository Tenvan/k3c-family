// Zustand der Planungsseite nach `planning:changed` (B-909): nur die genannten IDs nachladen und in die Listen
// einarbeiten. Reine Logik ohne React, damit Vitest sie prüft.
import type { PlanItem, PlanningChanged, PlanningData, PlanSession, PlanTicket } from '../api';

/** Gesammelte Nachlade-Wünsche seit dem letzten Lauf; `all` = ganze Liste neu (erster Aufruf, ids fehlen, zu viele). */
export interface Pending {
  all: boolean;
  ids: ReadonlySet<string>;
}

/** Höchste Zahl IDs je Nachladen (Go: höchstens 100 je Aufruf); mehr Änderungen laden die ganze Liste. */
export const MAX_IDS = 100;

export const FIRST: Pending = { all: true, ids: new Set() };
export const NOTHING: Pending = { all: false, ids: new Set() };

/** Ereignis vermerken: ohne `ids` (null, leer) ist die Lage unklar, dann alles neu. */
export function collect(p: Pending, payload: PlanningChanged | null | undefined): Pending {
  const ids = payload?.ids;
  if (p.all || !ids?.length) return FIRST;
  const next = new Set([...p.ids, ...ids]);
  return next.size > MAX_IDS ? FIRST : { all: false, ids: next };
}

/** Felder einer Ticket-Datei: `# B-1 · Titel` und die Zeilen `- **Feld:** Wert`. */
export function parseTicket(text: string): Record<string, string> {
  const out: Record<string, string> = {};
  const h1 = /^#\s+\S+\s+·\s+(.+)$/m.exec(text);
  if (h1) out.title = h1[1].trim();
  for (const m of text.matchAll(/^- \*\*([^:*]+):\*\*[ \t]*(.*)$/gm)) {
    const key = m[1].trim();
    if (!(key in out)) out[key] = m[2].trim();
  }
  return out;
}

const first = (v: string) => v.split(/\s+/)[0] ?? v;

/** Ticket mit den Feldern aus seiner Datei; Felder, die dort fehlen, bleiben. */
export function patchTicket(t: PlanTicket, text: string): PlanTicket {
  const f = parseTicket(text);
  const pick = (key: string, cur: string, norm: (v: string) => string = (v) => v) => (f[key] !== undefined && f[key] !== '' ? norm(f[key]) : cur);
  return {
    ...t, title: f.title ?? t.title, domain: pick('Domäne', t.domain), typ: pick('Typ', t.typ), prio: pick('Prio', t.prio),
    env: pick('Umgebung', t.env), status: pick('Status', t.status), sprint: pick('Sprint', t.sprint, first),
    spec: pick('Spec', t.spec), project: pick('Projekt', t.project ?? '', first),
  };
}

/** Hat die Session-Datei andere Kopffelder (Umgebung, Domäne, Abhängig von) als der Listeneintrag? Dann ist die Liste veraltet. */
export function sessionHeadChanged(x: PlanSession, text: string): boolean {
  const f = parseTicket(text);
  if (f['Umgebung'] === undefined && f['Domäne'] === undefined) return false; // ohne Kopf lässt sich nichts vergleichen
  const deps = (f['Abhängig von'] ?? '').match(/[A-Z]+\d+\.\d+/g) ?? [];
  const same = (a: string[], b: string[]) => a.length === b.length && a.every((v, i) => v === b[i]);
  return (f['Umgebung'] ?? '') !== (x.env ?? '') || (f['Domäne'] ?? '') !== (x.domain ?? '') || !same(deps, x.deps ?? []);
}

export interface Applied {
  data: PlanningData;
  /** Fließtexte der geladenen Sessions (ID → Markdown, leer ohne Datei). */
  texts: Record<string, string>;
  /** Struktur geändert (neuer oder weggefallener Sprint, Session, Eintrag; Sprint oder Projekt berührt): Liste neu laden. */
  reload: boolean;
}

/** Nachgeladene Dokumente einarbeiten: Tickets werden ersetzt oder entfernt, Session-Texte gemerkt. Sprint und Projekt
 *  tragen Tabellen und Reihenfolge, neue oder verschwundene Einträge ändern die Listen: beides lädt die Liste neu. */
export function applyItems(data: PlanningData, items: PlanItem[]): Applied {
  const texts: Record<string, string> = {};
  let tickets = data.tickets;
  let reload = false;
  for (const it of items) {
    if (it.kind === 'session' && !it.error) texts[it.id] = it.text;
    const step = applyItem(data, tickets, it);
    tickets = step.tickets;
    reload ||= step.reload;
  }
  return { data: tickets === data.tickets ? data : { ...data, tickets }, texts, reload };
}

/** Ein nachgeladenes Dokument: neue Ticket-Liste und ob die Liste neu zu laden ist. */
function applyItem(data: PlanningData, tickets: PlanTicket[], it: PlanItem): { tickets: PlanTicket[]; reload: boolean } {
  const known = tickets.some((t) => t.nr === it.id);
  const session = data.sprints.some((s) => s.sessions.some((x) => x.nr === it.id));
  if (it.error || (it.kind === 'ticket' && it.path.includes('/archiv/'))) { // archiviert zählt wie entfernt
    return known ? { tickets: tickets.filter((t) => t.nr !== it.id), reload: false } : { tickets, reload: session };
  }
  if (it.kind === 'ticket' && known) {
    const old = tickets.find((t) => t.nr === it.id)!;
    const next = patchTicket(old, it.text);
    // Sprintwechsel: PlanSprint.tickets zieht nach
    return { tickets: tickets.map((t) => (t.nr === it.id ? next : t)), reload: next.sprint !== first(old.sprint) };
  }
  if (it.kind === 'session' && session) {
    const x = data.sprints.flatMap((sp) => sp.sessions).find((y) => y.nr === it.id)!;
    return { tickets, reload: !x.wt && sessionHeadChanged(x, it.text) };
  }
  return { tickets, reload: true }; // Sprint, Projekt, neues Ticket oder neue Session
}
