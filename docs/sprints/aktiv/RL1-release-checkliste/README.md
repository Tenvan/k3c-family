# RL1 · INF · Release-Checkliste

- **Status:** aktiv
- **Domäne:** INF
- **Prio:** hoch
- **Reife:** bereit
- **Einschiebbar:** ja
- **Tickets:** B-170
- **Start-Commit:** 1fa9529
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** 2026-10-04, Chat (Ralf), Revision 1, durch 🧑; umfasst B-170; mit Änderungen aus dem Spec-Review (Q20-Frage gestrichen, B-170 an Q20 angeglichen)

## Ausgangslage

`task check:all` und die CI (`.github/workflows/ci.yml`) existieren, eine Release-Checkliste nicht. Details in B-170.

## Ziel

Ein Abschnitt „Release“ in `docs/arbeitsweise.md` macht jeden Release prüfbar. Am Ende sichtbar: die Liste und ein Probelauf ohne Tag.

## Beteiligte und Zielgruppen

🧑 gibt Releases frei und beschließt den Rhythmus (Q20); Agent schreibt und prüft die Liste.

## Anforderungen

B-170 › Anforderungen.

## Nicht-Ziele

itch.io und Englisch (B-023), Pi-Image (SP11), Version im Status (B-141).

## Regeln und Einschränkungen

Prozess nur in `docs/arbeitsweise.md`, kein neues Prozess-Dokument; Aufgaben über `task`; Taggen nur durch 🧑.

## Beispiele

Phase 1 fertig → Liste abarbeiten → 🧑 setzt Tag `v0.1.0`.

## Ausnahme- und Fehlerfälle

Ein Punkt rot → kein Tag, Befund als Ticket.

## Akzeptanzkriterien

- **AC-01** `docs/arbeitsweise.md` enthält den Abschnitt „Release“, jeder Punkt nennt Befehl oder Datei und erwartetes Ergebnis (B-170/AC-01).
- **AC-02** Die Liste verweist auf Golden amd64 und arm64, `task check:all`, Migration, Dev-Reste, Pi-Image, Version, Credits und Tag-Schema (B-170/AC-02).
- **AC-03** Ein Probelauf ohne Tag ist durchgeführt und in der Abnahme festgehalten (B-170/AC-03).
- **AC-04** Rhythmus und Auslöser sind von 🧑 beschlossen (B-170/AC-04).

## Offene Fragen

keine (Release-Rhythmus: Q20, geklärt 2026-10-03)

## Sessions

| Nr. | Datei | Typ | Agent | Status |
|---|---|---|---|---|
| RL1.1 | `RL1.1-abschnitt-release.md` | Umsetzung | autonom | fertig |
| RL1.2 | `RL1.2-probelauf.md` | Workshop | Mensch | offen |

Bei `Reife: Entwurf` genügen Stichpunkte mit den Kriterien in Klammern statt Session-Dateien. Vor dem Aktivieren
wird jede Session als eigene Datei nach `docs/vorlagen/session.md` geschrieben.

## Abnahme

2026-10-04, Agent (Claude Opus 5.5) in RL1.2, Abschluss auf Anweisung von 🧑 (Chat). AC-01, AC-02, AC-04: Ergebnis RL1.1. AC-03: Probelauf ohne Tag durchgeführt (Ergebnis RL1.2, Stand `origin/develop` 46aa69b); Pi-Pull und Versionszeile auf der Xbox angenommen, Validierung offen.
Rot: Dev-Mode im Release-Image an (B-273), `TestRestore` flackert unter Windows (B-274), Overlay standardmäßig an (B-098, K5). Ein Tag wäre damit blockiert.
Version: v0.6.1 vorgeschlagen (Doku-Sprint, Patch); gesetzt erst nach Bestätigung durch 🧑 und grüner Checkliste.
