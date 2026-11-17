# B-1047 · Der Wald wird mit dem Roden offener und heller

- **Domäne:** CLI
- **Typ:** Idee
- **Prio:** mittel
- **Umgebung:** live
- **Status:** offen
- **Sprint:** BB2
- **Projekt:** PAP
- **Erstellt:** 2026-11-17
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –
- **Duplikat von:** –
- **Ersetzt:** –

## Ausgangslage

`docs/standards/bildaufbau.md` § 5: In K2C ist der ungerodete Wald dunkel und geschlossen, das gerodete Land um den Hub hell und offen mit Blick auf das Leitmotiv. Bei uns sieht der Wald überall gleich aus, gerodete Flächen verändern das Bild kaum.

## Ziel

Fortschritt sieht man als Licht und Weite: Je mehr gerodet ist, desto offener und heller wird das Bild an dieser Stelle.

## Beteiligte und Zielgruppen

Spieler am TV. Umsetzung: Claude (CLI, Rust-Client), Fachprüfung `k3c-welten`. Abnahme: 🧑.

## Anforderungen

- Das Spiel bestimmt je Bildausschnitt die Walddichte aus dem Zustand (Bäume im Umkreis) und zeichnet danach die Waldwand (B-1041) dichter oder lässt den Blick auf die Ferne frei.
- Im dichten Wald dunklere Tönung, nur die Fackel als Licht; gerodet heller, Leitmotiv (B-1042) sichtbar.
- Übergänge weich, kein Springen beim Fällen eines Baums.

## Nicht-Ziele

Änderung der Rodungs-Regeln in der Engine.

## Regeln und Einschränkungen

- **Bereich:** `rust/crates/k3c-game/**`
- **Abhängig von:** B-1041, B-1042, B-1046
- **Braucht 🧑:** Freigabe, Abnahme
- **Größe:** M
- **Sperre:** –
- Das Spiel liest nur den Zustand, rechnet keine Regeln.

## Beispiele

Monarch reitet vom Hub nach außen: Himmel und Berg verschwinden hinter einer dunkler werdenden Waldwand.

## Ausnahme- und Fehlerfälle

Plantagen (Baumschule) zählen als licht, nicht als Wald.

## Akzeptanzkriterien

- **AC-01** Test: Walddichte aus einem Zustand mit vielen und mit wenigen Bäumen ergibt dunkle bzw. offene Darstellung.
- **AC-02** Am TV (🧑): Übergang Hub → Wald wirkt wie K2C, kein Springen.

## Offene Fragen

keine

## Notizen

Gedächtnis `reference_k2c-bildauswertung` (Bilder 1, 2, 8).
