# WZ29 · Zusammenfassung

Regelbasiert aus der Sprint-README erzeugt (`plan_digest`, B-696); maßgeblich bleibt [README.md](README.md).

- **Titel:** Ausbau der Sound- und Grafik-Seite: Hörproben, Vorschauen, Filter, AI-Chat
- **Projekt:** WZG

## Ziel

Sound- und Grafik-Seite haben differenzierte Hörproben und Vorschauen (auch Skills und Skill-Effekte), einheitlich filter- und sortierbare Szenen, eine Chat-Unterstützung mit bestätigten Vorschlägen und einen Offline-Vorrat: eine gefüllte Download-Vorschlagsliste für Ton und Grafik, aus der Ralf online empfohlene Assets vorab lädt. Am Ende sichtbar: Skill-Szene mit Hörprobe, Skill-Effekte-Szene, F …

## Ergebnis

2026-10-25, Claude (WZ29.8): - Prüfläufe: `task check`, `task check:go` und `task check:studio` sind grün. Zuvor hat Claude einen Lint-Fehler behoben: `describe` mit mehr als 60 Zeilen in `SuggestionList.test.tsx`, jetzt aufgeteilt. `check:race` wurde lokal übersprungen (kein C-Compiler), die CI prüft es. - Kriterien: AC-01 erfüllt durch WZ30.1 und WZ29.2/.4, AC-02 durch WZ29.2, AC-03 durch WZ29.4, AC-04 durch WZ29.7, AC-05 durch WZ29.5 und .6. Die Reviews zu WZ29.6 (SSRF, Bestätigung im Backend) und WZ29.7 (Q70 im Backend) sind behoben. - Offline: Die gebaute Studio-Oberfläche lädt keine exte …

## Kriterien

| Kriterium | Status | Inhalt |
|---|---|---|
| AC-01 | erfüllt | Gemeinsamer Filter-/Sortier-Baustein, beide Seiten nutzen ihn (B-599). Erfüllt durch WZ30. … |
| AC-02 | verschoben | Sound-Szenen Skills, Gegner/Bosse, Bauen, Reiten, UI, Warnungen, Misch-, Lautheits- und Ab … |
| AC-03 | geprüft | Grafik-Szenen Skill-Effekte, Schilder/Gerüste, Gegner je Biom, Reittiere, Bosse, Biom und … |
| AC-04 | – | Der Chat erhält den Seitenkontext, Vorschläge als Karten, Schreiben nur nach „Übernehmen“, … |
| AC-05 | – | Die Download-Vorschlagsliste für Ton und Grafik ist gefüllt (mindestens drei Kandidaten je … |
| AC-06 | – | Ralf hat Szenen, Filter, einen Chat-Vorschlag, den Offline-Vorrat und die Kampf-Anzeigen a … |

## Tickets

| Ticket | Status | Kriterien | Titel |
|---|---|---|---|
| B-597 | erledigt | AC-02 verschoben | Die Sound-Seite hat differenzierte Hörproben, auch für Skills |
| B-598 | erledigt | AC-03 geprüft | Die Grafik-Seite hat differenzierte Vorschauen, auch für Skill-Effekte |
| B-599 | erledigt | AC-01 erfüllt | Szenen auf Sound- und Grafik-Seite sind sortier- und filterbar |
| B-600 | erledigt | AC-04 –, AC-06 – | Der AI-Chat unterstützt die Zuordnung auf Sound- und Grafik-Seite |
| B-601 | erledigt | AC-05 –, AC-06 – | Eine Download-Vorschlagsliste für empfohlene Assets ermöglicht Offline-Tests |
| B-602 | erledigt | AC-03 geprüft, AC-06 – | Das Studio zeigt Boss-Leiste, Phasen, Warnkreis und Event-Banner mit Mock-Zustän … |
