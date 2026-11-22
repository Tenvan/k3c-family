// Aufträge an Claude von der Seite Spieltest (B-1224): kopieren oder als neue Code-Session öffnen (CopyPrompt).
// Die Workbench ruft keine KI; Claude liest per MCP (playtest_*) und schreibt zurück.

/** Auswerten: Claude liest die Session, bildet Befunde und prüft jeden gegen offene Tickets. */
export const evaluatePrompt = (id: string): string =>
  `Spieltest ${id} auswerten: mit playtest_get die Session lesen (Transkript, Marker, Fragebogen), bei Bedarf Bilder mit playtest_frame ansehen, `
  + 'Befunde bilden (Zitat, Zeitpunkt, Bild, Spielkontext, Vorschlag Typ, Prio, Projekt, Sprint), jeden Befund mit plan_list gegen offene Tickets prüfen '
  + '(Treffer als duplicate_of statt neuem Ticket) und mit playtest_findings_set speichern. Keine Tickets anlegen.';

/** Tickets anlegen: nur angehakte Befunde ohne Duplikat. */
export const createTicketsPrompt = (id: string): string =>
  `Spieltest ${id}: Tickets für die angehakten Befunde anlegen mit playtest_tickets_create (Projekt und Sprint wie am Befund; fehlen sie, bei mir nachfragen, `
  + 'fehlt der Sprint, zuerst plan_create kind sprint vorschlagen). Duplikate bekommen nur einen Hinweis im vorhandenen Ticket. Danach das Ergebnis melden.';
