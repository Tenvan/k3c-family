# RL1.2 · Probelauf der Checkliste ohne Tag, Sprint abschließen

- **Status:** offen
- **Typ:** Workshop
- **Agent:** Mensch
- **Branch:** rl1/2-probelauf
- **Abhängig von:** RL1.1
- **Tickets:** B-170
- **Kriterien:** AC-03

## Ziel

Die Release-Checkliste ist einmal vollständig durchgegangen, ohne einen Tag zu setzen; das Ergebnis steht in der Abnahme der Sprint-README, rote Punkte sind Tickets, der Sprint liegt in `docs/sprints/erledigt/`.

## Kontext

Doku-Sprint ohne Review: diese Session schließt den Sprint ab (Schritte 4–5 der Review-Session, `docs/arbeitsweise.md` › Sprint-Lebenslauf). 🧑 führt den Probelauf, weil Punkte wie Pi-Image und Version auf der Xbox Geräte brauchen (Plan § 11.1: Sperre durch 🧑 bei RL1). Ein Agent darf mitlaufen: die Befehle der Liste ausführen, CI-Läufe nachsehen und Abnahme sowie Ordner-Verschiebung schreiben. Hardware-Punkte, die das Gerät gerade nicht erlaubt, gelten nach „Hardware entkoppelt“ als `angenommen, Validierung offen` und stehen so in der Abnahme.

## Erlaubte Dateien

- Sprint-README (Abnahme, Tabelle), diese Datei (Ergebnis)
- `docs/backlog/` (Status B-170, neue Tickets für rote Punkte), `docs/sprints/README.md` (Fahrplan), `docs/roadmap.md`

## Nicht-Ziele

Tag setzen, Befunde beheben (nur Tickets), Änderung der Liste (falls nötig: Ticket oder kurze Korrektur mit Begründung im Ergebnis).

## Schritte

1. Stand `origin/develop` nehmen, die Liste aus `docs/arbeitsweise.md` › „Release“ Punkt für Punkt abarbeiten (Befehle lokal, CI-Läufe des letzten Merge-Commits, Pi und Landingpage am Gerät, soweit verfügbar).
2. Je Punkt: grün, rot (Ticket) oder `angenommen, Validierung offen` mit Grund.
3. Abnahme (höchstens fünf Zeilen) in die Sprint-README: Datum, Ergebnis je Kriterium, Tickets, `Version: v… vorgeschlagen (Grund)` (Patch, Doku-Sprint).
4. B-170 auf `erledigt` setzen und nach `docs/backlog/archiv/` verschieben (Index-Zeile in „Archiv“).
5. Sprint-Ordner nach `docs/sprints/erledigt/`, `Status: erledigt`, Fahrplan anpassen, PR öffnen.

## Fertig, wenn

- [ ] AC-03: Probelauf ohne Tag durchgeführt, Ergebnis je Punkt in der Abnahme; rote Punkte als Tickets.
- [ ] Sprint liegt unter `docs/sprints/erledigt/`, B-170 archiviert.

## Prüfen

```bash
task check
```

Manuelle Prüfungen (Pi, Xbox) nur durch 🧑.

## Ergebnis

–
