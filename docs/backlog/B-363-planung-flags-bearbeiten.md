# B-363 · Die Planungsseite ändert Prio, Umgebung, Agent und Projekt direkt im Detail

- **Domäne:** SRV
- **Typ:** Idee
- **Prio:** mittel
- **Umgebung:** offline
- **Status:** offen
- **Sprint:** –
- **Projekt:** WZG
- **Erstellt:** 2026-10-08
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

Wunsch von 🧑 (2026-10-08, beim Umzug in PJ3): Die Flags `autonom`/`Mensch`, `offline`/`live` und die Prio sollen in der Detailansicht der Planungsseite (`tools/k3c-dev/frontend/src/planning/`) bearbeitbar sein. Heute: `SessionDetail` in `Backlog.tsx` stellt `Agent` und `Umgebung` per `FieldMenu` um, aber nur für offene Sessions (`pickable`). Tickets haben keine Bearbeitung für `Prio`, `Umgebung` und `Projekt`. Seit PJ2 ordnet die Ticket-Prio die Tickets innerhalb eines Projekts (die Sprint-Prio entfällt, B-361).

## Ziel

Im Detail eines Tickets lassen sich Prio, Umgebung und Projekt ändern, im Detail einer Session Agent und Umgebung; die Änderung läuft wie der Rang über `plan_set` und zieht Index und Tabellen nach.

## Beteiligte und Zielgruppen

Wer spielt, entwickelt, betreibt oder entscheidet (🧑)? Keine Verantwortlichen erfinden.

## Anforderungen

- Was das Ergebnis können muss, auch Qualität (deterministisch, 2+ Spieler, Leistung).

## Nicht-Ziele

Was ausdrücklich nicht dazugehört, mit Ticket-Nummer, falls es später kommt.

## Regeln und Einschränkungen

Regeln aus `CLAUDE.md`, Entscheidungen (`docs/decisions/`), Domäne, Komplexitäts-Budget, Verträge (Protokoll, Spielstand).

## Beispiele

Typische Situation → erwartetes Ergebnis. Passt nichts: `nicht relevant` mit Grund.

## Ausnahme- und Fehlerfälle

Ungültige oder seltene Situation → gewolltes Verhalten. Passt nichts: `nicht relevant` mit Grund.

## Akzeptanzkriterien

- **AC-01** Ticket-Detail: `Prio` (hoch, mittel, niedrig), `Umgebung` (offline, live, ?) und `Projekt` (Kürzel der Projekte) per Menü änderbar; danach stimmen Ticket-Datei und `docs/backlog/README.md` (Test mit Mock-Daten bzw. Go-Test auf einer Kopie von `docs/`).
- **AC-02** Session-Detail: `Agent` und `Umgebung` änderbar wie heute, Verhalten bei erledigten Sessions geklärt (Offene Frage).
- **AC-03** Ein abgelehnter Wert (z. B. Projekt unbekannt) zeigt die Meldung des Tools und ändert nichts.

## Offene Fragen

- Sollen Agent und Umgebung auch bei `fertig`/`verworfen` änderbar sein (heute nur offene Sessions)?
- Soll der Sprint im Detail ebenfalls Felder bekommen (z. B. `Projekt`), oder reicht die Zuordnung über Tickets und Projekt-Karten?

## Notizen

Links, Messwerte, verworfene Ansätze. Darf leer bleiben (`–`).
