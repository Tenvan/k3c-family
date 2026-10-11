# B-402 · Die Planungsseite zeigt in den Session-Zeilen den Titel der Session

- **Domäne:** DEV
- **Typ:** Idee
- **Prio:** mittel
- **Umgebung:** offline
- **Status:** eingeplant
- **Sprint:** WZ2
- **Projekt:** WZG
- **Erstellt:** 2026-10-11
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** 2026-10-11, 🧑 im Chat (Sprint WZ2, Revision 2)

## Ausgangslage

Wunsch von 🧑 (2026-10-11, mit Bild): In der Sprint-Karte der Planungsseite zeigen die Session-Zeilen nur `Umsetzung · autonom` statt des Titels. `SessionRow` in `tools/k3c-dev/frontend/src/planning/SprintCard.tsx` zeigt `x.titel`, sonst `Typ · Agent`; `ParseSprint` in `tools/k3c-dev/internal/planning/planning.go` füllt `Titel` nur für Stichpunkt-Entwürfe (`bullet`), nicht für Sessions mit Datei (Zeile der Tabelle `Nr | Datei | Typ | Agent | Status`). Der Titel steht in der ersten Zeile der Session-Datei (`# WZ2.1 · Titel`).

## Ziel

Jede Session-Zeile der Planungsseite zeigt den Titel der Session aus ihrer Datei; Typ und Agent bleiben als Zusatz sichtbar.

## Beteiligte und Zielgruppen

Wer spielt, entwickelt, betreibt oder entscheidet (🧑)? Keine Verantwortlichen erfinden.

## Anforderungen

- Der Titel kommt aus der Überschrift der Session-Datei (Teil nach `ID · `) und wird in `Session.Titel` geliefert (`plan_list`, `plan_get` und Planungsseite zeigen dasselbe).
- Stichpunkt-Entwürfe behalten ihren Titel aus dem Stichpunkt.
- Fehlt die Datei oder die Überschrift, bleibt es bei `Typ · Agent`.
- Lange Titel werden abgeschnitten, der volle Titel steht im Tooltip.

## Nicht-Ziele

Was ausdrücklich nicht dazugehört, mit Ticket-Nummer, falls es später kommt.

## Regeln und Einschränkungen

Regeln aus `CLAUDE.md`, Entscheidungen (`docs/decisions/`), Domäne, Komplexitäts-Budget, Verträge (Protokoll, Spielstand).

## Beispiele

Typische Situation → erwartetes Ergebnis. Passt nichts: `nicht relevant` mit Grund.

## Ausnahme- und Fehlerfälle

Ungültige oder seltene Situation → gewolltes Verhalten. Passt nichts: `nicht relevant` mit Grund.

## Akzeptanzkriterien

- **AC-01** Go-Test: `ParseSprint` bzw. das Laden eines Sprints liefert für eine Session mit Datei den Titel aus der Überschrift der Datei.
- **AC-02** Komponententest: `SessionRow` zeigt den Titel; ohne Titel `Typ · Agent`.

## Offene Fragen

–  (Entschieden von 🧑 am 2026-10-11: nur Session-Titel in den Session-Zeilen, keine Ticket-Titel in der Zeile `Tickets: …`.)

## Notizen

Aufgenommen in Sprint WZ2 (Revision 2, 🧑 2026-10-11), Session WZ2.7.
