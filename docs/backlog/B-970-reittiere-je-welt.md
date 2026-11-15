# B-970 · Jede Welt hat ein eigenes Reittier mit kleiner Eigenheit

- **Domäne:** REG
- **Typ:** Idee
- **Prio:** mittel
- **Umgebung:** offline
- **Status:** eingeplant
- **Sprint:** SI4
- **Projekt:** KMP
- **Erstellt:** 2026-11-15
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –
- **Duplikat von:** –
- **Ersetzt:** –

## Ausgangslage

Ideenrunde k3c-07 (2026-11-15), Vorschlag von Claude, von 🧑 ausgewählt. Heute reitet jeder Monarch immer das Standard-Pferd (`horse`, Tempo- und Sprintfaktor 1,0; `docs/rules/monarch.md` § 7, Glossar „Reittier“).

## Ziel

Jede Welt hat ein eigenes Reittier mit genau einer Stärke, die in dieser Welt zählt. Es steht auf der Insel an einem Gatter und wird mit Münzen gekauft.

## Beteiligte und Zielgruppen

Spieler: ein oder zwei Monarchen auf der Couch. Umsetzung: Claude in autonomen Sessions (SI4). Freigabe der Spec, Bestätigung der Defaults und Abnahme am TV: 🧑.

## Anforderungen

Dieses Ticket trägt **Regeltext und Engine**; Assets und Anzeige stehen in B-1000, die Auswahl im Welten-Editor in B-1003.

- **Eigenheit (🧑 2026-11-15): eine Stärke, passend zur Welt.** Kein Nachteil. Kamel (Nil) sprintet länger, Rentier (Polar) wird im Schnee nicht gebremst, das Sengoku-Pferd sprintet schneller.
- **Freischalten (🧑 2026-11-15): auf der Insel finden und bezahlen.** Das Tier steht irgendwo auf der Insel an einem Gatter; wer Münzen einwirft, steigt um. Jeder Monarch zahlt für sein eigenes.
- **Inselwechsel (🧑 2026-11-15): fährt mit, gilt für die Welt.** Ein gekauftes Reittier bleibt dem Monarchen bis zum Ende der Welt und belegt keinen Frachtplatz (B-967).
- Ohne Kauf reitet der Monarch weiter das Standard-Pferd (`horse`, Faktoren 1,0).
- Funktioniert mit 2 Spielern; die Monarchen können verschiedene Reittiere haben.

Ergänzt (Defaults, siehe Offene Fragen):

- Reittier-Katalog in `data/monarch.json` › `mounts` (je Eintrag `id`, `sprite`, `speedFactor`, `sprintFactor`, `price`); `mount` bleibt das Standard-Reittier. Die Welt-Datei bekommt das optionale Feld `mount` (Katalog-ID; fehlt es, hat die Welt kein eigenes Reittier und kein Gatter). `sengoku` wählt `sengokuHorse`, `nil` wählt `camel`; `standard` und `britannien` bleiben ohne. Das Rentier kommt mit der Polar-Welt.
- Gatter: auf jeder Insel der Welt genau eines, an der Oberfläche (Tiefe 0), mindestens `mountGate.minHubDistanceUnits` vom Hub entfernt; die Position berechnet die Simulation deterministisch aus dem Insel-Seed (`engine/rng`), der Level-Generator bleibt unverändert (Muster: `addShipSite` in `engine/sim/ship.go`).
- Kauf: Der Monarch wirft am Gatter `price` Münzen ein und reitet danach das Welt-Reittier; das Gatter bleibt für den anderen Monarchen. Wer es schon hat, kann nicht noch einmal zahlen.
- Das Reittier steht je Spieler im Zustand und im Spielstand; `MountOf` (`engine/sim/monarch_mount.go`) liefert es. Es bleibt bei Stufenwechsel, Inselwechsel, Verlust der Krone, Speichern und Laden; ein neues Spiel in einer anderen Welt beginnt wieder mit dem Standard-Pferd.
- Die Regel „das Reittier wird weder gekauft noch verloren“ in `docs/rules/monarch.md` § 7 wird entsprechend geändert.
- Preis und Stärke vorläufig (Balancing später).

## Nicht-Ziele

Nachteile je Reittier, Reittier als reine Grafik, Reittier als Fracht, Reittier als Belohnung nach dem Endgegner.

## Regeln und Einschränkungen

- **Bereich:** `docs/rules/monarch.md`, `docs/rules/welten.md`, `engine/sim/**`, `engine/room/**`, `data/monarch.json`, `data/worlds/sengoku.json`, `data/worlds/nil.json`
- **Abhängig von:** –
- **Braucht 🧑:** Freigabe
- **Größe:** M
- **Sperre:** spielstand
- Zwei Sessions: SI4.1 Regeltext (REG), SI4.3 Engine (SIM). Sim-Code im eigenen Worktree mit eigener Dev-Workbench (B-925, Agent `k3c-sim-dev`).
- Keine Rücksicht auf alte Spielstände. Keine Simulation, kein Balancing (gesperrt). Kein Eingriff in `engine/level/**`.

## Beispiele

- Nil, Insel 1: Monarch 1 findet das Kamel am Gatter, wirft Münzen ein und steigt um. Monarch 2 bleibt auf dem Standard-Pferd. Auf Insel 2 reitet Monarch 1 weiter das Kamel.
- Sengoku: Beide Monarchen kaufen nacheinander am selben Gatter; beide sprinten schneller.

## Ausnahme- und Fehlerfälle

- Zu wenig Münzen → kein Kauf, nichts wird abgezogen.
- Monarch hat das Welt-Reittier schon → Einwurf am Gatter wird abgelehnt.
- Später Beitritt → Standard-Pferd; das Gatter der aktuellen Insel steht auch ihm offen.
- Welt-Datei nennt eine unbekannte Reittier-ID → Laden schlägt mit klarer Meldung fehl.
- Welt ohne Feld `mount` → kein Gatter, Verhalten wie heute.
- Beide Monarchen werfen im selben Tick ein → beide kaufen, Reihenfolge nach Spieler-Index.

## Akzeptanzkriterien

- **AC-01** `docs/rules/monarch.md` § 7 beschreibt Katalog, Gatter, Kauf, Mitfahren und die Stärken; die Zeilen „immer beritten“ und „weitere Reittiere“ sind angepasst; Werte als vorläufig markiert.
- **AC-02** `data/monarch.json` hat `mounts` mit `sengokuHorse` und `camel` sowie `mountGate.minHubDistanceUnits`; Go-Test lädt und prüft (Faktoren ≥ 1, Preis > 0).
- **AC-03** Die Welt-Datei kennt `mount`; Go-Test: unbekannte ID wird abgelehnt, `sengoku` und `nil` haben ihr Reittier, `standard` und `britannien` keines; `docs/rules/welten.md` § 2 und § 3 nennen das Feld.
- **AC-04** Go-Test: Jede Insel einer Welt mit Reittier hat genau ein Gatter an der Oberfläche, im Mindestabstand zum Hub, bei gleichem Seed an derselben Stelle.
- **AC-05** Go-Test Kauf: Münzen werden abgezogen, `MountOf` liefert die Katalogwerte; zu wenig Münzen oder schon gekauft → keine Änderung.
- **AC-06** Go-Test: Zwei Monarchen haben unabhängige Reittiere; die Stärke wirkt nur beim Käufer.
- **AC-07** Go-Test: Das Reittier bleibt nach `NextIsland` und belegt keinen Frachtplatz.
- **AC-08** Go-Test: Das Reittier übersteht Speichern und Laden (`IslandSaveVersion` erhöht, Fixtures angepasst).
- **AC-09** `check_run task:check:diff` grün; `testdata/golden/` unverändert oder Änderung im Commit begründet.

## Offene Fragen

Alle mit Default entschieden (vorläufig, 🧑 bestätigt):

- Preis und Stärke? → Preis 12 Münzen; Sengoku-Pferd `sprintFactor` 1,2; Kamel `speedFactor` 1,1 (vorläufig, Balancing später).
- „Kamel sprintet länger“? → Der Sprint hat heute keine Ausdauer (geprüft: kein solches Feld in `engine/sim` und `data/monarch.json`). Default: Das Kamel ist im Dauerlauf schneller (`speedFactor`). Alternative für 🧑: Sprint-Ausdauer für alle Reittiere einführen (eigene Regel, eigenes Ticket).
- Wo steht das Gatter? → An der Oberfläche jeder Insel, mindestens 60 Units vom Hub.
- Schnee-Bremse und Rentier? → Kommen mit der Polar-Welt, die es noch nicht gibt; nicht Teil dieses Tickets.
- Katalog für neue Welten? → Ja, wie beim Wetter; Auswahl im Editor über B-1003.

## Notizen

Grundzüge in der Ideenrunde k3c-07 (2026-11-15) mit 🧑 geklärt. 2026-11-15 von L zerlegt in B-970 (Regel und Engine), B-1000 (Assets und Anzeige) und B-1003 (Welten-Editor).
