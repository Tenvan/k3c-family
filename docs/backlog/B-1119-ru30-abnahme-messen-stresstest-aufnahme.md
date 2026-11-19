# B-1119 · RU30 Abnahme: Messläufe, Stresstest und Aufnahmevergleich des Rust-Spiels

- **Domäne:** CLI
- **Typ:** Schuld
- **Prio:** mittel
- **Umgebung:** ?
- **Status:** offen
- **Sprint:** RU51
- **Projekt:** RSU
- **Erstellt:** 2026-11-19
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –
- **Duplikat von:** –
- **Ersetzt:** –

## Ausgangslage

Sprint RU30 baut nur das Werkzeug für Messmodus (`--measure`), Stresstest und Aufnahme-Modus. Die Läufe selbst fallen unter die Sim-Sperre und „Messungen zum Schluss“. Lane: zweit (Second Worker fährt sie).

## Ziel

Messläufe, Stresstest und Aufnahmevergleich des Rust-Spiels gegen das Go-Spiel fahren und sichten; Prüfliste für Ralf im Spiel.

## Beteiligte und Zielgruppen

Wer spielt, entwickelt, betreibt oder entscheidet (🧑)? Keine Verantwortlichen erfinden.

## Anforderungen

- Was das Ergebnis können muss, auch Qualität (deterministisch, 2+ Spieler, Leistung).

## Nicht-Ziele

Was ausdrücklich nicht dazugehört, mit Ticket-Nummer, falls es später kommt.

## Regeln und Einschränkungen

Regeln aus `CLAUDE.md`, Entscheidungen (`docs/decisions/`), Domäne, Komplexitäts-Budget, Verträge (Protokoll, Spielstand).

Planungsangaben für die Arbeitswellen (`docs/arbeitsweise.md` › Parallelität und Wellen), je eine Zeile, Werte ohne Freitext; Pflicht ab `Spec: freigegeben`:

- **Bereich:** `pfad/**`, `pfad/datei.go` (alle Dateien, die das Ticket anfasst; Sammeldateien wie `docs/backlog/README.md` nicht nennen)
- **Abhängig von:** – (nur IDs: B-000, SP00, SP00.1; der andere Stand muss erledigt und gemergt sein)
- **Braucht 🧑:** – (oder Freigabe | Entscheidung | Abnahme | Gerät; `–` heißt vollständig autonom)
- **Größe:** S (S = eine Session, M = zwei bis drei, L = vor der Freigabe in Tickets je Teil zerlegen)
- **Sperre:** – (oder golden | protokoll | spielstand | deps | taskfile | exe)

## Beispiele

Typische Situation → erwartetes Ergebnis. Passt nichts: `nicht relevant` mit Grund.

## Ausnahme- und Fehlerfälle

Ungültige oder seltene Situation → gewolltes Verhalten. Passt nichts: `nicht relevant` mit Grund.

## Akzeptanzkriterien

- **AC-01** `--measure 60` (ruhiger Rechner, keine parallelen Builds) schreibt einen Bericht; er liegt neben dem Go-Bericht (`bilder_ohne_takt` rund die Hälfte bei 60 Hz Bild und 30 Hz Takt ist normal; deckelt Fifo-VSync die Bildrate? sonst VSync-Option für RU39); `--measure-load` Go gegen Rust.
- **AC-02** Ein Stresstest (nach B-1117) schreibt `reports/native-stress-*.json` mit denselben Feldern wie der Go-Bericht.
- **AC-03** `--shots` mit `--seed`, `--menu`, `--flow`, `--at` nach `task gen:paper`: Sichtvergleich mit den Go-Bildern (B-1114); F12 hat bis B-1115 keine Sprites.
- **AC-04** Ralf prüft im Spiel: (1) `k3c-game.exe` startet, Hauptmenü, Icon (wartet auf B-1118); (2) zwei Spieler spielen eine Nacht mit Pad und Tastatur, jeder Monarch mit eigener Bildhälfte, Beitritt/Verlassen, Pause (B im Menü zurück, im Spiel unbelegt), Niederlage führt ins Hauptmenü; (3) Ruckeln bei 60 Hz in beiden Hälften, auch mit vielen Gegnern, beim Stufen- und Inselwechsel (Schiffsfahrt, Ladeanzeige ohne Hänger), Leistungsanzeige F2; (4) Entwickler-Tasten F1, F5 bis F10, F12; (5) fehlender Grafik-Ordner (`--grafik` auf leeren Ordner) zeigt Meldung mit Pfad, Enter beendet mit Exit 1; (6) Stresstest aus dem Testing-Menü schreibt einen Bericht.

## Offene Fragen

Entscheidung, betroffener Umfang, wer entscheidet (🧑). Sonst `keine`.

## Notizen

Links, Messwerte, verworfene Ansätze. Darf leer bleiben (`–`).
