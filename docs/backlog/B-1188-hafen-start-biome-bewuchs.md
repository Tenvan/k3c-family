# B-1188 · Hafen als Startpunkt neuer Spiele, ein bis zwei Biome vom Lager, vom Wald umgeben und freizuräumen

- **Domäne:** SIM
- **Typ:** Idee
- **Prio:** mittel
- **Umgebung:** ?
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

Neue Spiele starten nicht am Hafen; der Hafen ist weder vom Wald umgeben noch hängt seine Biom-Zahl vom Seed ab.

## Ziel

Neue Spiele beginnen am Hafen; der Seed erzeugt vom Lager aus mindestens 1 und höchstens 2 Biome bis zum Hafen; der Hafen ist wie ein Biom von Bäumen umgeben (einstellbar) und muss freigeräumt werden.

## Beteiligte und Zielgruppen

Spieler; 🧑 Ralf nimmt am PC ab.

## Anforderungen

- Startpunkt neuer Spiele ist der Hafen.
- Zahl der Biome zwischen Lager und Hafen: 1 bis 2, vom Seed bestimmt.
- Der Hafen ist komplett von Bäumen umgeben, Dichte einstellbar (Datenwert), und muss freigeräumt werden.

## Nicht-Ziele

Neue Inseln, Seekarte.

## Regeln und Einschränkungen

Rust (`k3c-sim`, `k3c-level`, `k3c-room`), Go-Engine eingefroren. Keine Rücksicht auf alte Spielstände. Deterministisch.

Planungsangaben für die Arbeitswellen:

- **Bereich:** `rust/crates/k3c-sim/**`, `rust/crates/k3c-level/**`, `rust/crates/k3c-room/**`, `data/**`
- **Abhängig von:** B-1187
- **Braucht 🧑:** Abnahme
- **Größe:** M
- **Sperre:** golden

## Beispiele

Seed A: 1 Biom zwischen Lager und Hafen; Seed B: 2; nie 0 oder 3. Hafen von Bäumen umschlossen, erst nach Rodung erreichbar.

## Ausnahme- und Fehlerfälle

Dichte 0 im Datenwert: Hafen frei.

## Akzeptanzkriterien

- **AC-01** Neue Spiele starten am Hafen.
- **AC-02** Seed ergibt 1 bis 2 Biome zwischen Lager und Hafen (Test über viele Seeds).
- **AC-03** Hafen ist von Bäumen umgeben, Dichte einstellbar, Test.
- **AC-04** Abnahme am PC durch 🧑.

## Offene Fragen

keine

## Notizen

Links, Messwerte, verworfene Ansätze. Darf leer bleiben (`–`).
