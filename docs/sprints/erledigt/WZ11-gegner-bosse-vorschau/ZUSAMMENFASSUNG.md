# WZ11 · Zusammenfassung

Regelbasiert aus der Sprint-README erzeugt (`plan_digest`, B-696); maßgeblich bleibt [README.md](README.md).

- **Titel:** Gegner- und Boss-Vorschau im Asset-Studio, Figuren-Seiten entfernen
- **Projekt:** WZG

## Ziel

Nach WZ11 zeigt das Asset-Studio alle Gegnertypen nach Biom und alle Bosse je Tiefe mit den Animationen und Effekten des Spiels, markiert jede fehlende Darstellung als Lücke (auch Warnkreis, Boss-Leiste, Phasen-Text und Event-Banner bis B-469), und `figuren.html`/`aufstellung.html` sind entfernt, weil ihr Inhalt im Studio liegt. Am Ende sichtbar: Gegner-Seite im Fenster „Asset-Studio“ neben Grafik …

## Ergebnis

2026-10-18, Review WZ11.4 (autonom, getrennter Reviewer): AC-01 bis AC-08, AC-12 bis AC-16, AC-18, AC-19 geprüft; AC-09 bis AC-11 verschoben (B-469); AC-17 offen (WZ11.5). Behoben: AC-06 (Zusatzfelder `swarmSize`, `aoe`, `phases`, `kiteDistance` an der Karte, mit Lücke). Keine neuen Tickets. Lücken-Liste und Abnahme-Liste: `WZ11.5-abnahme.md` › Kontext. Version: v– vorgeschlagen (keine, reines Entwickler-Werkzeug).

## Kriterien

| Kriterium | Status | Inhalt |
|---|---|---|
| AC-01 | geprüft | `B-464/AC-01` alle 17 Gegnertypen gelistet (Test). |
| AC-02 | geprüft | `B-464/AC-02` Gruppen je Biom nach Tiefe, `portal` vor `night`, Gruppe „ohne Biom“ (Test). |
| AC-03 | geprüft | `B-464/AC-03` Gegner-Animationen einzeln mit `tint`, `scale`, `alpha`, `y`. |
| AC-04 | geprüft | `B-464/AC-04` fehlende Animation → `idle` und Lücke (Test). |
| AC-05 | geprüft | `B-464/AC-05` Getroffen `hit`, Sterben `kill`, eigene Animationen als Lücke. |
| AC-06 | geprüft | `B-464/AC-06` Merkmale an der Karte, ohne Darstellung als Lücke. |
| AC-07 | geprüft | `B-464/AC-07` Boss-Gruppe je Tiefe mit 6 Bossen (Test). |
| AC-08 | geprüft | `B-464/AC-08` Boss-Grafik wie im Spiel, Lücke „keine eigene Boss-Grafik“. |
| AC-09 | verschoben | `B-464/AC-09` verschoben nach `B-469/AC-01` (Warnkreis); in WZ11 nicht zu erfüllen, die Lü … |
| AC-10 | verschoben | `B-464/AC-10` verschoben nach `B-469/AC-02` und `B-469/AC-03` (Boss-Leiste, Endboss-Phasen … |
| AC-11 | verschoben | `B-464/AC-11` verschoben nach `B-469/AC-04` (Event-Banner für `bossSpawned`, `bossPhase`, … |
| AC-12 | geprüft | `B-464/AC-12` je Boss die zutreffenden Effekte (Warnkreis nur bei Flächenschlag, Phasen-Te … |
| AC-13 | geprüft | `B-464/AC-13` Spielmaßstab, Monarch-Vergleich zuschaltbar. |
| AC-14 | geprüft | `B-464/AC-14` Gegner-Seite nutzt die Abspiel-Bausteine aus WZ10.1 und die Bühne aus WZ7.3; … |
| AC-15 | geprüft | `B-464/AC-15` Grafik-Auswahl je Gegner-ID und `boss`, Register unverändert. |
| … | | weitere in README.md |

## Tickets

| Ticket | Status | Kriterien | Titel |
|---|---|---|---|
| B-464 | erledigt | AC-01 geprüft, AC-02 geprüft, AC-03 geprüft, AC-04 geprüft, AC-05 geprüft, AC-06 geprüft, AC-07 geprüft, AC-08 geprüft, AC-09 verschoben, AC-10 verschoben, AC-11 verschoben, AC-12 geprüft, AC-13 geprüft, AC-14 geprüft, AC-15 geprüft, AC-16 geprüft | Eine Gegner-Seite im Asset-Studio zeigt alle Gegnertypen und Bosse mit ihren Vor … |
