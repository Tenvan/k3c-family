# B-1222 · Spieltest-Modus fragt am Spielende drei Fragen je Spieler per Pad ab

- **Domäne:** CLI
- **Typ:** Idee
- **Prio:** niedrig
- **Umgebung:** offline
- **Status:** offen
- **Sprint:** WZ66
- **Projekt:** WZG
- **Erstellt:** 2026-11-22
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –
- **Duplikat von:** –
- **Ersetzt:** –

## Ausgangslage

Gesprochenes Feedback (WZ64) ist reich, aber ungleich: Wer viel redet, zählt mehr. B-151 (Archiv) hatte einen Spieleabend-Fragebogen auf Papier. Im Spiel gibt es keine Abfrage.

## Ziel

Nach jedem Spieltest liegt je Spieler eine kurze, vergleichbare Bewertung vor, ohne Papier und ohne Aufstehen.

## Beteiligte und Zielgruppen

Spieler am Sofa (bis 2, eigene Pads), Ralf (🧑) bei der Auswertung.

## Anforderungen

- Nur mit `--playtest`: Beim Verlassen des Spiels (Menü Beenden oder Ende der Partie) erscheint vor dem Schließen der Fragebogen.
- Drei Fragen, jeder Spieler antwortet in seiner Ansicht mit eigenem Pad: Spaß (1–5 Sterne), Schwierigkeit (zu leicht / passt / zu schwer), Nochmal spielen? (ja / nein).
- B überspringt die aktuelle Frage (B = zurück im Menü), Start überspringt den ganzen Bogen.
- Ergebnis nach `<ordner>/survey.json` mit `ts`, Antworten je Spieler (fehlende als `null`).
- Pad- und Tastaturbedienung, gut lesbar am TV; Datei ≤ 400 Zeilen, Funktion ≤ 60 Zeilen.

## Nicht-Ziele

- Freitext-Eingabe (dazu ist die Sprachaufnahme da).
- Auswertung über mehrere Sessions (später, falls gebraucht).

## Regeln und Einschränkungen

- B geht im Menü zurück; im laufenden Spiel bleibt B unbelegt.
- Jede Mechanik funktioniert mit 2 Spielern gleichzeitig.

- **Bereich:** `rust/crates/k3c-game/src/playtest/**`, `rust/crates/k3c-game/src/menu/**`
- **Abhängig von:** B-1220
- **Braucht 🧑:** Freigabe
- **Größe:** S
- **Sperre:** –

## Beispiele

- Zwei Spieler beenden die Partie → beide sehen den Bogen in ihrer Ansicht, Spieler 1 gibt 4 Sterne/passt/ja, Spieler 2 drückt Start → `survey.json` mit Antworten von Spieler 1 und `null` für Spieler 2.

## Ausnahme- und Fehlerfälle

- Spiel stürzt ab oder wird hart beendet → kein Bogen, kein `survey.json`; die Workbench zeigt „ohne Fragebogen“.
- Ein Spieler antwortet nicht → nach 60 s ohne Eingabe schließt der Bogen mit `null`.

## Akzeptanzkriterien

- **AC-01** Unit-Test: `survey.json` hat das Format oben, Überspringen ergibt `null`.
- **AC-02** Unit-Test: ohne `--playtest` erscheint kein Bogen.
- **AC-03** Am TV mit 2 Pads beantworten beide Spieler den Bogen gleichzeitig (🧑 prüft im Spiel).
- **AC-04** `check_run task:check:diff` grün.

## Offene Fragen

keine

## Notizen

Vorbild B-151 (Spieleabend-Fragebogen, Archiv).
