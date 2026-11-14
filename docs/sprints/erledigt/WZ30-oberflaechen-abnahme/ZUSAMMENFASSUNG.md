# WZ30 · Zusammenfassung

Regelbasiert aus der Sprint-README erzeugt (`plan_digest`, B-696); maßgeblich bleibt [README.md](README.md).

- **Titel:** Oberflächen zur Abnahme: Szenen, Filter, Quellen, Chat-Karten mit Mock-Daten
- **Projekt:** WZG

## Ziel

Alle in WZG geplanten Seiten und Dialoge stehen im Studio und in der Workbench, bedienbar mit Mock-Daten, sodass Ralf sie abnehmen kann. Am Ende sichtbar: Studio im Mock (`?mock=1`) zeigt Filter, Sound-Szenen und Proben, Grafik-Szenen mit Kampf-Anzeigen, Seite „Quellen“ mit Download-Dialog und Chat-Vorschlagskarten.

## Ergebnis

2026-10-24: AC-01 bis AC-05 erfüllt (WZ30.1–WZ30.5 › Ergebnis), AC-06 offen (WZ30.7, Ralf). Behoben im Review: Szenenleiste, Dateigrenze `api/types.ts`. Bekannt: `engine/devrun`-Uhrzeit-Test unter Last (B-284). Keine neuen Tickets. 2026-10-31: AC-06 von Ralf (🧑) abgenommen, im Chat bestätigt; Claude hat die Prüfung nicht selbst gesehen. Alle Kriterien erfüllt, Sprint erledigt.

## Kriterien

| Kriterium | Status | Inhalt |
|---|---|---|
| AC-01 | erfüllt | Filter- und Sortier-Baustein in `@webkit`, Sound- und Grafik-Szenen nutzen ihn (B-599/AC-0 … |
| AC-02 | erfüllt | Sound-Szenen Skills, Gegner/Bosse, Bauen, Reiten, UI, Warnungen sowie Misch-, Lautheits- u … |
| AC-03 | erfüllt | Grafik-Szenen Skill-Effekte, Schilder/Gerüste, Gegner je Biom, Reittiere, Bosse und „Kampf … |
| AC-04 | erfüllt | Seite „Quellen“ mit Übersicht je Quelle, Vorschlagsliste, Bestätigungsdialog für den Downl … |
| AC-05 | erfüllt | Chat-Vorschlagskarten mit Vorschau/Hörprobe, „Übernehmen“/„Verwerfen“, gesperrtem „Überneh … |
| AC-06 | offen | Ralf hat alle WZG-Oberflächen (WZ28 und WZ30) im Mock abgenommen. |

## Tickets

| Ticket | Status | Kriterien | Titel |
|---|---|---|---|
| B-597 | erledigt | AC-02 erfüllt | Die Sound-Seite hat differenzierte Hörproben, auch für Skills |
| B-598 | erledigt | AC-03 erfüllt | Die Grafik-Seite hat differenzierte Vorschauen, auch für Skill-Effekte |
| B-599 | erledigt | AC-01 erfüllt | Szenen auf Sound- und Grafik-Seite sind sortier- und filterbar |
| B-600 | erledigt | AC-05 erfüllt | Der AI-Chat unterstützt die Zuordnung auf Sound- und Grafik-Seite |
| B-601 | erledigt | AC-04 erfüllt | Eine Download-Vorschlagsliste für empfohlene Assets ermöglicht Offline-Tests |
| B-602 | erledigt | AC-03 erfüllt | Das Studio zeigt Boss-Leiste, Phasen, Warnkreis und Event-Banner mit Mock-Zustän … |
