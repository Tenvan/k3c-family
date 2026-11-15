# B-923 · Gebäudeplaner samt MCP-Tools aus dem alten Studio fehlt in der Workbench

- **Domäne:** DEV
- **Typ:** Problem
- **Prio:** hoch
- **Umgebung:** ?
- **Status:** erledigt
- **Sprint:** FB3
- **Projekt:** WZG
- **Erstellt:** 2026-11-15
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** autonom, Claude
- **Duplikat von:** –
- **Ersetzt:** –

## Ausgangslage

Hinweis 🧑 2026-11-15: Mit dem Wegfall von `tools/k3c-studio` (WB1) ging der Gebäudeplaner verloren: Backend `app_plan*.go`, `app_planer*.go` samt Tests und die zugehörigen MCP-Tools. Im Git-Verlauf liegt der alte Stand (u. a. #392, #495).

## Ziel

Der Gebäudeplaner ist als Seite im Bereich Assets der Workbench zurück, mit seinen MCP-Tools im Server `k3c-workbench`.

## Beteiligte und Zielgruppen

Wer spielt, entwickelt, betreibt oder entscheidet (🧑)? Keine Verantwortlichen erfinden.

## Anforderungen

- Funktionsumfang wie im alten Studio (Planen, Regeln, Zählung, Speichern, Vorschau).
- Oberfläche nach Atomic Design über `@webkit`.
- MCP-Tools nach dem Namensschema `<bereich>_<verb>` mit „Nutze es bei … Statt …“.

## Nicht-Ziele

Was ausdrücklich nicht dazugehört, mit Ticket-Nummer, falls es später kommt.

## Regeln und Einschränkungen

- Entscheidung 005: Assets nur über `assets_*`; der Planer schreibt nur, was er im alten Studio schrieb.
- **Größe:** M

## Beispiele

Typische Situation → erwartetes Ergebnis. Passt nichts: `nicht relevant` mit Grund.

## Ausnahme- und Fehlerfälle

Ungültige oder seltene Situation → gewolltes Verhalten. Passt nichts: `nicht relevant` mit Grund.

## Akzeptanzkriterien

- **AC-01** Die Workbench hat eine Seite Gebäudeplaner mit dem Umfang des alten Studios.
- **AC-02** Die MCP-Tools des Planers sind im Katalog und per Roundtrip-Test geprüft.
- **AC-03** Die alten Tests sind übernommen und grün (`check:dev`).

## Offene Fragen

keine

## Notizen

Links, Messwerte, verworfene Ansätze. Darf leer bleiben (`–`).
