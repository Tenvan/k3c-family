# M2 · SRV · k3c-dev II: Nutzungsstatistik, Berichte und Spielstände

- **Status:** geplant
- **Domäne:** SRV
- **Reife:** Entwurf
- **Einschiebbar:** ja
- **Tickets:** B-062, B-063
- **Start-Commit:** –
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

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

Entwurf. Vor dem Aktivieren jede Session als Datei nach `docs/vorlagen/session.md` schreiben, die Kriterien in Klammern werden ihr Feld `Kriterien`.

- M2.1 `internal/usage`: Bereiche, Histogramm, Perzentile, Ausreißer, Ranglisten, Zeitreihe, Schnappschuss (AC-01, AC-02).
- M2.2 Speichern (atomar, `.bak`), `workbench_status` ergänzen, `internal/gamedata` mit `reports_list`, `report_read`,
  `saves_list`, Instructions ergänzen (AC-02, AC-03, AC-04).
- M2.3 🔍 Review (alle).

## Abnahme

–
