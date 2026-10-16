# GEB5.5 · Review und Abnahme

- **Status:** fertig
- **Typ:** Review
- **Agent:** autonom
- **Domäne:** CLI
- **Umgebung:** live
- **Branch:** geb5/5-review
- **Abhängig von:** GEB5.4
- **Tickets:** B-435
- **Kriterien:** AC-03

## Ziel

Der Diff von GEB5 ist geprüft, 🧑 hat Lager, Farm mit Plantage, Kaserne und Treppen in der Vorschau und im Spiel abgenommen, der eine PR des Sprints ist offen.

## Kontext

- Leichter Review: nur schwere Befunde im Diff; besonders ältere PNGs unverändert, gelöschte Grafik nirgends mehr genutzt (auch Fixtures unter `tools/k3c-dev`), Plantage-Zuordnung trifft keine Waldbäume, Datei ≤ 400, Funktion ≤ 60 Zeilen.
- **Sichtprüfung (AC-03):** nur mit Freigabe von 🧑 für diesen Lauf. Vite und Spielserver über `svc_start`, Adressen über `get_urls`. Vorschau `assetvorschau.html?szene=gebaeude`; im Spiel Farm mit Setzlingen und nachgewachsener Pappel (Tag 1), Lager, Kaserne und Treppen ab Hub-Stufe 2 (Dev-Spielstand oder Dev-Befehl, falls vorhanden; sonst prüft 🧑 selbst). Dabei die Annahmen aus GEB5 › Offene Fragen (Maße, Überdeckung Kaserne/Schmiede, zwei Plantage-Phasen) mit 🧑 bestätigen.

## Erlaubte Dateien

- Dateien des Sprint-Diffs, nur für schwere Befunde oder von 🧑 verlangte Nachbesserungen
- `docs/sprints/`, `docs/backlog/`, `docs/projekte/`, `docs/roadmap.md`

## Nicht-Ziele

Stil-Nacharbeit ohne Auftrag von 🧑, GEB6-Objekte, Farm-Stufen, TV-Test.

## Schritte

1. `Status: in Arbeit`; `task check` und `task check:go` grün.
2. `git fetch && git diff origin/develop...origin/sprint/geb5` lesen, Befunde behandeln.
3. Mit Freigabe von 🧑: Vorschau (und ggf. Spiel) zeigen, Abnahme im Ergebnis festhalten.
4. Nachweis je Kriterium AC-01 bis AC-06; Abschnitt „Abnahme“.
5. Sprint nach `erledigt/`, `Status: erledigt`; PR `sprint/geb5` → `develop` öffnen.

## Fertig, wenn

- [ ] AC-03: Abnahme durch 🧑 steht im Ergebnis.
- [ ] AC-01, AC-02, AC-04–AC-06: Nachweise aus GEB5.1–GEB5.4 geprüft.
- [ ] `task check` und `task check:go` grün; PR offen.

## Prüfen

```bash
task check
task check:go
```

Manuelle Prüfung im Browser-Pane nur mit Freigabe von 🧑 für diesen Lauf.

## Ergebnis

- **Prüfen:** `check_run task:check` und `task:check:go` grün (nach `git merge origin/develop`).
- **Review:** `git diff origin/develop...HEAD` von einem vom Schreiber getrennten Reviewer gelesen: keine schweren Befunde (ältere PNGs unverändert, nur `A` für `treppe-*`; gelöschte Grafik nirgends mehr genutzt; `plotX` im Client = Go-Formel; kein `Math.random`; Dateien ≤ 400 Zeilen). Hinweis ohne Befund: Ein Waldbaum, der auf ±0,01 Units genau auf einem Platz steht, würde als Pappel gezeichnet (rein optisch).
- **AC-01, AC-02, AC-04–AC-06 geprüft:** Nachweise aus GEB5.1–GEB5.4 (Session-Ergebnisse); `dev:test` rot nur durch nicht zugehörige Fälle (B-448, Windows-Rename-Flake).
- **AC-03 abgenommen durch 🧑** (Ralf, Chat „abnahme“, 2026-10-16). Welche Teile er angesehen hat, nannte er nicht; Claude hat nicht selbst abgenommen. Nachbesserungen nannte er keine; die Annahmen aus GEB5 › Offene Fragen gelten damit weiter.
- Neue Tickets: B-448 (GEB5.4); B-449 (Abnahme) damit erledigt.
