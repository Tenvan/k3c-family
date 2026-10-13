# KB2 · SRV, CLI · Protokoll-Nachträge Kampf: Inselwechsel, Game Over, Händler, Client-Typen

- **Status:** geplant
- **Projekt:** KMP
- **Domäne:** SRV, CLI
- **Reife:** Entwurf
- **Tickets:** B-385, B-344, B-381, B-386, B-324
- **Start-Commit:** – (wird beim Aktivieren gesetzt: `git rev-parse --short origin/develop`)
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

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

## Offene Fragen

B-386 (🧑): Gilt das Netz-Budget Q08 (≤ 200 Byte je Tick und Client) für den gesamten Zustand oder nur für Ereignisse? Gemessen: `delta` Ø 913 B, `events` Ø 2,9 B. Blockiert die Freigabe von AC-03.

## Sessions

| Nr. | Datei | Typ | Agent | Status |
|---|---|---|---|---|
| KB2.1 | `KB2.1-island-switch-ereignis.md` | Umsetzung | autonom | offen |
| KB2.2 | `KB2.2-gameover-nicht-speichern.md` | Umsetzung | autonom | offen |
| KB2.3 | `KB2.3-haendler-protokoll.md` | Umsetzung | autonom | offen |
| KB2.4 | `KB2.4-client-typen-gegner.md` | Umsetzung | autonom | offen |
| KB2.5 | `KB2.5-review.md` | Review | autonom | offen |

Bei `Reife: Entwurf` genügen Stichpunkte mit den Kriterien in Klammern statt Session-Dateien. Vor dem Aktivieren
wird jede Session als eigene Datei nach `docs/vorlagen/session.md` geschrieben.

## Abnahme

Wird von der Review-Session (Doku-Sprint: letzte Session) ausgefüllt, höchstens fünf Zeilen: Datum, Kriterien
(Verweis auf Session-Ergebnisse), behobene Befunde, neue Tickets. Bis dahin `–`.
