# WZ6 · INF, DEV · Asset-Studio als eigene EXE mit Seiten für Grafik und Sound

- **Status:** geplant
- **Projekt:** WZG
- **Domäne:** INF, DEV
- **Reife:** bereit
- **Tickets:** B-459
- **Start-Commit:** – (wird beim Aktivieren gesetzt: `git rev-parse --short origin/develop`)
- **Spec:** freigegeben
- **Revision:** 2
- **Freigabe:** 2026-10-17, Chat (Ralf)

## Ausgangslage

B-459 › Ausgangslage: Die Assets-Seite steckt in k3c-dev; jede geplante Änderung stört die Workbench (Neubau, MCP-Ausfall). Grafik und Sound teilen sich eine Seite.

## Ziel

Ein eigenes Asset-Studio (EXE mit festem Pfad, eigener Task) mit je einer Seite für Grafik und Sound; die Workbench ist davon entkoppelt. Am Ende sichtbar: beide Seiten im neuen Fenster, Workbench ohne Assets-Seite, von 🧑 abgenommen.

## Beteiligte und Zielgruppen

`B-459 › Beteiligte und Zielgruppen`; Agenten setzen um, ein getrennter Reviewer prüft, 🧑 nimmt am PC ab.

## Anforderungen

`B-459 › Anforderungen`. Sprint-eigen: gemeinsames Go-Modul `tools/assetkit` (aus `tools/k3c-dev/internal/assets`, mit `replace` wie in `tools/k3c-dev/go.mod`, kein `go.work`); Studio-Frontend liest Spiel-Code aus `src/` nur über einen Vite-/TS-Alias; die reine Logik von `assetvorschau` zieht ins Studio-Frontend um.

## Nicht-Ziele

`B-459 › Nicht-Ziele`.

## Regeln und Einschränkungen

`B-459 › Regeln und Einschränkungen`. Domänen nacheinander: INF (WZ6.1) → DEV (WZ6.2–WZ6.8). `assetvorschau.html` fällt als Grenzfall in DEV (wie B-299). Browser- oder Fensterprüfungen nur mit Freigabe von 🧑 für den Lauf.

## Beispiele

`B-459 › Beispiele`.

## Ausnahme- und Fehlerfälle

`B-459 › Ausnahme- und Fehlerfälle`.

## Akzeptanzkriterien

- **AC-01** `B-459/AC-01` eigene EXE `bin/k3c-studio.exe` mit Task, nur Loopback.
- **AC-02** `B-459/AC-02` Seiten Grafik und Sound mit heutigem Umfang.
- **AC-03** `B-459/AC-03` Workbench ohne Assets-Seite, Verweis startet das Studio, Neubau stört sie nicht.
- **AC-04** `B-459/AC-04` Tests in `task check:all` und CI.
- **AC-05** `B-459/AC-05` Domänentabelle und Glossar.
- **AC-06** `B-459/AC-06` gesamter Inhalt von `assetvorschau.html` (Grafik- und Sound-Szenen) im Studio, Seite und Skript entfernt; `grafiken.html`, `soundtest.html` bleiben, Inhalte zusätzlich im Studio.
- **AC-07** `B-459/AC-07` MCP bleibt in k3c-dev; Neuladen ohne Verlust, Speichern auf veraltetem Stand abgelehnt.
- **AC-08** 🧑 nimmt Studio, umgezogene Vorschau und ruhige Workbench ab und entscheidet über B-421 (Abnahme-Session).

## Offene Fragen

Entschieden von 🧑 am 2026-10-17 (B-459 › Offene Fragen); Revision 2 freigegeben am 2026-10-17 (Chat). Offen: keine.

## Sessions

| Nr. | Datei | Typ | Agent | Status |
|---|---|---|---|---|
| WZ6.1 | `WZ6.1-domaene-glossar.md` | Umsetzung | autonom | offen |
| WZ6.2 | `WZ6.2-geruest-k3c-studio.md` | Umsetzung | autonom | offen |
| WZ6.3 | `WZ6.3-seite-grafik.md` | Umsetzung | autonom | offen |
| WZ6.4 | `WZ6.4-seite-sound.md` | Umsetzung | autonom | offen |
| WZ6.5 | `WZ6.5-workbench-ohne-assets.md` | Umsetzung | autonom | offen |
| WZ6.6 | `WZ6.6-assetvorschau-entfernen.md` | Umsetzung | autonom | offen |
| WZ6.7 | `WZ6.7-review.md` | Review | autonom | offen |
| WZ6.8 | `WZ6.8-abnahme.md` | Review | Mensch | offen |

## Abnahme

Wird von der Review-Session (Doku-Sprint: letzte Session) ausgefüllt, höchstens fünf Zeilen: Datum, Kriterien
(Verweis auf Session-Ergebnisse), behobene Befunde, neue Tickets. Bis dahin `–`.
