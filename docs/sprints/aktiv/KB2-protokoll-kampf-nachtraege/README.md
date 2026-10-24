# KB2 · SRV, CLI · Protokoll-Nachträge Kampf: Inselwechsel, Game Over, Händler, Client-Typen

- **Status:** aktiv
- **Projekt:** KMP
- **Domäne:** SRV, CLI
- **Reife:** bereit
- **Tickets:** B-385, B-344, B-381, B-386, B-324, B-080
- **Start-Commit:** 5ff6ce7c
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** autonom, Claude, 2026-10-23, Revision 1

## Ausgangslage

Nach K2–K4 offen: `islandSwitch` entsteht im Raum nach `StepIsland` und erreicht keinen Client (B-385, `engine/room/actions.go`, `island_switch.go`), der Raum würde nach „Komplett verloren“ den letzten Spielstand überschreiben (B-344), `docs/protocol.md` und `src/model/types.ts` kennen `merchant.raid` und Händler-HP nicht (B-381), die Client-Typen der Gegner- und Wellendaten weichen von den JSON-Dateien ab (B-324). B-386 fragt, ob das Netz-Budget Q08 für alles oder nur für Ereignisse gilt.

## Ziel

Server, Protokoll und Client-Typen des Kampfs sind konsistent: Inselwechsel kommt beim Client an, Game Over zerstört keinen Spielstand, das Protokoll beschreibt den Händler-Überfall, Client-Typen passen zu den Daten. Am Ende sichtbar: `task check` und `task check:go` grün, Protokoll-Beispiele ergänzt.

## Beteiligte und Zielgruppen

Spielende (kein Datenverlust nach Game Over), Agent in SRV und CLI, 🧑 entscheidet B-386.

## Anforderungen

B-385, B-344, B-381, B-386, B-324 › Anforderungen (Specs noch Entwurf).

## Nicht-Ziele

Neue Kampf-Ereignisse; Darstellung im Client (K5); Änderungen der Simulation (KB1).

## Regeln und Einschränkungen

Protokoll-Änderungen (KB2.1, KB2.3) bekommen je eine eigene Session, die nur Protokoll und beide Enden anpasst; Beispiele unter `testdata/protocol/`. Spielstand-Format bleibt (Version v5). Nach KB1, da KB1.1 Ereignisse der Sim ändert.

## Beispiele

Alle wechseln auf die nächste Insel → Clients erhalten `islandSwitch` im selben Takt. Insel „Komplett verloren“ → danach kein Autosave, Spielstand bleibt wie vor der letzten Nacht.

## Ausnahme- und Fehlerfälle

Spieler verlässt den Raum nach Game Over → kein Speichern über den letzten Stand (🚫 im Log). Client kennt ein unbekanntes Ereignis → ignorieren mit Warnung 🤒.

## Akzeptanzkriterien

- **AC-01** `islandSwitch` erreicht beim Inselwechsel jeden Client, mit Test in `engine/room/` (B-385).
- **AC-02** Der Raum speichert nach „Komplett verloren“ nicht mehr (B-344).
- **AC-03** `docs/protocol.md`, `src/model/types.ts` und das Beispiel beschreiben `merchant.raid` und die Händler-HP; die Entscheidung zu Q08 (B-386) steht im Protokoll (B-381, B-386).
- **AC-04** `src/model/data.ts` kennt alle Felder aus `enemies.json` und `waves.json` ohne `as unknown as` (B-324).
- **AC-05** Go-Tests belegen: Eine Dev-Aktion ist im nächsten `delta` sichtbar, der Gradwechsel wirkt ab der nächsten Welle und wird ohne Dev-Mode abgelehnt (B-080/AC-01, B-080/AC-02). KB2.6 beim Bereitmachen vor das Review ordnen.

## Offene Fragen

Vorentschieden (autonom, Claude, 2026-10-23), Bestätigung durch 🧑 über KR6:

- **B-386 (Q08):** Variante B. Das Budget ≤ 200 Byte je Tick und Client gilt für die Ereignisse; der Zustand (`snap`, `delta`) wird nur gemessen (wie K4.2). Steht nach KB2.3 in `docs/protocol.md` und `docs/fragenkatalog.md` als Default.
- **B-385:** Die Ereignisse des letzten Ticks der alten Insel gehen mit einem letzten `delta` an jedes Gerät, bevor `level` und `snap` der neuen Insel folgen.
- **B-344:** Nach `Island.Over` speichert der Raum nicht mehr; er bleibt offen, bis alle gegangen sind.
- KB2.6 läuft vor K5.3 (K5.3 braucht die Dev-Aktion `grade` über WebSocket und den Grad im Zustand).

## Sessions

| Nr. | Datei | Typ | Agent | Status |
|---|---|---|---|---|
| KB2.1 | `KB2.1-island-switch-ereignis.md` | Umsetzung | autonom | in Arbeit |
| KB2.2 | `KB2.2-gameover-nicht-speichern.md` | Umsetzung | autonom | offen |
| KB2.3 | `KB2.3-haendler-protokoll.md` | Umsetzung | autonom | offen |
| KB2.4 | `KB2.4-client-typen-gegner.md` | Umsetzung | autonom | offen |
| KB2.5 | `KB2.5-review.md` | Review | autonom | offen |
| KB2.6 | `KB2.6-dev-aktion-grad-nachweis.md` | Umsetzung | autonom | fertig |

Bei `Reife: Entwurf` genügen Stichpunkte mit den Kriterien in Klammern statt Session-Dateien. Vor dem Aktivieren
wird jede Session als eigene Datei nach `docs/vorlagen/session.md` geschrieben.

## Abnahme

Wird von der Review-Session (Doku-Sprint: letzte Session) ausgefüllt, höchstens fünf Zeilen: Datum, Kriterien
(Verweis auf Session-Ergebnisse), behobene Befunde, neue Tickets. Bis dahin `–`.
