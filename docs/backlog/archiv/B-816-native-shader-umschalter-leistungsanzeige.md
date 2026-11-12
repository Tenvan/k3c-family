# B-816 · Fluss-Shader und Kacheln lassen sich im Spiel umschalten, Bildrate und Rechenzeit sind im Bild sichtbar

- **Domäne:** CLI
- **Typ:** Idee
- **Prio:** niedrig
- **Umgebung:** live
- **Status:** erledigt
- **Sprint:** NG14
- **Projekt:** NAT
- **Erstellt:** 2026-11-11
- **Spec:** freigegeben
- **Revision:** 2
- **Freigabe:** autonom, Claude, 2026-11-12

## Ausgangslage

Der Fluss-Shader (NG11, B-790) lässt sich nur über die Umgebungsvariable `K3C_RIVER` gegen Kacheln umschalten. Bildrate und Rechenzeit sind nur im Messmodus sichtbar, nicht im Bild.

## Ziel

Ralf schaltet Kachel und Shader im laufenden Spiel um und sieht Bildrate und Rechenzeit im Bild.

## Beteiligte und Zielgruppen

Ralf (Sichtprüfung am PC und Mini-PC), Claude (Umsetzung).

## Anforderungen

- Umschalten per Taste, Wert bleibt erhalten: neues Feld in `view.Settings`, gespeichert in `_settings.json` (`local/settings.go`), Standard wie heute (`K3C_RIVER` setzt weiter den Startwert, wenn keine Einstellung gespeichert ist).
- Optionale Anzeige von Bildrate und Rechenzeit im Bild (Fläche `StatsRect`), ebenfalls als Einstellung gespeichert.
- Tasten (entschieden): F3 schaltet Fluss Kacheln/Shader, F2 die Leistungsanzeige; nur Tastatur, kein Eintrag im Optionen-Menü.
- Keine Kosten, wenn die Anzeige aus ist (keine Wanduhr, keine Allokation je Bild).

## Nicht-Ziele

Neue Shader-Effekte, Messläufe (gesperrt).

## Regeln und Einschränkungen

Entscheidung 008. Die Taste darf `View + Menu` und `B` nicht belegen. Die Verträge `view` und `world` ändern sich nur zusätzlich; `view.Settings` bekommt Felder nur additiv.

Planungsangaben für die Arbeitswellen (`docs/arbeitsweise.md` › Parallelität und Wellen):

- **Bereich:** `native/k3c-game/view/**`, `native/k3c-game/world/**`, `native/k3c-game/game/**`, `native/k3c-game/local/settings.go`, `native/k3c-game/local/settings_test.go`
- **Abhängig von:** –
- **Braucht 🧑:** –
- **Größe:** S
- **Sperre:** –

## Beispiele

Taste drücken → Fluss wechselt von Kacheln zu Shader ohne Neustart.

## Ausnahme- und Fehlerfälle

Shader-Fehler → Rückfall auf Kacheln (wie B-790); Treiberfehler der GPU fällt laut B-790 › Notizen nicht darunter.

## Akzeptanzkriterien

- **AC-01** Test: Umschalten ändert den Zustand und zeichnet im jeweils anderen Modus.
- **AC-02** Test: Anzeige aus → keine Zeitnahme im Bildpfad.
- **AC-03** Test: Anzeige an → Bildrate und Rechenzeit stehen im Frame-Text.

## Offene Fragen

Entschieden autonom, Claude, 2026-11-12: Tasten F3 (Fluss) und F2 (Leistungsanzeige), Werte dauerhaft in den Einstellungen (`_settings.json`); kein Eintrag im Optionen-Menü, weil das ein Entwickler-Schalter ist und die Optionen aus B-800/B-809 nicht wächst. Begründung: „Wert bleibt erhalten“ verlangt Speichern; das Menü ist Bereich von NG13.

## Notizen

Aus B-790 › Notizen (NG11, 2026-11-11).
