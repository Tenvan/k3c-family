# B-363 · Die Planungsseite ändert Prio, Umgebung, Agent und Projekt direkt im Detail

- **Domäne:** DEV
- **Typ:** Idee
- **Prio:** mittel
- **Umgebung:** offline
- **Status:** eingeplant
- **Sprint:** WZ2
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

Agenten und 🧑 beim Planen über die Planungsseite von k3c-dev; 🧑 nimmt die Seite am Bildschirm ab.

## Anforderungen

- Änderungen laufen über `plan_set` (wie der Rang) und ziehen Ticket-Datei, `docs/backlog/README.md` und Tabellen nach.
- Ticket-Detail: `Prio`, `Umgebung`, `Projekt` per Menü; Session-Detail: `Agent`, `Umgebung`.
- Agent und Umgebung bleiben nur bei **offenen** Sessions änderbar (heutiges Verhalten, `pickable`); bei `fertig` und `verworfen` nicht (🧑 2026-10-11).
- Der Sprint bekommt **keine** eigenen Felder im Detail; das Projekt kommt über Tickets und Projekt-Karten (🧑 2026-10-11).

## Nicht-Ziele

Flags an erledigten oder verworfenen Sessions ändern; Felder am Sprint (`Projekt` o. Ä.); weitere Felder wie `Status` oder `Rang` in der Seite (laufen bereits anders).

## Regeln und Einschränkungen

Domäne DEV (`tools/k3c-dev/frontend/src/planning/` und Go-Tools), Planung nur über `plan_set`, Datei ≤ 400 Zeilen, Funktion ≤ 60 Zeilen, Komplexitäts-Budget. Die Tool-Meldung bei abgelehntem Wert wird unverändert angezeigt.

## Beispiele

- Ticket B-001 im Detail, `Prio` von mittel auf hoch → Datei und Index zeigen hoch (`AC-01`).
- Offene Session, `Agent` von autonom auf Mensch → Datei und Fahrplan folgen (`AC-02`).

## Ausnahme- und Fehlerfälle

- Unbekannter Wert (z. B. Projekt-Kürzel gibt es nicht) → Meldung des Tools, nichts ändert sich (`AC-03`).
- Session `fertig` oder `verworfen` → Agent und Umgebung nicht änderbar, Menü nicht angeboten (`AC-02`).

## Akzeptanzkriterien

- **AC-01** Ticket-Detail: `Prio` (hoch, mittel, niedrig), `Umgebung` (offline, live, ?) und `Projekt` (Kürzel der Projekte) per Menü änderbar; danach stimmen Ticket-Datei und `docs/backlog/README.md` (Test mit Mock-Daten bzw. Go-Test auf einer Kopie von `docs/`).
- **AC-02** Session-Detail: `Agent` und `Umgebung` bei offenen Sessions änderbar wie heute; bei `fertig` und `verworfen` nicht angeboten (Test).
- **AC-03** Ein abgelehnter Wert (z. B. Projekt unbekannt) zeigt die Meldung des Tools und ändert nichts.

## Offene Fragen

–  (Entschieden von 🧑 am 2026-10-11: Agent/Umgebung nur bei offenen Sessions; keine Felder am Sprint.)

## Notizen

Links, Messwerte, verworfene Ansätze. Darf leer bleiben (`–`).
