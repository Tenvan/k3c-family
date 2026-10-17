# WZ10 · DEV · Monarch-, Gegner- und Boss-Vorschau im Asset-Studio

- **Status:** geplant
- **Projekt:** WZG
- **Domäne:** DEV
- **Reife:** Entwurf
- **Tickets:** B-463, B-464
- **Start-Commit:** – (wird beim Aktivieren gesetzt: `git rev-parse --short origin/develop`)
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

`B-463 › Ausgangslage` und `B-464 › Ausgangslage`: Monarch (4 Spielerfarben, 13 Reittiere, 24 Skills), 17 Gegnertypen und 6 Bosse sind nur statisch in `figuren.html`/`aufstellung.html` (Canvas ohne Phaser) zu sehen; Reiten, Sprinten, Schlagen, Effekte und Boss-Anzeigen nur im Spiel, und dort nur das Standard-Reittier. Warnkreis, Boss-Leiste, Phasen-Text und Event-Banner zeichnet das Spiel erst nach K5.1 (B-132); WZ10 zeigt sie als Lücke, die Anbindung ist B-469. Voraussetzung: Asset-Studio mit Aliasen `@game`/`@data` aus WZ6 sowie Phaser und Bühne aus WZ7.3 (B-460).

## Ziel

Nach WZ10 spielt das Asset-Studio Monarch (immer beritten), Reittiere, Gegner und Bosse mit den Animationen und Effekten des Spiels ab, markiert jede fehlende Darstellung als Lücke (auch Warnkreis, Boss-Leiste, Phasen-Text und Event-Banner bis B-469), und `figuren.html`/`aufstellung.html` sind entfernt.

Am Ende sichtbar: Monarch- und Gegner-Seite im Fenster „Asset-Studio“ neben Grafik und Sound, Entwicklerseite ohne die Kacheln „Unsere Aufstellung“ und „Alle Figuren“, `task check:all` und CI grün, von 🧑 am PC abgenommen.

## Beteiligte und Zielgruppen

`B-463 › Beteiligte und Zielgruppen`, `B-464 › Beteiligte und Zielgruppen`. Agenten setzen die Sessions autonom um (DEV), ein getrennter Reviewer prüft in der Review-Session, 🧑 nimmt am PC ab und entscheidet über Folge-Tickets aus den Lücken.

## Anforderungen

`B-463 › Anforderungen`, `B-464 › Anforderungen`. Sprint-eigen:

- Abspiel-Bausteine (Steuerung, Maßstab, Reittier-Lader, Lücken-Markierung) liegen an einer Stelle im Studio-Frontend; Monarch- und Gegner-Seite importieren sie. Die Phaser-Bühne gibt es schon aus WZ7.3 (B-460); WZ10.1 nutzt sie wieder und führt weder Phaser noch eine zweite Bühne ein.
- Monarch und Gegner erscheinen als eigene Seiten in der Navigation des Asset-Studios neben Grafik und Sound (WZ6.2).
- Der Monarch erscheint nur beritten; Laufen und Sprinten zu Fuß gibt es nicht und ist keine Lücke.
- Warnkreis, Boss-Leiste, Phasen-Text und Event-Banner erscheinen je Boss nur dort, wo der Effekt zutrifft, als Lücke „Anzeige im Spiel fehlt (B-132, Anbindung B-469)“: Warnkreis nur bei Bossen mit Flächenschlag (`trollKing`, `crystalHeartWarden`), Phasen-Text nur beim Endboss mit Phasen (`crystalHeartWarden`), Boss-Leiste und Event-Banner bei jedem Boss (`goblinLeader`: kein Warnkreis, keine Warnkreis-Lücke). Ihre Anbindung ist nicht Teil von WZ10.
- Die Lücken beider Seiten sind am Sprint-Ende im Ergebnis der Review-Session als Liste zusammengefasst, damit 🧑 Folge-Tickets (GRA, CLI) entscheiden kann.

## Nicht-Ziele

`B-463 › Nicht-Ziele`, `B-464 › Nicht-Ziele`. Dazu: Änderungen an `src/scenes/`, `src/core/`, `src/model/` (stattdessen CLI-Ticket); Anzeigen von K5 (B-132) im Spiel; die Anbindung von Warnkreis, Boss-Leiste, Phasen und Event-Banner an die Gegner-Seite (B-469, nach K5.1 und WZ10); Phaser oder die Bühne einführen (WZ7.3); Folge-Tickets aus den Lücken umsetzen; Glossar-Pflege während der Sessions (Begriffe kommen vor der Freigabe ins Glossar).

## Regeln und Einschränkungen

`B-463 › Regeln und Einschränkungen`, `B-464 › Regeln und Einschränkungen`. Für den ganzen Sprint:

- **Domäne DEV** in allen Sessions; das Entfernen der Referenzseiten (WZ10.6) ist Grenzfall DEV wie WZ6.6/B-299 und läuft als eigene Session nach den beiden Seiten.
- **Abhängigkeiten:** WZ6 erledigt; WZ7.3 erledigt (Phaser und Bühne, B-460; B-463 ist nicht Eigentümer der Abhängigkeit); WZ10.1 vor allen Seiten-Sessions; WZ10.6 erst, wenn WZ10.2, WZ10.3, WZ10.4 und WZ10.5 die Inhalte belegen (B-463/AC-14). K5.1 ist keine Abhängigkeit, fehlender Zeichencode wird zur Lücke (B-464/AC-12); die Anbindung macht B-469.
- **Session-Größe:** je Session eine Domäne, ein Commit, ≤ ~400 geänderte Code-Zeilen; Code-Review vor jedem Commit.
- **Sprintgröße:** Mit 8 Sessions liegt der Sprint über dem Richtwert 3–6 (WZ6 hat 7, WZ7 hat 7); die Aufteilung steht unter Offene Fragen.
- **Prüfung:** Vitest im Studio-Frontend über `task check:all`; Fensterprüfungen nur mit Freigabe von 🧑 für den Lauf. Die Review-Session läuft nie im selben Lauf wie eine Umsetzung.

## Beispiele

`B-463 › Beispiele`, `B-464 › Beispiele`. Dazu: Entwicklerseite (`dev.html`) nach WZ10.6 → Abschnitt Entwicklung ohne „Unsere Aufstellung“ und „Alle Figuren“; `lizenzen.html` und `grafiken.html` funktionieren unverändert.

## Ausnahme- und Fehlerfälle

`B-463 › Ausnahme- und Fehlerfälle`, `B-464 › Ausnahme- und Fehlerfälle`. Dazu: Importiert nach WZ10.6 noch Code `allFigures` oder `lineup` (z. B. das Studio), bleiben die Exporte und die Session nennt den Grund im Ergebnis; findet WZ10.6 eine Karte der alten Seiten ohne Ort im Studio, setzt sie `Status: blockiert` mit Frage-Ticket statt zu entfernen.

## Akzeptanzkriterien

- **AC-01** `B-463/AC-01` jede Monarch-Animation einzeln (`run` beritten), Schleife, Einzelbild, Tempo.
- **AC-02** `B-463/AC-02` Animationsdefinitionen nur aus dem Spiel bzw. `data/sprites.json`.
- **AC-03** `B-463/AC-03` Reiten auf allen 13 Reittieren, Pose gleich `mountPose` (Test).
- **AC-04** `B-463/AC-04` Laufen und Sprinten nur beritten, kein Laufen oder Sprinten zu Fuß, keine Lücke dafür (Test der Vorschau-Liste).
- **AC-05** `B-463/AC-05` Schlagen mit `attack` und Effekt `swing`.
- **AC-06** `B-463/AC-06` alle Skills mit Linie, Stufe, Art, Lücken markiert.
- **AC-07** `B-463/AC-07` Preset-Skills ohne Eintrag (`backstab`, `smokeBomb`) als Lücke (Test).
- **AC-08** `B-463/AC-08` Münzen geben und aufheben, Wurf als Lücke.
- **AC-09** `B-463/AC-09` Getroffen, Tod, Respawn wie im Spiel, Lücken markiert.
- **AC-10** `B-463/AC-10` alle 4 Spielerfarben nebeneinander synchron, je mit Kennfarbe aus `PLAYER_COLORS`.
- **AC-11** `B-463/AC-11` Spielmaßstab, Höhe gleich `spriteTop` (Test).
- **AC-12** `B-463/AC-12` Grafik-Auswahl `monarch:*` und `mount:*`, Register unverändert.
- **AC-13** `B-463/AC-13` `figuren.html`/`aufstellung.html` samt Skripten, Kacheln, Texten und Verweisen entfernt, `task check` und `task build` grün.
- **AC-14** `B-463/AC-14` jede Karte der alten Seiten hat vorher einen Ort im Studio.
- **AC-15** `B-463/AC-15` Vitest der Monarch-Logik in `task check:all` und CI.
- **AC-16** `B-464/AC-01` alle 17 Gegnertypen gelistet (Test).
- **AC-17** `B-464/AC-02` Gruppen je Biom nach Tiefe, Gruppe „ohne Biom“ (Test).
- **AC-18** `B-464/AC-03` Gegner-Animationen einzeln mit `tint`, `scale`, `alpha`, `y`.
- **AC-19** `B-464/AC-04` fehlende Animation → `idle` und Lücke (Test).
- **AC-20** `B-464/AC-05` Getroffen `hit`, Sterben `kill`, eigene Animationen als Lücke.
- **AC-21** `B-464/AC-06` Merkmale an der Karte, ohne Darstellung als Lücke.
- **AC-22** `B-464/AC-07` Boss-Gruppe je Tiefe mit 6 Bossen (Test).
- **AC-23** `B-464/AC-08` Boss-Grafik wie im Spiel, Lücke „keine eigene Boss-Grafik“.
- **AC-24** `B-464/AC-09` verschoben nach `B-469/AC-01` (Warnkreis); in WZ10 nicht zu erfüllen, die Lücke deckt AC-27 ab.
- **AC-25** `B-464/AC-10` verschoben nach `B-469/AC-02` und `B-469/AC-03` (Boss-Leiste, Endboss-Phasen); in WZ10 nicht zu erfüllen, die Lücke deckt AC-27 ab.
- **AC-26** `B-464/AC-11` verschoben nach `B-469/AC-04` (Event-Banner für `bossSpawned`, `bossPhase`, `bossDefeated`); in WZ10 nicht zu erfüllen, die Lücke deckt AC-27 ab.
- **AC-27** `B-464/AC-12` je Boss die zutreffenden Effekte (Warnkreis nur bei Flächenschlag, Phasen-Text nur beim Endboss mit Phasen, Boss-Leiste und Event-Banner bei jedem Boss, zusammen 15 Einträge) als Lücke „Anzeige im Spiel fehlt (B-132, Anbindung B-469)“, kein Ersatz im Studio (Test, Review).
- **AC-28** `B-464/AC-13` Spielmaßstab, Monarch-Vergleich zuschaltbar.
- **AC-29** `B-464/AC-14` gemeinsame Abspiel-Bausteine und Bühne aus WZ7.3, je nur einmal im Studio.
- **AC-30** `B-464/AC-15` Grafik-Auswahl je Gegner-ID und `boss`, Register unverändert.
- **AC-31** `B-464/AC-16` Vitest der Gegner-Logik in `task check:all` und CI.
- **AC-32** 🧑 hat Monarch- und Gegner-Seite im Asset-Studio am PC abgenommen und die Lücken-Liste gesehen (Abnahme-Session).
- **AC-33** `B-463/AC-16` Monarch-Seite nutzt die Phaser-Bühne aus WZ7.3 und führt keine eigene Phaser-Abhängigkeit ein (Review, `git diff` der `package.json` des Studios).

## Offene Fragen

Entschieden von 🧑 am 2026-10-17 (Chat): `B-463 › Offene Fragen` 1, 2 und 3 (Zustände, Technik, Ersatz der Referenzseiten), 6 (Monarch immer beritten) und 7 (Phaser und Bühne aus B-460/WZ7.3) sowie `B-464 › Offene Fragen` 1, 2, 3, 4 (Boss-Effekte als Lücke, Anbindung in B-469) und 5 (Bühne aus WZ7.3).

Offen (entscheidet 🧑 vor der Freigabe, nicht blockierend, Vorschläge in den Tickets):

- `B-463 › Offene Fragen` 4: Ort für Truppen und ungenutzte Sheets.
- `B-463 › Offene Fragen` 5: Tempo-Stufen.
- **Sprintgröße:** WZ10 hat 8 Sessions und liegt über dem Richtwert 3–6. Option A: ein Sprint mit 8 Sessions (wie WZ6 und WZ7 mit 7, weil Review und Abnahme getrennt sind). Option B: Monarch und Gegner in zwei Sprints, etwa WZ10 mit Abspiel-Bausteinen und Monarch-Seite (WZ10.1, WZ10.2, WZ10.3, dazu eigene Review und Abnahme) und ein weiterer Sprint mit Gegner-Seite, Bossen und dem Entfernen der Referenzseiten (WZ10.4, WZ10.5, WZ10.6, dazu eigene Review und Abnahme); das Entfernen läuft dabei im zweiten Sprint, weil es beide Seiten belegt haben muss (B-463/AC-14). Vorschlag der Planung: Option B, weil jeder Sprint dann nach einer Abnahme am PC einzeln fertig ist; bei Option B ist B-463 dem ersten, B-464 dem zweiten Sprint zuzuordnen und die Sprint-Nummer des zweiten noch zu vergeben.
- **Neue Begriffe ins Glossar vor der Freigabe:** Monarch-Seite, Gegner-Seite, Abspiel-Baustein, Lücke, Spielerfarbe (Variante des Monarchen je Spieler-Index), Kennfarbe (Farbe des Spielers aus `PLAYER_COLORS`), Warnkreis, Boss-Leiste, Event-Banner.

## Sessions

- WZ10.1 (DEV) Abspiel-Bausteine auf der Bühne aus WZ7.3 (kein neues Phaser): Spiel-Animationen, Steuerung, Maßstab, Reittier-Lader, Lücken-Markierung (AC-02, AC-11, AC-29, AC-33)
- WZ10.2 (DEV) Monarch-Seite: Laufen und Sprinten beritten, Reiten je Reittier, Spielerfarben mit Kennfarbe, Grafik-Auswahl (AC-01, AC-03, AC-04, AC-10, AC-12, AC-15)
- WZ10.3 (DEV) Monarch-Seite: Schlagen, Skills, Münzen, Getroffen/Tod/Respawn (AC-05, AC-06, AC-07, AC-08, AC-09, AC-15)
- WZ10.4 (DEV) Gegner-Seite: Gegnertypen nach Biom, Merkmale, Monarch-Vergleich, Grafik-Auswahl (AC-16, AC-17, AC-18, AC-19, AC-20, AC-21, AC-28, AC-29, AC-30, AC-31)
- WZ10.5 (DEV) Bosse je Tiefe mit Name, Art, Fähigkeit, Phasen und den Lücken je Boss für die zutreffenden Effekte (Warnkreis, Boss-Leiste, Phasen-Text, Event-Banner); Anbindung später in B-469 (AC-22, AC-23, AC-27, AC-31)
- WZ10.6 (DEV) `figuren.html` und `aufstellung.html` samt Skripten, Kacheln, Texten und Verweisen entfernen (AC-13, AC-14)
- WZ10.7 (DEV) Review, autonom, mit Lücken-Liste für 🧑 (alle; AC-24, AC-25 und AC-26 sind nach B-469 verschoben)
- WZ10.8 (DEV) Abnahme am PC, Agent: Mensch (AC-32)

Bei `Reife: Entwurf` genügen Stichpunkte; vor dem Aktivieren wird jede Session als eigene Datei nach `docs/vorlagen/session.md` geschrieben. Die Aufteilung in zwei Sprints (Offene Fragen › Sprintgröße) verändert diese Liste nicht, nur ihre Zuordnung.

## Abnahme

Wird von der Review-Session (Doku-Sprint: letzte Session) ausgefüllt, höchstens fünf Zeilen: Datum, Kriterien
(Verweis auf Session-Ergebnisse), behobene Befunde, neue Tickets. Bis dahin `–`.
