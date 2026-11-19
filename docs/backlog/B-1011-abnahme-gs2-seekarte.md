# B-1011 · Abnahme GS2: Seekarte als Papierschnitt und Schiffsfahrt sind am TV gesichtet, Gen-Läufe geprüft

- **Domäne:** CLI
- **Typ:** Frage
- **Prio:** mittel
- **Umgebung:** ?
- **Status:** offen
- **Sprint:** RU60
- **Projekt:** RSU
- **Erstellt:** 2026-11-15
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –
- **Duplikat von:** –
- **Ersetzt:** –

## Ausgangslage

Sprint GS2 hat die Seekarte als Papierschnitt-Diorama und die Schiffsfahrt beim Inselwechsel geliefert (PRs #638, #642, #648). Die Review-Session GS2.4 hat zwei Fehler behoben; offen sind die TV-Abnahme durch 🧑 und die Gen- und Shot-Läufe.

## Ziel

GS2 ist abgenommen: Diorama und Schiffsfahrt am TV gesichtet, Gen-Läufe grün, Zeichnen ohne Allokationen belegt.

## Beteiligte und Zielgruppen

🧑 (TV), Claude (Gen, Race, Shot-Messung).

## Anforderungen

- Diorama am Hafen solo und zu zweit: Miniaturen je Welt, Aufsätze, Zustandszeichen, Lesbarkeit.
- Schiffsfahrt: Fahrzeit, Bogen, Welle, Fahne; Wechsel aus einer tieferen Stufe; Fahrt zwischen verschiedenen Reihen ohne Sprung.
- Gen-Läufe und Shot-Messung grün.

## Nicht-Ziele

Simulation, Balancing; neue Funktionen.

## Regeln und Einschränkungen

- Umgehängt auf Rust (B-1143, 2026-11-19): Das Spiel ist `rust/crates/k3c-game`; die Go-Vorlage steht nur im Tag `go-referenz-final` (Entscheidung 013). Die Fahrt liegt in `rust/crates/k3c-game/src/game/voyage.rs`.
- Werte der Fahrt stellt 🧑 in `data/native.json` › `voyage` nach.

Planungsangaben für die Arbeitswellen (`docs/arbeitsweise.md` › Parallelität und Wellen), je eine Zeile, Werte ohne Freitext; Pflicht ab `Spec: freigegeben`:

- **Bereich:** `rust/crates/k3c-game/src/hud/**`, `rust/crates/k3c-game/src/game/voyage.rs`, `data/native.json`
- **Abhängig von:** GS2.4
- **Braucht 🧑:** Abnahme
- **Größe:** S
- **Sperre:** –

## Beispiele

Sengoku: Am Hafen der Reisfeld-Ebene zeigt die Karte zwei Ziele; nach der Wahl fährt das Schiff 3 s zur gewählten Insel.

## Ausnahme- und Fehlerfälle

Fällt ein Punkt durch → eigene Nachbesserung, dieses Ticket bleibt offen.

## Akzeptanzkriterien

- **AC-01** `check_run task:test` samt `gen:paper` und Grafik-Golden grün; `task check:race` gelaufen (lokal ohne gcc als übersprungen nennen).
- **AC-02** Shot-Läufe `K3C_VOYAGE_SHOT` und Seekarte in `native/k3c-game/hud` zeigen keine Allokation des HUD je Bild; Bilder liegen bei.
- **AC-03** 🧑 hat das Diorama am TV gesichtet (zugleich B-964/AC-05).
- **AC-04** 🧑 hat die Schiffsfahrt am TV gesichtet (zugleich B-977/AC-05).

## Offene Fragen

keine

## Notizen

Aus GS2.4.
