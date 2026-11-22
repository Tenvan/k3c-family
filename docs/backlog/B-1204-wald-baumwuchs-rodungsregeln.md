# B-1204 · Oberfläche hat vollen Baumwuchs mit Lichtungen; gerodet wird von den Rändern der Waldstücke

- **Domäne:** SIM
- **Typ:** Idee
- **Prio:** mittel
- **Umgebung:** offline
- **Status:** offen
- **Sprint:** WD1
- **Projekt:** WIR
- **Erstellt:** 2026-11-21
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** autonom, Claude, 2026-11-21
- **Duplikat von:** –
- **Ersetzt:** –

## Ausgangslage

Die Oberfläche (Biom `forest`, Tiefe 0) hat Bäume nur im Chunk-Typ `forest` (32–42 je 50-Unit-Chunk, Gewicht 12 von 15); `clearing`, `rocks`, `stream` und Ereignis-Chunks (Truhe, Camp, Hafen, Portal, Ausgang) haben 0–8. Jeder Baum ist einzeln markierbar (`node_markable`, `rust/crates/k3c-sim/src/state/node_mark.rs`). Grill-Runde Ralf 2026-11-21.

## Ziel

Die Oberfläche ist im Standard ein dichter Wald; Truhen, Camps, Häfen, Portale und der Ausgang liegen in baumfreien Lichtungen; Wald wird von den Rändern der Waldstücke her abgetragen statt beliebig gepickt. Am Ende sichtbar: im Debug-Overlay grün schraffierte Waldstücke mit den markierbaren äußeren 5 Bäumen.

## Beteiligte und Zielgruppen

Ralf entscheidet und nimmt am PC mit Pad ab; Hauptrechner (First Worker): Level, Sim, Anzeige.

## Anforderungen

- **AC-01** Dichte: Auf der Oberfläche hat jeder Chunk-Typ (auch Ereignis-Chunks) die volle Dichte wie `forest` (ca. 32–42 je 50 Units). Hub-Streifen und Lichtungen (AC-02) bleiben baumfrei.
- **AC-02** Hub und Lichtungen: Im Hub-Chunk steht kein Baum bis kurz vor dem ersten Mauer-Bauplatz links und rechts. Truhe, Camp, Hafen, Portal und Ausgang haben je eine baumfreie Lichtung (Startwert Radius 6 Units, in `data/`), die nie zuwächst; die Truhe lässt sich dort immer öffnen.
- **AC-03** Waldstück: Bäume mit Lücken höchstens 3 Units bilden ein Waldstück; Lichtungen trennen es nicht, nur der Hub-Streifen trennt links und rechts. Ein Waldstück ist mindestens eine halbe Stufenbreite lang (Wert in `data/`, je Welt übersteuerbar).
- **AC-04** Äußere fünf: In einem Waldstück sind nur die 5 äußersten Bäume je Seite (links, rechts) zum Roden markierbar; fällt einer, rückt der nächste nach. Einzelne Bäume und kleine Gruppen (kürzer als die Mindestlänge) sind frei markierbar, aber höchstens 5 zugleich je Insel-Stufe. Test mit 2 Spielern.
- **AC-05** Anzeige: Waldstücke als grün schraffierte Fläche, markierbare äußere Bäume hervorgehoben, im Debug-Overlay im Spiel, in der Level-Ansicht der Workbench und im Cheat-Fenster (F4, Umschalter).
- Qualität: deterministisch (`engine/rng` entspricht `k3c-rng`), 2+ Spieler, nur mit nativen Regeln (Go-Parität und Golden unverändert).

## Nicht-Ziele

Balancing der Holzmengen und der Rodungskosten; Frame-Budget-Maßnahmen (Culling, Dichte-Obergrenze) vor der Messung; neue Grafik für Bäume oder Lichtungen; andere Biome als die Oberfläche (Tiefe 0).

## Regeln und Einschränkungen

Entscheidung 013 (neue Mechanik nur in Rust); Level-Generator und `data/biomes/*.json` sind mit Go geteilt (Golden `level-forest.json`), darum gilt jede Änderung nur hinter `native_rules` oder als Rust-only-Wert (`data/sizes.json`-Muster aus LA3). Alte Spielstände egal (Ralf). Markierung bleibt ein Zahlziel (Kosten wie bisher).

Planungsangaben für die Arbeitswellen:

- **Bereich:** `rust/crates/k3c-level/**`, `rust/crates/k3c-sim/**`, `rust/crates/k3c-data/**`, `rust/crates/k3c-game/src/devtools/**`, `rust/crates/k3c-game/src/props/**`, `rust/crates/k3c-devwin/**`, `rust/crates/k3c-devproto/**`, `tools/k3c-workbench/frontend/src/world/**`, `data/sizes.json`, `docs/rules/**`
- **Abhängig von:** –
- **Braucht 🧑:** Abnahme
- **Größe:** M
- **Sperre:** golden | spielstand

## Beispiele

- Oberfläche, Seed `k3c`: Hub-Streifen frei, links und rechts davon dichter Wald; Truhe in einer 12 Units breiten Lichtung mitten im Wald.
- Spieler markiert Baum Nr. 6 vom Rand eines Waldstücks: abgelehnt, keine Münze abgezogen; nach Rodung der ersten fünf rückt Baum 6 an den Rand und ist markierbar.
- Freistehende Baumgruppe von 4 Bäumen: alle markierbar; ein fünfter loser Baum auch, ein sechster erst, wenn ein markierter gefällt ist.

## Ausnahme- und Fehlerfälle

- Waldstück wird durch Rodung geteilt oder verkürzt: neu berechnen; ein Rest unter der Mindestlänge zählt dann als kleine Gruppe, schon markierte Bäume bleiben markiert.
- Markierter Baum, der durch Nachrücken nicht mehr am Rand steht: bleibt markiert.
- Ereignis-Ort ohne Platz für die Lichtung (Hub-Nähe, Rand): Lichtung wird gekürzt, nie ein Bauplatz verdeckt.
- Plantage-Bäume (Baumschule) und Camp-Wächter-Bäume brauchen keine Markierung und sind ausgenommen.

## Akzeptanzkriterien

- **AC-01** Level-Test über 40 Seeds: jeder Chunk außerhalb Hub-Streifen und Lichtungen hat die volle Dichte.
- **AC-02** Test: Hub-Streifen und alle Ereignis-Lichtungen baumfrei, Truhe erreichbar, über 40 Seeds.
- **AC-03** Test: Waldstück-Erkennung (Gap 3, Mindestlänge, Lichtungen trennen nicht, Hub trennt).
- **AC-04** Test: äußere 5 je Seite nachrückend; 5-zugleich-Limit für kleine Gruppen; 2 Spieler.
- **AC-05** Anzeige in Debug-Overlay, Workbench und F4 (Test der Geometrie; Sicht am PC durch 🧑).

## Offene Fragen

keine (Grill-Runde 2026-11-21).

## Notizen

**Annahmen (Claude, autonom, im Nachgang zu melden):**
1. „Kurz vor dem ersten Mauer-Bauplatz“ = Hub-Streifen reicht je Seite bis 4 Units vor den äußersten Mauer-Bauplatz der Hub-Layout-Daten; die nächste Seite ab dort Wald.
2. Waldstück-Mindestlänge = halbe Stufenbreite, als Wert in `data/` (`sizes.json`-Muster), je Welt übersteuerbar.
3. „Seite“ eines Waldstücks = linkes und rechtes Ende nach x; Nachrücken nach jeder Rodung.
4. Das 5-zugleich-Limit zählt markierte, noch stehende Einzelbäume je Insel-Stufe über alle Spieler zusammen.
5. Abgelehnte Markierung kostet nichts und zeigt den Hinweis „nur äußere Bäume“.
6. Lichtung Startwert Radius 6 Units; Lichtungen zählen als Lücke (trennen das Waldstück nicht).
7. Dichte „voll“ = Bereich `tree` des Chunk-Typs `forest` (32–42) für alle Typen; `rocks`, `stream` behalten ihre Felsen zusätzlich.
8. Mit 600–700 Bäumen je Oberfläche: Frame-Budget wird nicht vorab geändert, Messung im Abnahme-Ticket (Entscheidung Ralf).
- Spieltest 2026-11-22-10, Befund f1: „Hier kann ich nicht mehr markieren, anderen Stelle auch noch welche markiert sind. Es sollte aber fünf in beiden Richtungen von links und rechts der Baumkette sollten markierbar sein.“ (Welt standard, Insel 1 Stufe 0, Tag 1 Tag, Oberwelt (Wald), Spieler 1. Ralf testet die Rodungsregel: erst „nur die ersten fünf Bäume“ (00:29), dann „ich glaube 5 insgesamt geht nur“ (01:13), gemeint war fünf je Baumreihe und Seite (01:18). Bei 01:22 bis 01:29 verweigert das Spiel das Markieren, weil an einer anderen Stelle schon markiert ist. Das widerspricht AC-04 von B-1204 (äußere fünf je Seite, das 5-zugleich-Limit nur für kleine Gruppen). Unklar bleibt, was bei 00:59 bis 01:06 zuerst nicht ging („doch, jetzt geht es“).)
- Spieltest 2026-11-22-10, Befund f2: „hier ist um dem Lager keine Lichtung. Das heißt, das Zelt steht hinter den Bäumen. Das ist natürlich auch nicht so gewollt.“ (Welt standard, Insel 1 Stufe 0, Tag 1 Tag (Dämmerung in 3:50), Oberwelt (Wald), Spieler 1. Das Bild zeigt das Zelt mit Lagerfeuer und zwei Bürger mitten in dichtem Wald, Bäume davor und dahinter. B-1204 AC-02 verlangt baumfreie Lichtungen für Camps und einen baumfreien Hub-Streifen; WD1.1 steht auf fertig. Prüfen, ob die Aufnahme aus einer EXE vor WD1.1 stammt oder die Lichtung fehlt.)
