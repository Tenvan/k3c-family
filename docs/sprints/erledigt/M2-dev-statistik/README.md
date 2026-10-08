# M2 · SRV · k3c-dev II: Nutzungsstatistik, Berichte und Spielstände

- **Status:** erledigt
- **Projekt:** –
- **Domäne:** SRV
- **Prio:** mittel
- **Reife:** bereit
- **Einschiebbar:** ja
- **Tickets:** B-062, B-063
- **Start-Commit:** 93cab71
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** 2026-09-30 🧑 Chat (M2 Revision 1 mit B-062 und B-063)

## Ausgangslage

Nach M1 zählt `k3c-dev` Aufrufe nur im Speicher und kennt weder Xbox-Berichte noch Spielstände.

## Ziel

`k3c-dev` wertet Aufrufe über Sitzungen aus (Perzentile, Ausreißer, Zeitreihe) und macht Berichte und Spielstände für
Agenten lesbar. Am Ende sichtbar: Nach einem Neustart nennt `workbench_status` weiter die Gesamtzahlen; `reports_list`
und `saves_list` antworten mit einer Zeile je Datei.

## Beteiligte und Zielgruppen

Entwickler und Coding-Agenten; die MCP-Seite (M4) nutzt die Statistik.

## Anforderungen

B-062 › Anforderungen, B-063 › Anforderungen.

## Nicht-Ziele

B-062 › Nicht-Ziele, B-063 › Nicht-Ziele. Keine Oberfläche (M3, M4).

## Regeln und Einschränkungen

B-062 und B-063 › Regeln und Einschränkungen. Einschiebbar nach M1. Nur Standardbibliothek, keine Ausnahmen außerhalb
der Domäne.

## Beispiele

B-062 › Beispiele, B-063 › Beispiele.

## Ausnahme- und Fehlerfälle

B-062 und B-063 › Ausnahme- und Fehlerfälle.

## Akzeptanzkriterien

- **AC-01** Perzentile, Ausreißer und Ranglisten (B-062/AC-01, B-062/AC-02, B-062/AC-03).
- **AC-02** Zeitreihe und Speichern über Neustarts (B-062/AC-04, B-062/AC-05).
- **AC-03** Berichte und Spielstände lesbar, Pfade begrenzt (B-063/AC-01, B-063/AC-02, B-063/AC-03).
- **AC-04** `npm run check:dev` grün, Grenzen eingehalten (B-062/AC-06).

## Offene Fragen

keine

## Sessions

| Nr. | Datei | Typ | Agent | Status |
|---|---|---|---|---|
| M2.1 | `M2.1-nutzungsstatistik.md` | Umsetzung | autonom | fertig |
| M2.2 | `M2.2-speichern-spieldaten.md` | Umsetzung | autonom | fertig |
| M2.3 | `M2.3-review.md` | Review | autonom | fertig |

## Abnahme

- 2026-09-30, leichtes Review über `93cab71..main` (26 Code-Dateien) durch drei Reviewer (Sonnet): Statistik, Speichern, Spieldaten.
- Kriterien: AC-01 geprüft (M2.1), AC-02 und AC-03 geprüft (M2.2), AC-04 geprüft (M2.1, M2.2, nach den Fixes erneut).
- Behoben: unbekannte Tool-Namen ließen Statistik und Datei ohne Grenze wachsen; zweite Beschädigung überschrieb die
  erste `.bak` (jetzt mit Zeitstempel); fremde Datei mit Lücken → Panik nach dem `recover`; Lesefehler verschob eine
  gültige Datei und der erste Flush überschrieb sie (jetzt unangetastet, Lauf nur im Speicher); Timer- und
  End-Flush konnten sich überholen; doppelte Minute an der Minutenwende. Neue Tickets: keine.
