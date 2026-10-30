# B-653 · Bauschilder an Bäumen und Felsen nur nach Markierung, keine Bauoptionen für gesperrte Gebäude, Holzmauer als Zaun

- **Domäne:** CLI
- **Typ:** Problem
- **Prio:** hoch
- **Umgebung:** live
- **Status:** eingeplant
- **Sprint:** BS13
- **Projekt:** BAS
- **Erstellt:** 2026-10-28
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** 2026-10-28, Chat, durch 🧑, Revision 1

## Ausgangslage

Beobachtungen 🧑 (2026-10-28): (1) An Bäumen und Felsen stehen Bauschilder (Wimpel), obwohl sie nicht zum Abbau markiert sind. (2) Der Client zeigt weiter Bauoptionen (Aktion/Overlay) für Gebäude, die noch nicht gebaut werden dürfen; nur die Schilder an den Plätzen sind nicht mehr da. (3) Die Holzmauer sieht nicht wie ein Zaun aus.

## Ziel

Bauschilder und Bauoptionen erscheinen nur, wo wirklich gebaut oder abgebaut werden darf; die Holzmauer sieht aus wie ein Holzzaun.

## Beteiligte und Zielgruppen

Spieler (Xbox/PC/Mobile, Split-Screen); 🧑 nimmt die Optik ab.

## Anforderungen

- **Schilder an Bäumen und Felsen:** Ein Bauschild/Wimpel erscheint dort nur, wenn der Knoten **zum Abbau markiert** ist (Markierung setzt der Spieler), und zwar **anstelle des bisherigen Wimpels**; unmarkiert: kein Schild.
- **Bauoptionen:** Die Aktions-Overlays (Taste und Aktion am Ort, `monarch.md` § 4) und Zahlziele zeigen nur Gebäude, die an diesem Platz **jetzt gebaut werden dürfen** (Hub-Stufe, Material, gebaute Voraussetzung, bei B-652 auch Baum/Fels-Freiraum). Gesperrte Gebäude: keine Option, kein Preis.
- **Holzmauer als Zaun:** Die Holzmauer (Stufe 1) wird als **Holzzaun** gezeichnet (senkrechte Pfähle, Querbalken, Spitzen); Stein und höher bleiben Mauern. Grafik über den Asset-Studio-Weg (`tools/k3c-studio`, Gebäude-Planer, GEB-Projekt), kein Code im Sim.

## Nicht-Ziele

Neue Plätze oder Regeln der Mauern/Türme (B-652); neue Grafikstile für andere Gebäude.

## Regeln und Einschränkungen

Client zeichnet nur den Zustand des Servers (`src/scenes/render/`, `src/scenes/hud/`), die Logik dazu in `engine/sim` + Test; `installPageChrome()`/Seitenregeln unberührt; Logging mit Emoji; Sprites über den Gebäude-Planer mit Beschreibung (GEB-Regeln).

## Beispiele

Baum ohne Markierung: kein Schild; Baum nach Markierung: Bauschild statt Wimpel. Platz für Kaserne vor Hub-Stufe 2: keine Option, kein Preis.

## Ausnahme- und Fehlerfälle

Markierung wird zurückgenommen: Schild verschwindet; Bauer hat den Knoten abgebaut: kein Schild. Mehrere Spieler am selben Platz sehen dieselbe Optionsliste.

## Akzeptanzkriterien

- **AC-01** Kein Schild an unmarkierten Bäumen/Felsen; markierte zeigen das Bauschild statt des Wimpels (Test am Render-Zustand, Browser-Abnahme 🧑).
- **AC-02** Die Aktionsliste/Overlays enthalten nur Gebäude, die am Platz jetzt baubar sind (Sim-Test auf die Aktionen, Client-Test auf die Anzeige).
- **AC-03** Holzmauer Stufe 1 sieht wie ein Zaun aus (Asset-Beschreibung und Sprite, Abnahme 🧑); Stufen ab Stein unverändert.

## Offene Fragen

- Wo genau erscheinen heute die Optionen für gesperrte Gebäude (Aktionsliste im Sim oder nur Client-HUD)? Claude prüft das bei der Umsetzung.
- Soll der Zaun beim Ausbau auf Stufe 2 (Stein) weiterhin als Mauer erscheinen (Vorschlag: ja).

## Notizen

Links, Messwerte, verworfene Ansätze. Darf leer bleiben (`–`).
