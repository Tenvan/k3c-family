# GEB5.5 · Review und Abnahme

- **Status:** offen
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

Wird am Ende der Session ausgefüllt: Nachweis je Kriterium (`AC-01 geprüft: task check grün`,
`AC-02 verschoben: Grund, B-0NN`), wer manuell geprüft hat, Abweichungen vom Plan, neue Tickets. Bis dahin `–`.
