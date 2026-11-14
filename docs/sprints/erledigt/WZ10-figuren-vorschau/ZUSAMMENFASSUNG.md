# WZ10 · Zusammenfassung

Regelbasiert aus der Sprint-README erzeugt (`plan_digest`, B-696); maßgeblich bleibt [README.md](README.md).

- **Titel:** Abspiel-Bausteine und Monarch-Vorschau im Asset-Studio
- **Projekt:** WZG

## Ziel

Nach WZ10 spielt das Asset-Studio den Monarchen (immer beritten) mit den Animationen und Effekten des Spiels ab, zeigt alle 13 Reittiere, Skills, Münzen, Getroffen/Tod/Respawn und die vier Spielerfarben nebeneinander, markiert jede fehlende Darstellung als Lücke und stellt die Abspiel-Bausteine bereit, die WZ11 für die Gegner-Seite wiederverwendet. Am Ende sichtbar: die Monarch-Seite im Fenster „A …

## Ergebnis

2026-10-18 (Review WZ10.4): AC-01 bis AC-12, AC-15, AC-16 mit Nachweis aus Code, Tests und getrenntem Reviewer (Ergebnisse WZ10.1–WZ10.4); AC-13 und AC-14 verschoben nach WZ11 (AC-18, AC-19); AC-17 offen (WZ10.5, 🧑 am PC). Befunde: keine schweren; Abweichung von B-463/AC-02 (Laden der Reittiere außerhalb des Atlas nachgebaut, `src/scenes/` ist tabu) mit Ticket B-476. Version: v… vorgeschlagen: keine Änderung (Werkzeug-Sprint, kein Spielstand- oder Protokoll-Wechsel).

## Kriterien

| Kriterium | Status | Inhalt |
|---|---|---|
| AC-01 | – | `B-463/AC-01` jede Monarch-Animation einzeln (`run` beritten), Schleife, Einzelbild, Tempo … |
| AC-02 | – | `B-463/AC-02` Animationsdefinitionen nur aus dem Spiel bzw. `data/sprites.json`. |
| AC-03 | geprüft | `B-463/AC-03` Reiten auf allen 13 Reittieren, Pose gleich `mountPose` (Test). |
| AC-04 | geprüft | `B-463/AC-04` Laufen und Sprinten nur beritten, kein Laufen oder Sprinten zu Fuß, keine Lü … |
| AC-05 | geprüft | `B-463/AC-05` Schlagen mit `attack` und Effekt `swing`. |
| AC-06 | geprüft | `B-463/AC-06` alle Skills mit Linie, Stufe, Art, Lücken markiert. |
| AC-07 | geprüft | `B-463/AC-07` Preset-Skills ohne Eintrag (`backstab`, `smokeBomb`) als Lücke (Test). |
| AC-08 | geprüft | `B-463/AC-08` Münzen geben und aufheben, Wurf als Lücke. |
| AC-09 | geprüft | `B-463/AC-09` Getroffen, Tod, Respawn wie im Spiel, Lücken markiert. |
| AC-10 | geprüft | `B-463/AC-10` alle 4 Spielerfarben nebeneinander synchron, je mit Kennfarbe aus `PLAYER_CO … |
| AC-11 | geprüft | `B-463/AC-11` Spielmaßstab, Höhe gleich `spriteTop` (Test). |
| AC-12 | geprüft | `B-463/AC-12` Grafik-Auswahl `monarch:*` und `mount:*`, Register unverändert. |
| AC-13 | – | `B-463/AC-13` verschoben nach WZ11 (dort AC-18): `figuren.html`/`aufstellung.html` samt Sk … |
| AC-14 | verschoben | `B-463/AC-14` verschoben nach WZ11 (dort AC-19): jede Karte der alten Seiten hat vorher ei … |
| AC-15 | geprüft | `B-463/AC-15` Vitest der Monarch-Logik in `task check:all` und CI. |
| … | | weitere in README.md |

## Tickets

| Ticket | Status | Kriterien | Titel |
|---|---|---|---|
| B-463 | erledigt | AC-01 –, AC-02 –, AC-03 geprüft, AC-04 geprüft, AC-05 geprüft, AC-06 geprüft, AC-07 geprüft, AC-08 geprüft, AC-09 geprüft, AC-10 geprüft, AC-11 geprüft, AC-12 geprüft, AC-13 –, AC-14 verschoben, AC-15 geprüft, AC-16 geprüft | Eine Monarch-Seite im Asset-Studio zeigt Reiten, Schlagen, Skills und weitere Vo … |
