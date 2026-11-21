# B-1181 · Lichtquellen als Assets: animierte Flammen, Fackeln, Lampen und Feuerschalen mit Ankerpunkten an Gebäuden

- **Domäne:** PAP
- **Typ:** Idee
- **Prio:** mittel
- **Umgebung:** ?
- **Status:** offen
- **Sprint:** LA2
- **Projekt:** RSU
- **Erstellt:** 2026-11-21
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –
- **Duplikat von:** –
- **Ersetzt:** –

## Ausgangslage

Die Fackeln im Rust-Spiel sind Platzhalter: ein Holzstab und eine geschichtete Flamme aus Rechtecken (`world/lightdraw.rs`, `draw_torch_part`, `draw_flames`), gleich für alle Gebäude und Welten; sie sitzen an festen Versätzen aus `data/light.json` (nur Sim-Plätze). 🧑 2026-11-21 (Nachprüfung LA1.5): Es fehlt die Fackel-Atmosphäre.

## Ziel

Lichtquellen sind Papierschnitt-Assets: ein animiertes, größeres Fackelfeuer je Welt, dazu verschiedene Fackeln, Lampen und Feuerschalen, und jedes Gebäude trägt benannte Ankerpunkte für Lichter.

## Beteiligte und Zielgruppen

🧑 entscheidet über Look und Abnahme am PC; Agenten setzen über MCP `assets_*` um.

## Anforderungen

- Flamme: animiertes Sheet (Papierschnitt), größer als heute, je Welt eine Variante (Farbton, Form).
- Halter-Varianten: Fackel, Wandlampe, Laterne, Feuerschale; je Gebäudeart eine passende.
- Ankerpunkte: pro Gebäude-Asset benannte Punkte (Lage und Art des Lichts), die das Spiel statt der festen Versätze nutzt; keine Fackel auf Türen.
- Licht-Hof und Flamme folgen weiter dem Schein aus `light.json`; Sim-Daten bleiben (Go-Golden).

## Nicht-Ziele

Neue Lichtmechanik, Regeln.

## Regeln und Einschränkungen

Assets nur über `assets_*` (Entscheidung 005); Datei ≤ 400 Zeilen; 2 Spieler; Frame-Budget.

Planungsangaben für die Arbeitswellen:

- **Bereich:** `public/grafik/**`, `data/paper.json`, `rust/crates/k3c-game/src/world/lightdraw.rs`, `rust/crates/k3c-game/src/props/**`
- **Abhängig von:** LA1.5
- **Braucht 🧑:** Entscheidung | Abnahme
- **Größe:** L
- **Sperre:** –

## Beispiele

Taverne: Laterne neben der Tür; Burg: zwei Feuerschalen am Tor; Sengoku: Papierlaterne.

## Ausnahme- und Fehlerfälle

Gebäude ohne Ankerpunkte: kein Licht (statt Versatz).

## Akzeptanzkriterien

- **AC-01** Je Welt ein animiertes Flammen-Asset, angebunden und sichtbar nachts.
- **AC-02** Mindestens drei Halter-Arten, Ankerpunkte je Gebäude, Test: keine Lichtquelle liegt im Türbereich.
- **AC-03** Abnahme 🧑 am PC.

## Offene Fragen

🧑: Welche Halter-Art für welches Gebäude und welche Welt? Vor der Freigabe in Tickets je Teil zerlegen (Größe L).

## Notizen

Herkunft: LA1.5, Nachprüfung 2026-11-21. Sofortmaßnahme in LA1.5: Fackeln über den Gebäuden und unter den Figuren, weiter von der Tür (`glow.torchSpread`), Burg mit zwei Fackeln (`glow.castleTorches`), wärmerer Schein (`glow.warmth`), Flamme größer und flackernd. **Lane:** Asset-Änderungen plant Claude immer für den Second Worker (`Lane: zweit`, 🧑 2026-11-21): die Asset-Sessions dieses Tickets bekommen Lane zweit, committen und pushen dort; die Anbindung im Spiel (`world/lightdraw.rs`) bleibt beim Hauptrechner.
