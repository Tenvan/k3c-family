# B-315 · Der Level-Betrachter erzeugt einen Spielstand mit allen Gebäuden voll ausgebaut

- **Domäne:** SRV
- **Typ:** Idee
- **Prio:** hoch
- **Umgebung:** live
- **Status:** eingeplant
- **Sprint:** SV1
- **Projekt:** WRT
- **Erstellt:** 2026-10-06
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

Für das Grafik-Review (GR3.4) muss 🧑 jedes Gebäude in jeder Ausbaustufe sehen. Heute geht das nur durch langes Spielen. `leveltest.html` (B-092) zeigt ein generiertes Level und startet es im Spiel, legt aber keinen Spielstand mit gebauten Gebäuden an. Die Stufen liefert die Simulation (`engine/sim/hub_level.go`, Feld `level` der Bauplätze); den Hub-Ausbau überträgt das Protokoll erst mit B-208.

## Ziel

Ein Klick im Level-Betrachter erzeugt einen Spielstand, in dem alle Bauplätze aller Stufen gebaut und voll ausgebaut sind, und startet ihn im Spiel. Nutzen: Grafik-Abnahmen in Minuten statt nach einer langen Partie.

## Beteiligte und Zielgruppen

🧑 bei Grafik-Reviews; Agents bei Abnahme-Interviews.

## Anforderungen

- Seed und Biom wie im Level-Betrachter gewählt; alle Gebäude gebaut, jede Ausbaustufe mindestens einmal sichtbar (Hub-Stufen, Mauer- und Turm-Materialstufen).
- Alle Stufen des Raums (Inselstufen) bekommen den Ausbau, nicht nur die erste.
- Deterministisch: gleicher Seed → gleicher Spielstand.
- Spielstand landet wie jeder andere unter `saves/` und ist mit Namen ladbar.

## Nicht-Ziele

Darstellung der Stufen im Renderer (B-208 und Folge-Ticket CLI); Balancing.

## Regeln und Einschränkungen

Nur `engine/rng`; Dev-Funktion, im Heimnetz-Betrieb nicht für Spieler sichtbar; Seiten-Regeln aus `CLAUDE.md` für den Knopf in `leveltest.html` (PLAT).

## Beispiele

Level-Betrachter, Seed `abc`, Wald → „Voll ausgebaut starten“ → Spiel zeigt Hub Stufe 5 neben allen Mauer- und Turmstufen.

## Ausnahme- und Fehlerfälle

Eine Stufe hat keine Grafik → Platzhalter-Form wie im Renderer (GR3/AC-03), kein Fehler.

## Akzeptanzkriterien

- **AC-01** Ein Go-Test belegt: Der erzeugte Spielstand hat jeden Bauplatz aller Stufen gebaut und jede Ausbaustufe mindestens einmal.
- **AC-02** Ein Go-Test belegt: Gleicher Seed ergibt denselben Spielstand.
- **AC-03** `leveltest.html` startet den Spielstand mit einem Klick bzw. einer Taste (Tastatur), Beobachtung durch 🧑 am PC.

## Offene Fragen

- Entschieden 2026-10-06 (🧑, Chat): Spielstand und Dev-API (SRV) sowie der Knopf im Level-Betrachter (PLAT) kommen in SV1; der Knopf ist eine eigene Session als Domänen-Ausnahme.
- Entschieden 2026-10-06 (🧑, Chat): „Alle Level“ heißt alle Inselstufen eines Raums, nicht alle Biome.

## Notizen

Wunsch von 🧑 im Chat 2026-10-06 („Spielstand mit allen ausgebauten Gebäuden auf allen Leveln … über den Level-Betrachter“). Verwandt: B-092, B-095, B-075, B-208, GR3.4.
