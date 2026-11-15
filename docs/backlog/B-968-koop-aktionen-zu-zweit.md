# B-968 · Zwei Monarchen am selben Ort lösen gemeinsame Koop-Aktionen aus

- **Domäne:** REG
- **Typ:** Idee
- **Prio:** mittel
- **Umgebung:** offline
- **Status:** eingeplant
- **Sprint:** SI2
- **Projekt:** KMP
- **Erstellt:** 2026-11-15
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** 🧑 2026-11-15
- **Duplikat von:** –
- **Ersetzt:** –

## Ausgangslage

Ideenrunde k3c-07 (2026-11-15), Vorschlag von Claude, von 🧑 ausgewählt. Zusammenspiel der beiden Monarchen wird bisher nicht eigens belohnt.

## Ziel

Dinge, die nur gemeinsam gehen, belohnen das Zusammenspiel auf der Couch. Erste Koop-Aktion ist die **Doppel-Truhe**: Sie öffnet sich nur, wenn beide Monarchen gleichzeitig davorstehen, und gibt deutlich mehr Gold.

## Beteiligte und Zielgruppen

Spieler: zwei Monarchen auf der Couch (Solo verliert nichts). Umsetzung: Claude in autonomen Sessions (SI2). Freigabe der Spec, Bestätigung der Defaults und Abnahme am TV: 🧑.

## Anforderungen

Dieses Ticket trägt **Regeltext und Engine**; Asset und Anzeige stehen in B-998.

- **Erste Aktion (🧑 2026-11-15): Doppel-Truhe.** Besondere Truhe mit zwei Schlössern; sie öffnet sich nur, wenn beide Monarchen gleichzeitig davorstehen, und gibt deutlich mehr Gold als eine normale Truhe.
- **Solo (🧑 2026-11-15): normale Truhe.** Mit einem Spieler erzeugt der Level-Generator an der Stelle eine gewöhnliche Truhe; Solo verliert nichts, der Koop-Bonus läuft über Multiplikator (Basiswerte = Solo).
- **Beute (🧑 2026-11-15): Fontäne für beide.** Die Truhe wirft ihre Münzen als Fontäne aus (wie B-953), beide sammeln, was sie erwischen.
- Steht nur ein Monarch davor, bleibt die Truhe zu und zeigt, dass der zweite fehlt.
- Weitere Koop-Aktionen (gemeinsam bezahlen und bauen, gemeinsamer Angriff) sind spätere Kandidaten, nicht Teil dieses Tickets.

Ergänzt (Defaults, siehe Offene Fragen):

- Der Level-Generator bleibt unverändert. Welche Truhe das Kennzeichen „doppelt“ trägt, berechnet die Simulation deterministisch aus Insel-Seed, Stufe und Position der Truhe (`engine/rng`), im Mittel jede `doubleChest.everyNth`-te Truhe; das Kennzeichen wird nicht gespeichert.
- Das Kennzeichen wirkt nur, solange mindestens zwei Monarchen gesteuert werden; sonst verhält sich die Truhe wie eine normale (Öffnen allein, normales Gold).
- „Gleichzeitig davor“ heißt: zwei gesteuerte Monarchen derselben Stufe stehen im selben Tick im Aufheb-Radius der Truhe.
- Gold = normales Truhen-Gold × `doubleChest.goldFactor`, ausgeworfen über die vorhandene Fontäne (`engine/sim/economy_fountain.go`).
- Was eine normale Truhe dem Öffner zusätzlich gibt (Skill-Punkt, `chestSkillPoint`), bekommen beide Monarchen.
- Der Zustand nennt je wartender Doppel-Truhe, welcher Monarch davorsteht, damit B-998 das leuchtende Schloss zeichnen kann.
- Werte in `data/economy.json` › `doubleChest`, vorläufig (Balancing später).

## Nicht-Ziele

Gemeinsames Bauen, gemeinsamer Angriff, besondere Beute (Ausrüstung, Material), Truppe als zweiter Schlüssel.

## Regeln und Einschränkungen

- **Bereich:** `docs/rules/koop.md`, `engine/sim/**`, `data/economy.json`
- **Abhängig von:** –
- **Braucht 🧑:** Freigabe
- **Größe:** M
- **Sperre:** –
- Zwei Sessions: SI2.1 Regeltext (REG), SI2.3 Engine (SIM). Sim-Code im eigenen Worktree mit eigener Dev-Workbench (B-925, Agent `k3c-sim-dev`).
- Kein Eingriff in `engine/level/**` und kein neues Feld im Spielstand; braucht die Umsetzung eines von beiden, hält die Session an und meldet es (dann Sperre `golden` bzw. `spielstand`).
- Keine Simulation, kein Balancing (gesperrt); Koop-Werte nur über Multiplikator-Felder.

## Beispiele

- Koop: Monarch 1 steht an der Doppel-Truhe, ein Schloss leuchtet; Monarch 2 kommt dazu → Truhe springt auf, Fontäne, beide sammeln.
- Solo: an derselben Stelle steht eine normale Truhe.
- Koop, Spieler 2 verlässt das Spiel → die ungeöffnete Doppel-Truhe öffnet sich für Monarch 1 wie eine normale Truhe.

## Ausnahme- und Fehlerfälle

- Ein Spieler geht, während Doppel-Truhen ungeöffnet sind → sie verhalten sich wie normale Truhen; kommt wieder ein zweiter Spieler, sind sie wieder Doppel-Truhen.
- Monarch ohne Krone → zählt mit, solange er gesteuert wird.
- Die Monarchen stehen in verschiedenen Stufen → die Truhe bleibt zu.
- Mehr als zwei Spieler → zwei Monarchen genügen.
- Stufe ohne Truhe → keine Doppel-Truhe.

## Akzeptanzkriterien

- **AC-01** `docs/rules/koop.md` (neu) beschreibt Koop-Aktionen und die Doppel-Truhe in der Tabellenform der Regeldateien (Regel, Begründung, Daten, Zielkorridor), mit Kopf nach dem Muster der anderen Regeldateien; Werte als vorläufig markiert.
- **AC-02** `data/economy.json` hat `doubleChest` mit `everyNth`, `goldFactor`; ein Go-Test lädt und prüft die Werte (> 0).
- **AC-03** Go-Test Solo: Eine gekennzeichnete Truhe öffnet sich für einen Monarchen mit normalem Gold; `testdata/golden/` bleibt unverändert.
- **AC-04** Go-Test Koop: Steht ein Monarch davor, bleibt die Truhe zu und der Zustand nennt ihn als wartend.
- **AC-05** Go-Test Koop: Stehen beide davor, öffnet die Truhe, wirft Gold × `goldFactor` als Fontäne aus und beide erhalten den Zusatz der normalen Truhe.
- **AC-06** Go-Test: Sinkt die Zahl gesteuerter Monarchen auf eins, öffnet die Truhe allein; steigt sie wieder, wartet sie wieder auf zwei.
- **AC-07** Go-Test: Gleicher Seed → dieselben gekennzeichneten Truhen; ohne `math/rand`.
- **AC-08** `check_run task:check:diff` grün.

## Offene Fragen

Alle mit Default entschieden (vorläufig, 🧑 bestätigt):

- Wie viel Gold, wie viele Doppel-Truhen? → `goldFactor` 3, `everyNth` 3 (vorläufig, Balancing später).
- Was wird aus ungeöffneten Doppel-Truhen, wenn ein Spieler geht oder dazukommt? → Das Kennzeichen bleibt, wirkt aber nur bei zwei gesteuerten Monarchen.
- Mehr als 2 Spieler? → Zwei Monarchen reichen.
- Monarch ohne Krone? → Zählt mit.
- Wo steht die Regel? → Neue Datei `docs/rules/koop.md`, damit spätere Koop-Aktionen einen Ort haben.

## Notizen

Grundzüge in der Ideenrunde k3c-07 (2026-11-15) mit 🧑 geklärt. 2026-11-15 zerlegt in B-968 (Regel und Engine) und B-998 (Asset und Anzeige).
