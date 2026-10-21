# WZ14 · REG · Geplante Chunks tragen Ruinen und Gegnerlager

- **Status:** geplant
- **Projekt:** WZG
- **Domäne:** REG
- **Reife:** Entwurf
- **Tickets:** B-468
- **Start-Commit:** – (wird beim Aktivieren gesetzt: `git rev-parse --short origin/develop`)
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** 2026-10-21, Ralf im Chat (Revision 2)

## Ausgangslage

`B-468 › Ausgangslage`: Geplante Chunks (B-461, WZ8) tragen feste Objekte, aber keine vorgebauten Gebäude (Ruinen) und keine festen Gegner (Gegnerlager); dafür gibt es in Sim, Spielstand und Wellenregeln keine Grundlage.

## Ziel

Ein geplanter Chunk kann eine Ruine und ein Gegnerlager tragen; beides ist im Editor des Welt-Betrachters setzbar, wird beim Laden geprüft, erscheint in Vorschau und Spiel und verhält sich nach neuen Regeln (`B-468 › Ziel`). Am Ende sichtbar: Sondergegend mit Ruine und Gegnerlager in Chunk- und Live-Vorschau, von 🧑 abgenommen.

## Beteiligte und Zielgruppen

`B-468 › Beteiligte und Zielgruppen`: 🧑 entscheidet die Regeln im Workshop, plant im Editor, nimmt ab und gibt die Spec frei; Spielende; Agenten je Domäne, getrennter Reviewer.

## Anforderungen

`B-468 › Anforderungen` (grob, bis WZ8 erledigt und der Regel-Workshop gelaufen ist). Sprint-eigen: Die Spec wird nach WZ8 und dem Workshop vervollständigt; erst dann wird der Sprint bereit gemacht. Der Zuschnitt (ein oder zwei Sprints: Ruinen, Gegnerlager) ist offen.

## Nicht-Ziele

`B-468 › Nicht-Ziele`: alles, was WZ8 liefert (Format, Chunk-Seed, feste Objekte, Generator, Editor); neue Gegner-Arten, Bosse, Wellen-Balancing; mehrere Inseln (B-103); Hub-Freischaltung (B-462); Migration alter Spielstände.

## Regeln und Einschränkungen

`B-468 › Regeln und Einschränkungen`: Abhängigkeit von WZ8 (erledigt); Domänen nacheinander REG → SIM → SRV → CLI → DEV, je Session eine; Regeln aus B-461 (eigener Zufalls-Strom je geplantem Chunk, Golden ohne Planung byte-gleich, atomares Schreiben); mit 2 Spielern gleichzeitig; Taste B unbelegt.

## Beispiele

`B-468 › Beispiele`: Wald, Platz +4 mit Ruine „Schmiede“ → beschädigte Schmiede an fester Lage, reparierbar; Höhle, Platz −3 mit Gegnerlager → feste Gegner greifen in Reichweite an (Regeln offen, Workshop).

## Ausnahme- und Fehlerfälle

`B-468 › Ausnahme- und Fehlerfälle`: ungültige Lage, unbekannte Gebäudeart oder Gegner-Art → beim Laden abgelehnt mit Insel, Tiefe, Platz und Grund; Verhalten beim Burgfall laut Regel (offen).

## Akzeptanzkriterien

Grob; Nummern und Nachweise werden mit der Spec vervollständigt.

- **AC-01** Regeln für Ruinen und Gegnerlager von 🧑 beschlossen, Regeltext und Glossar (`B-468/AC-01`).
- **AC-02** Der Generator liest und prüft die reservierten Felder unabhängig vom Seed (`B-468/AC-02`).
- **AC-03** Ruinen und Gegnerlager verhalten sich in der Sim nach den Regeln, auch mit 2 Spielern (`B-468/AC-03`).
- **AC-04** Ohne Ruine und Gegnerlager bleiben Golden-Daten und Benchmark unverändert (`B-468/AC-04`).
- **AC-05** Vorschau, Grafik und Editor zeigen und bearbeiten beides (`B-468/AC-05`).
- **AC-06** 🧑 nimmt eine Sondergegend in der Vorschau ab (`B-468/AC-06`).

## Offene Fragen

Entscheidet 🧑 im Regel-Workshop nach WZ8: Regeln für Ruinen und Gegnerlager, Spielstand-Version und Protokoll, Sprint-Zuschnitt (`B-468 › Offene Fragen` 1–4). Blockiert die Freigabe.

## Sessions

| Nr. | Datei | Typ | Agent | Status |
|---|---|---|---|---|
| WZ14.1 | `WZ14.1-workshop-regeln.md` | Workshop | Mensch | offen |

- WZ14.1 (REG) Workshop mit 🧑: Regeln für Ruinen und Gegnerlager, Regeltext, Glossar (AC-01).
- WZ14.2 (SIM) Reservierte Felder lesen und prüfen, Sim-Logik für Ruine und Gegnerlager (AC-02, AC-03, AC-04).
- WZ14.3 (SRV) Vorschau-Snapshot und Spielstand (AC-03, AC-05).
- WZ14.4 (CLI) Grafik für Ruine und Gegnerlager (AC-05).
- WZ14.5 (DEV) Editor und Vorschau erweitern (AC-05).
- WZ14.6 (DEV) Review: `task check:all`, Diff aller Domänen, PR (alle).
- WZ14.7 (DEV) Abnahme durch 🧑 am PC, Agent Mensch (AC-06).

## Abnahme

Wird von der Review-Session (Doku-Sprint: letzte Session) ausgefüllt, höchstens fünf Zeilen: Datum, Kriterien
(Verweis auf Session-Ergebnisse), behobene Befunde, neue Tickets. Bis dahin `–`.
