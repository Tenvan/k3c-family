# B-972 · Auf jeder Insel wohnt eine besondere Figur der Welt mit einem einmaligen Angebot

- **Domäne:** REG
- **Typ:** Idee
- **Prio:** mittel
- **Umgebung:** offline
- **Status:** eingeplant
- **Sprint:** SI5
- **Projekt:** KMP
- **Erstellt:** 2026-11-15
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** 🧑 2026-11-15
- **Duplikat von:** –
- **Ersetzt:** –

## Ausgangslage

Ideenrunde k3c-07 (2026-11-15), Vorschlag von Claude, von 🧑 ausgewählt. Es gibt bereits den Händler als Besucher im Hub (tauscht Material gegen Gold, `docs/rules/buerger.md` § 1); eine Figur mit Eigenart der Welt gibt es nicht.

## Ziel

Auf jeder Insel wohnt abseits in einer tieferen Stufe eine besondere Figur der Welt (Schwertschmied, Priesterin, Walfänger). Gegen Münzen zieht sie in den Hub und errichtet dort ein einmaliges Bauwerk der Welt mit einem Dauerbonus.

## Beteiligte und Zielgruppen

Spieler: ein oder zwei Monarchen auf der Couch. Umsetzung: Claude in autonomen Sessions (SI5), Fachprüfung durch `k3c-welten`. Freigabe der Spec, Bestätigung der Defaults und Abnahme am TV: 🧑.

## Anforderungen

Dieses Ticket trägt **Regeltext und Engine**; Assets und Anzeige stehen in B-1001, die Auswahl im Welten-Editor in B-1003.

- **Angebot (🧑 2026-11-15): einmaliges Bauwerk.** Die Figur errichtet ein besonderes Gebäude der Welt (Sengoku Dojo, Nil Schrein, Polar Tranhütte). Klar abgegrenzt vom Händler (Material gegen Gold).
- **Ort (🧑 2026-11-15): abseits, in einer tieferen Stufe.** Die Figur lebt in einer eigenen kleinen Hütte weit weg vom Hub, erst ab Stufe 2; man muss sie suchen und den Weg sichern.
- **Bonus (🧑 2026-11-15): ein Dauerbonus je Welt.** Genau ein dauerhafter Vorteil auf dieser Insel: Dojo macht Kämpfer stärker, Schrein heilt Mauern über Nacht, Tranhütte lässt Fackeln und Sicht weiter reichen.
- **Bauplatz (🧑 2026-11-15): im Hub der Oberwelt.** Der Monarch zahlt beim Einsiedler, die Figur zieht in den Hub und baut dort auf einem festen Hub-Platz (`data/hub.json`).
- Einmalig je Insel; funktioniert mit 2 Spielern (wer zahlt, ist egal, der Bonus gilt für die Insel).

Ergänzt (Defaults, siehe Offene Fragen):

- Katalog in neuer Datei `data/hermits.json` (je Figur `id`, `building`, `price`, `buildSeconds`, `hp`, Bonus-Werte): `swordsmith` mit `dojo`, `priestess` mit `shrine`. Die Welt-Datei bekommt das optionale Feld `hermit` (Katalog-ID; fehlt es, hat die Welt keine Figur). `sengoku` wählt `swordsmith`, `nil` wählt `priestess`; `standard` und `britannien` bleiben ohne. Walfänger und Tranhütte kommen mit der Polar-Welt.
- Hütte: auf jeder Insel der Welt mit mindestens zwei Stufen genau eine, in Stufe 2 (Tiefe 1), am Rand, der am weitesten vom Abgang entfernt ist; Position deterministisch aus dem Insel-Seed, Level-Generator unverändert.
- Zahlung: Ein Monarch wirft an der Hütte `price` Münzen ein. Damit beginnt sofort der Bau auf dem festen Hub-Platz `hermitHall` (`data/hub.json` › `islandSites`); der Weg der Figur in den Hub ist nur Anzeige, sie kann nicht angegriffen werden.
- Bonus wirkt, sobald das Bauwerk steht, für die ganze Insel (alle Stufen): Dojo Schaden der Krieger und Schützen × `damageFactor`; Schrein heilt bei Tagesanbruch jede stehende Mauer um `wallHealFraction` ihrer vollen HP.
- Das Bauwerk ist ein Gebäude wie die anderen im Hub: zerstörbar und reparierbar; solange es zerstört ist, ruht der Bonus. Kein Ausbau.
- Ob bezahlt ist, ergibt sich aus dem Bauplatz (Bau begonnen oder Gebäude steht); kein neues Feld im Spielstand.
- Preis und Stärke vorläufig (Balancing später).

## Nicht-Ziele

Segen für eine Nacht, besondere Truppenart, Ausbau des Bauwerks in Stufen, frei gewählter Bauplatz, Besucher im Hub wie der Händler.

## Regeln und Einschränkungen

- **Bereich:** `docs/rules/buerger.md`, `docs/rules/materialien-gebaeude.md`, `docs/rules/welten.md`, `engine/sim/**`, `data/hermits.json`, `data/embed.go`, `data/hub.json`, `data/buildings.json`, `data/worlds/sengoku.json`, `data/worlds/nil.json`
- **Abhängig von:** –
- **Braucht 🧑:** Freigabe
- **Größe:** M
- **Sperre:** –
- Zwei Sessions: SI5.1 Regeltext (REG), SI5.2 Engine (SIM). Sim-Code im eigenen Worktree mit eigener Dev-Workbench (B-925, Agent `k3c-sim-dev`).
- Kein Eingriff in `engine/level/**`, kein neues Feld im Spielstand; braucht die Umsetzung eines von beiden, hält die Session an und meldet es (dann Sperre `golden` bzw. `spielstand`).
- Der neue Hub-Platz muss die Hub-Prüfung bestehen (`engine/sim/hub_validate_layout.go`). Keine Simulation, kein Balancing (gesperrt).

## Beispiele

- Sengoku, Insel 1: In Stufe 2 steht abseits die Hütte des Schwertschmieds. Monarch 2 wirft Münzen ein, der Schmied wandert in den Hub und baut dort den Dojo; ab dann sind die Kämpfer dieser Insel stärker.
- Nil: Nach dem Bau des Schreins haben beschädigte Mauern am Morgen mehr HP als am Abend.
- Insel 2 derselben Welt: Dort wohnt wieder ein Schwertschmied; der Dojo von Insel 1 gilt nicht mehr.

## Ausnahme- und Fehlerfälle

- Einsiedler wird auf dem Weg in den Hub angegriffen → kann nicht passieren, der Weg ist nur Anzeige (Bürger sterben nicht).
- Insel mit nur einer Stufe → keine Figur, kein Bauwerk.
- Zu wenig Münzen → keine Zahlung, nichts wird abgezogen.
- Zweite Zahlung nach der ersten → abgelehnt, die Hütte ist leer.
- Bauwerk zerstört → Bonus ruht bis zur Reparatur; die Figur verlangt kein zweites Mal Münzen.
- Stufe 2 geht verloren, bevor gezahlt wurde → die Hütte ist erst wieder erreichbar, wenn die Stufe zurückgewonnen ist.
- Welt-Datei nennt eine unbekannte Figur → Laden schlägt mit klarer Meldung fehl.

## Akzeptanzkriterien

- **AC-01** `docs/rules/buerger.md` § 1 beschreibt die besondere Figur (Ort, Zahlung, Abgrenzung zum Händler), `docs/rules/materialien-gebaeude.md` § 3 das Bauwerk je Welt mit Bonus, Platz und Zerstörung; Werte als vorläufig markiert.
- **AC-02** `data/hermits.json` enthält `swordsmith` und `priestess`, ist in `data/embed.go` eingebunden; `data/hub.json` hat den Platz `hermitHall`; Go-Tests laden beides, die Hub-Prüfung besteht.
- **AC-03** Die Welt-Datei kennt `hermit`; Go-Test: unbekannte ID wird abgelehnt, `sengoku` und `nil` haben ihre Figur, `standard` und `britannien` keine; `docs/rules/welten.md` § 2 und § 3 nennen das Feld.
- **AC-04** Go-Test: Insel mit zwei oder mehr Stufen hat genau eine Hütte in Tiefe 1 am fernen Rand, bei gleichem Seed an derselben Stelle; Insel mit einer Stufe hat keine.
- **AC-05** Go-Test Zahlung: Münzen werden abgezogen, der Bau im Hub beginnt; zweite Zahlung und zu wenig Münzen ändern nichts; egal, welcher von zwei Monarchen zahlt.
- **AC-06** Go-Test Dojo: Schaden der Kämpfer × `damageFactor`, solange das Bauwerk steht; zerstört → Grundwert.
- **AC-07** Go-Test Schrein: Mauern heilen bei Tagesanbruch um `wallHealFraction`, höchstens bis zur vollen HP.
- **AC-08** Go-Test: Speichern und Laden nach der Zahlung erhält Bau und Bonus ohne neue Spielstand-Version; der Zustand nennt Hütte, Figur unterwegs und Bauwerk für B-1001.
- **AC-09** `check_run task:check:diff` grün; `testdata/golden/` unverändert oder Änderung im Commit begründet.

## Offene Fragen

Alle mit Default entschieden (vorläufig, 🧑 bestätigt):

- Preis und Stärke? → Preis 20 Münzen, Bauzeit 20 s; Dojo `damageFactor` 1,15; Schrein `wallHealFraction` 0,25 (vorläufig, Balancing später).
- Welcher Hub-Platz? → Neuer fester Platz `hermitHall` in `islandSites`, ab Hub-Stufe 2; den freien Versatz bestimmt SI5.2 mit der Hub-Prüfung (Belegung ungeprüft).
- Insel mit nur einer Stufe? → Keine Figur.
- Katalog für neue Welten? → Ja, wie beim Wetter; Auswahl im Editor über B-1003.
- Tranhütte und Sichtregel? → Kommen mit der Polar-Welt samt Schneesturm; nicht Teil dieses Tickets.
- Angriff auf dem Weg? → Der Weg ist nur Anzeige, der Bau beginnt mit der Zahlung.

## Notizen

Grundzüge in der Ideenrunde k3c-07 (2026-11-15) mit 🧑 geklärt. 2026-11-15 von L zerlegt in B-972 (Regel und Engine), B-1001 (Assets und Anzeige) und B-1003 (Welten-Editor).
