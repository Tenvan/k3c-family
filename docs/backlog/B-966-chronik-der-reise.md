# B-966 · Abgeschlossene Inseln hinterlassen auf der Seekarte ein Andenken und eine Zeile der Geschichte

- **Domäne:** REG
- **Typ:** Idee
- **Prio:** niedrig
- **Umgebung:** ?
- **Status:** offen
- **Sprint:** GS1
- **Projekt:** WZG
- **Erstellt:** 2026-11-15
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –
- **Duplikat von:** –
- **Ersetzt:** –

## Ausgangslage

Ideenrunde k3c-07 (2026-11-15), Vorschlag von Claude, von 🧑 ausgewählt. Die Seekarte (B-964) zeigt Inseln, die Geschichte (B-965) blendet nur kurz ein.

## Ziel

Jede abgeschlossene Insel hinterlässt im Diorama der Seekarte eine Trophäe des Endgegners, die Abschlusszeile der Geschichte und zwei Zahlen. Die Karte wird zum Tagebuch des Spielstands.

## Beteiligte und Zielgruppen

Wer spielt, entwickelt, betreibt oder entscheidet (🧑)? Keine Verantwortlichen erfinden.

## Anforderungen

- **Andenken (🧑 2026-11-15): Trophäe des Endgegners.** Auf der Insel-Miniatur steht ein kleines Zeichen des besiegten Endgegners (Maske, Horn, Banner) samt Fahne der Spieler; je Welt wenige Motive.
- **Inhalt (🧑 2026-11-15): Zeile plus zwei Zahlen.** Die Abschlusszeile der Geschichte (B-965), darunter klein: Tage bis zum Sieg und wer dabei war.
- **Zugang (🧑 2026-11-15): Seekarte und Spielstand-Auswahl.** Im Spiel auf der Seekarte am Hafen (B-964); zusätzlich zeigt die Spielstand-Auswahl im Menü die Karte mit den Trophäen als Vorschau.
- Die Chronik steht im Spielstand (Tage und Spieler je abgeschlossener Insel).
- Assets nur über MCP `assets_*`.

## Nicht-Ziele

Abbild der eigenen Burg je Ausbaustufe, ganze Geschichte der Insel zum Nachlesen, eigener Menüpunkt „Chronik“.

## Regeln und Einschränkungen

- **Bereich:** `native/k3c-game/hud/**`, `native/k3c-game/menu/**`, `engine/room/**`, `engine/store/**`, `public/grafik/**` (vorläufig)
- **Abhängig von:** B-964, B-965
- **Braucht 🧑:** TV-Abnahme
- **Größe:** M (vermutet)
- **Sperre:** spielstand (vermutet)
- Berührt das Spielstand-Format; keine Rücksicht auf alte Spielstände.

## Beispiele

Sengoku, Insel 1 nach 14 Tagen zu zweit besiegt: Auf der Miniatur steht die Maske des Endgegners mit Fahne; darunter die Abschlusszeile in Kunstsprache und „14 Tage · 2 Monarchen“. Dieselbe Karte erscheint als Vorschau in der Spielstand-Auswahl.

## Ausnahme- und Fehlerfälle

Ungültige oder seltene Situation → gewolltes Verhalten. Passt nichts: `nicht relevant` mit Grund.

## Akzeptanzkriterien

- **AC-01** Prüfbar formuliert: Test, Befehl, Datei oder Beobachtung am TV mit erwartetem Ergebnis.

Abnahme-Ticket eines Sprints oder Projekts (B-742): Kriterien AC-01 Sim (`sim_test`), AC-02 Balance, AC-03 Gen
(`check_run task:test` samt `gen:paper` und Golden), AC-04 Race (`check_run task:check:race`), Einzelheiten in
`docs/arbeitsweise.md` › Sim, Balance und Gen.

## Offene Fragen

- Welche Trophäen-Motive je Welt und Endgegner?
- „Wer dabei war“: Anzahl der Monarchen oder Namen/Farben der Spieler?

## Notizen

Grundzüge in der Ideenrunde k3c-07 (2026-11-15) mit 🧑 geklärt; Akzeptanzkriterien fehlen noch, Spec bleibt Entwurf.
