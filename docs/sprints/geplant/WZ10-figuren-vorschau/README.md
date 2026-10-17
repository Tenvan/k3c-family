# WZ10 · DEV · Abspiel-Bausteine und Monarch-Vorschau im Asset-Studio

- **Status:** geplant
- **Projekt:** WZG
- **Domäne:** DEV
- **Reife:** bereit
- **Tickets:** B-463
- **Start-Commit:** – (wird beim Aktivieren gesetzt: `git rev-parse --short origin/develop`)
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

`B-463 › Ausgangslage`: Der Monarch (4 Spielerfarben, 13 Reittiere, 24 Skills) ist nur statisch in `figuren.html`/`aufstellung.html` (Canvas ohne Phaser) zu sehen; Reiten, Sprinten, Schlagen und Effekte nur im Spiel, und dort nur mit dem Standard-Reittier. Eine gemeinsame Abspiel-Basis für Studio-Seiten gibt es noch nicht. Voraussetzung: Asset-Studio mit Aliasen `@game`/`@data` aus WZ6 sowie Phaser und Bühne aus WZ7.3 (B-460).

Stand der Teilung (Entscheidung von 🧑 am 2026-10-17): Der frühere Sprint WZ10 (Monarch, Gegner, Bosse, Entfernen der Referenzseiten) ist geteilt. WZ10 bleibt bei den gemeinsamen Abspiel-Bausteinen und der Monarch-Seite (B-463); die Gegner-Seite, die Bosse und das Entfernen von `figuren.html`/`aufstellung.html` stehen in WZ11 (B-464).

## Ziel

Nach WZ10 spielt das Asset-Studio den Monarchen (immer beritten) mit den Animationen und Effekten des Spiels ab, zeigt alle 13 Reittiere, Skills, Münzen, Getroffen/Tod/Respawn und die vier Spielerfarben nebeneinander, markiert jede fehlende Darstellung als Lücke und stellt die Abspiel-Bausteine bereit, die WZ11 für die Gegner-Seite wiederverwendet.

Am Ende sichtbar: die Monarch-Seite im Fenster „Asset-Studio“ neben Grafik und Sound; `figuren.html` und `aufstellung.html` sind noch vorhanden (Entfernen in WZ11); `task check:all` und CI grün; von 🧑 am PC abgenommen.

## Beteiligte und Zielgruppen

`B-463 › Beteiligte und Zielgruppen`. Agenten setzen die Sessions autonom um (DEV), ein getrennter Reviewer prüft in der Review-Session, 🧑 nimmt am PC ab und entscheidet über Folge-Tickets aus den Lücken (GRA, CLI). Die Gegner-Seite (WZ11) ist Nutznießer der Abspiel-Bausteine.

## Anforderungen

`B-463 › Anforderungen`. Sprint-eigen:

- Abspiel-Bausteine (Steuerung, Maßstab, Reittier-Lader, Lücken-Markierung) liegen an einer Stelle im Studio-Frontend. Die Monarch-Seite importiert sie; WZ11 importiert sie für die Gegner-Seite. Die Bausteine sind deshalb nicht auf den Monarchen zugeschnitten: Sie nehmen Sheet, Animation sowie `tint`, `scale`, `alpha` und `y` als Parameter und tragen keine Monarch-Annahmen.
- Die Phaser-Bühne gibt es schon aus WZ7.3 (B-460); WZ10.1 nutzt sie wieder und führt weder Phaser noch eine zweite Bühne ein.
- Die Monarch-Seite erscheint als eigene Seite in der Navigation des Asset-Studios neben Grafik und Sound (WZ6.2).
- Der Monarch erscheint nur beritten; Laufen und Sprinten zu Fuß gibt es nicht und ist keine Lücke.
- Die Lücken der Monarch-Seite sind im Ergebnis der Review-Session als Liste zusammengefasst, damit 🧑 Folge-Tickets (GRA, CLI) entscheiden kann.

## Nicht-Ziele

`B-463 › Nicht-Ziele`. Dazu:

- Gegner-Seite, Bosse und ihre Lücken (Warnkreis, Boss-Leiste, Phasen-Text, Event-Banner): WZ11 (B-464), Anbindung der Boss-Anzeigen B-469.
- Entfernen von `figuren.html` und `aufstellung.html` samt Skripten, Kacheln, Texten und Verweisen (B-463/AC-13) und der Nachweis, dass jede Karte einen Ort im Studio hat (B-463/AC-14): WZ11. Bis dahin bleiben beide Seiten unverändert.
- Ort für Truppen und ungenutzte Sheets (`B-463 › Offene Fragen` 4): gehört zum Entfernen, siehe WZ11.
- Änderungen an `src/scenes/`, `src/core/`, `src/model/` (stattdessen CLI-Ticket).
- Phaser oder die Bühne einführen (WZ7.3).
- Folge-Tickets aus den Lücken umsetzen.
- Glossar-Pflege während der Sessions (Begriffe kommen vor der Freigabe ins Glossar).

## Regeln und Einschränkungen

`B-463 › Regeln und Einschränkungen`. Für den ganzen Sprint:

- **Domäne DEV** in allen Sessions (`tools/k3c-studio/`, `src/tools/`); Spiel-Code nur lesend über `@game`.
- **Abhängigkeiten:** WZ6 erledigt; WZ7.3 erledigt (Phaser und Bühne, B-460; B-463 ist nicht Eigentümer der Abhängigkeit); WZ10.1 vor WZ10.2 und WZ10.3. WZ11 hängt von WZ10 ab (Abspiel-Bausteine aus WZ10.1, Monarch-Seite für den Vergleich und für B-463/AC-14).
- **Session-Größe:** je Session eine Domäne, ein Commit, ≤ ~400 geänderte Code-Zeilen; Code-Review vor jedem Commit.
- **Sprintgröße:** 5 Sessions (3 Umsetzung, Review, Abnahme) liegen im Richtwert 3–6; die Teilung ist entschieden (siehe Offene Fragen).
- **Prüfung:** Vitest im Studio-Frontend über `task check:all`; Fensterprüfungen nur mit Freigabe von 🧑 für den Lauf. Die Review-Session läuft nie im selben Lauf wie eine Umsetzung.

## Beispiele

`B-463 › Beispiele`. Dazu:

- Entwicklerseite (`dev.html`) nach WZ10 → unverändert mit den Kacheln „Unsere Aufstellung“ und „Alle Figuren“; sie entfallen erst mit WZ11.3.
- Monarch-Seite öffnen → die Seite steht in der Navigation neben Grafik und Sound, `grafiken.html` und `lizenzen.html` funktionieren unverändert.
- Reiten › Einhorn, Spielerfarbe 2 und Sprinten › `horse` wie in `B-463 › Beispiele`.

## Ausnahme- und Fehlerfälle

`B-463 › Ausnahme- und Fehlerfälle`. Dazu:

- Bühne aus WZ7.3 nicht vorhanden → WZ10.1 startet nicht, die Session setzt `Status: blockiert`.
- Ein Baustein braucht eine Änderung unter `src/scenes/` oder `src/model/` (z. B. weil eine Spiel-Funktion ohne `GameScene` nicht zeichnet) → Session `blockiert`, CLI-Ticket statt Änderung dort.
- Ein Baustein trägt eine Monarch-Annahme, die die Gegner-Seite nicht erfüllen kann → in WZ10 bereinigen (Parameter statt Festwert), nicht in WZ11 kopieren.

## Akzeptanzkriterien

Die Kriterien von B-463 behalten ihre Nummer als AC-01 bis AC-16; AC-13 und AC-14 gehören zum Entfernen der Referenzseiten und laufen in WZ11. AC-17 ist die Abnahme.

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
- **AC-13** `B-463/AC-13` verschoben nach WZ11 (dort AC-18): `figuren.html`/`aufstellung.html` samt Skripten, Kacheln, Texten und Verweisen entfernt. In WZ10 nicht zu erfüllen.
- **AC-14** `B-463/AC-14` verschoben nach WZ11 (dort AC-19): jede Karte der alten Seiten hat vorher einen Ort im Studio. In WZ10 nicht zu erfüllen.
- **AC-15** `B-463/AC-15` Vitest der Monarch-Logik in `task check:all` und CI.
- **AC-16** `B-463/AC-16` Monarch-Seite nutzt die Phaser-Bühne aus WZ7.3 und führt keine eigene Phaser-Abhängigkeit ein (Review, `git diff` der `package.json` des Studios).
- **AC-17** 🧑 hat die Monarch-Seite im Asset-Studio am PC abgenommen und die Lücken-Liste gesehen (Abnahme-Session).

## Offene Fragen

Entschieden von 🧑 am 2026-10-17 (Chat): `B-463 › Offene Fragen` 1, 2 und 3 (Zustände, Technik, Ersatz der Referenzseiten), 6 (Monarch immer beritten) und 7 (Phaser und Bühne aus B-460/WZ7.3).

**Sprintgröße:** entschieden (geteilt, 🧑 am 2026-10-17). WZ10 = Abspiel-Bausteine und Monarch-Seite (B-463); WZ11 = Gegner-Seite nach Biom, Bosse mit Lücken für Boss-Effekte (B-464, Anbindung B-469) und Entfernen von `figuren.html`/`aufstellung.html` (WZ11 hängt von WZ10 ab). Die frühere Option A (ein Sprint mit 8 Sessions) entfällt.

Offen (entscheidet 🧑 vor der Freigabe, nicht blockierend, Vorschlag im Ticket):

- `B-463 › Offene Fragen` 5: Tempo-Stufen 0,5×, 1×, 2× (AC-01).

`B-463 › Offene Fragen` 4 (Ort für Truppen und ungenutzte Sheets) betrifft das Entfernen der Referenzseiten und steht jetzt unter `WZ11 › Offene Fragen`.

**Neue Begriffe ins Glossar vor der Freigabe:** Monarch-Seite, Abspiel-Baustein, Lücke, Spielerfarbe (Variante des Monarchen je Spieler-Index), Kennfarbe (Farbe des Spielers aus `PLAYER_COLORS`). Gegner-Seite, Warnkreis, Boss-Leiste und Event-Banner gehören zu WZ11.

## Sessions

| Nr. | Datei | Typ | Agent | Status |
|---|---|---|---|---|
| WZ10.1 | `WZ10.1-abspiel-bausteine.md` | Umsetzung | autonom | offen |
| WZ10.2 | `WZ10.2-monarch-seite-bewegung.md` | Umsetzung | autonom | offen |
| WZ10.3 | `WZ10.3-monarch-seite-aktionen.md` | Umsetzung | autonom | offen |
| WZ10.4 | `WZ10.4-review.md` | Review | autonom | offen |
| WZ10.5 | `WZ10.5-abnahme.md` | Review | Mensch | offen |

- WZ10.1 (DEV) Abspiel-Bausteine auf der Bühne aus WZ7.3 (kein neues Phaser): Spiel-Animationen, Steuerung, Maßstab, Reittier-Lader, Lücken-Markierung, als Parameter statt Monarch-Festwerte, damit WZ11 sie nutzt (AC-02, AC-11, AC-16)
- WZ10.2 (DEV) Monarch-Seite: Laufen und Sprinten beritten, Reiten je Reittier, Spielerfarben mit Kennfarbe, Grafik-Auswahl (AC-01, AC-03, AC-04, AC-10, AC-12, AC-15)
- WZ10.3 (DEV) Monarch-Seite: Schlagen, Skills, Münzen, Getroffen/Tod/Respawn (AC-05, AC-06, AC-07, AC-08, AC-09, AC-15)
- WZ10.4 (DEV) Review, autonom, mit Lücken-Liste der Monarch-Seite für 🧑 (alle; AC-13 und AC-14 laufen in WZ11, AC-17 in WZ10.5)
- WZ10.5 (DEV) Abnahme am PC, Agent: Mensch (AC-17)

Bei `Reife: Entwurf` genügen Stichpunkte; vor dem Aktivieren wird jede Session als eigene Datei nach `docs/vorlagen/session.md` geschrieben.

## Abnahme

Wird von der Review-Session (Doku-Sprint: letzte Session) ausgefüllt, höchstens fünf Zeilen: Datum, Kriterien
(Verweis auf Session-Ergebnisse), behobene Befunde, neue Tickets. Bis dahin `–`.
