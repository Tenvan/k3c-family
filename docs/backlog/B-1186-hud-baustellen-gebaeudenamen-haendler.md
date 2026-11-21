# B-1186 · HUD: Baustellen-Texte kleiner mit Icons und x/n, Gebäudename über dem Monarchen, Händler und Stände sichtbar

- **Domäne:** CLI
- **Typ:** Idee
- **Prio:** mittel
- **Umgebung:** offline
- **Status:** offen
- **Sprint:** HB1
- **Projekt:** RSU
- **Erstellt:** 2026-11-21
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** autonom, Claude (2026-11-21, Auftrag von 🧑)
- **Duplikat von:** –
- **Ersetzt:** –

## Ausgangslage

Texte an Baustellen und Gebäuden überladen das Bild: lange Beschriftungen, Ressourcen als Wort, Ausbau mit langem Text. Händler und seine Stände erscheinen nicht.

## Ziel

Baustellen lesen sich auf einen Blick: kleinere Texte, Ressourcen als Icons, Ausbau als x/n; steht ein Monarch vor einem Gebäude, steht dessen Name darüber; sonstige feste Texte an Gebäuden entfallen; Händler und Stände sind sichtbar.

## Beteiligte und Zielgruppen

Spieler am Sofa; 🧑 Ralf nimmt am PC ab.

## Anforderungen

- Text über allen Baustellen verkleinern.
- Ressourcen-Kosten als Icons statt Wort.
- Ausbau-Text auf x/n kürzen.
- Händler und seine Stände anzeigen (Ursache klären: nicht gezeichnet oder nicht erzeugt).
- Steht ein Monarch vor einem Gebäude, erscheint darüber der Gebäudename.
- Andere feste Texte bei Gebäuden ausblenden.

## Nicht-Ziele

Neue Gebäude, neue Regeln.

## Regeln und Einschränkungen

Rust-Spiel (`k3c-game`), jede Ansicht mit 2 Spielern gleichzeitig, Texte über die vorhandenen Text-Slots (kein Allokieren je Bild), Dateien ≤ 400 Zeilen.

Planungsangaben für die Arbeitswellen:

- **Bereich:** `rust/crates/k3c-game/src/hud/**`, `rust/crates/k3c-game/src/props/**`
- **Abhängig von:** –
- **Braucht 🧑:** Abnahme
- **Größe:** M
- **Sperre:** –

## Beispiele

Baustelle Turm: Icons Holz 3, Stein 2 statt Text; Ausbau „1/3“; Monarch davor: Name „Turm“ über dem Gebäude.

## Ausnahme- und Fehlerfälle

Zwei Monarchen vor demselben Gebäude: ein Name. Ohne Icon-Sheet: Rückfall auf Kurztext.

## Akzeptanzkriterien

- **AC-01** Baustellen-Texte kleiner, Kosten als Icons, Ausbau als x/n.
- **AC-02** Gebäudename über dem Gebäude, solange ein Monarch davor steht; andere feste Gebäudetexte entfallen.
- **AC-03** Händler und Stände sind im Spiel sichtbar (Test oder Aufnahme).
- **AC-04** Abnahme am PC durch 🧑.

## Offene Fragen

keine

## Notizen

Links, Messwerte, verworfene Ansätze. Darf leer bleiben (`–`).
