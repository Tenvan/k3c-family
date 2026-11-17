# BB2 · CLI · Bildaufbau im Spiel: Gehlinie, Spiegel-Fluss, Weite, Licht und Tönung

- **Status:** geplant
- **Projekt:** PAP
- **Domäne:** CLI
- **Reife:** Entwurf
- **Tickets:** B-1043, B-1044, B-1045, B-1046, B-1047
- **Start-Commit:** – (wird beim Aktivieren gesetzt: `git rev-parse --short origin/develop`)
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

Der Standard `docs/standards/bildaufbau.md` (#686) verlangt drei Bänder (Gehlinie 58 %, Ufer 7 %, Fluss als Spiegel) und ≈ 5 M sichtbare Welt. Dazu Tönung je Tageszeit, Lichthöfe, Dunst und einen Wald, der mit dem Roden offener wird. Heute zeichnet das Spiel die Gehlinie bei 66,7 %, das Ufer mit ≈ 3 %, einen Wellen-Fluss ohne Spiegelung und ≈ 3,5 M Welt. Das Spiel wird nach Rust portiert (Go-Stopp), deshalb setzt BB2 im Rust-Client an.

## Ziel

Das Spiel hat die Stimmung von K2C: weite Welt, Spiegel-Fluss, Licht und Farbe je Tageszeit. Am Ende sichtbar: am TV in einer und zwei Ansichten.

## Beteiligte und Zielgruppen

Spieler am TV. Umsetzung: Claude (CLI, Rust-Client), Fachprüfung `k3c-hud`, `k3c-grafik`. Freigabe, Entscheidung zur Weite und Abnahme: 🧑.

## Anforderungen

Siehe `B-1043` bis `B-1047 › Anforderungen`. Frame-Budget: 60 Hz mit zwei Ansichten auf dem Mini-PC, schlechtester Frame zählt.

## Nicht-Ziele

Asset-Größen (BB1), Engine-Regeln, neue Wetter-Mechanik.

## Regeln und Einschränkungen

- **Bereich:** `rust/crates/k3c-game/**`, `data/paper.json`, `data/biomes/**`
- **Abhängig von:** RU7, BB1
- **Braucht 🧑:** Freigabe, Entscheidung (Weite), Abnahme
- **Sperre:** –
- Das Spiel zeichnet nur; Werte nach `data/`.

## Beispiele

Nacht am Hub, 2 Ansichten: Gehlinie je Hälfte bei 58 %, Lagerfeuer und Mond hell im Spiegel, Waldrand dunkel.

## Ausnahme- und Fehlerfälle

Siehe Tickets.

## Akzeptanzkriterien

- **AC-01** Gehlinie 58 %, Ufer 7 % (B-1043/AC-01).
- **AC-02** Fluss spiegelt die Welt im Frame-Budget (B-1044/AC-01, AC-02).
- **AC-03** Sichtbare Welt ≈ 5 M aus `data/` (B-1045/AC-01).
- **AC-04** Tönung, Lichthöfe, Dunst (B-1046/AC-01, AC-02).
- **AC-05** Wald nach Rodung (B-1047/AC-01).
- **AC-06** Abnahme am TV (🧑).
- **AC-07** Review: Prüfläufe grün, Abnahme-Ticket angelegt.

## Offene Fragen

Weite 4, 4,5 oder 5 M (B-1045), 🧑 am TV; Default 5 M.

## Sessions

| Nr. | Datei | Typ | Agent | Status |
|---|---|---|---|---|

- BB2.1 Gehlinie, Ufer, Weite (AC-01, AC-03)
- BB2.2 Spiegel-Fluss (AC-02)
- BB2.3 Tönung, Lichthöfe, Dunst (AC-04)
- BB2.4 Wald nach Rodung (AC-05)
- BB2.5 Review (AC-07)

## Abnahme

Wird von der Review-Session (Doku-Sprint: letzte Session) ausgefüllt, höchstens fünf Zeilen: Datum, Kriterien
(Verweis auf Session-Ergebnisse), behobene Befunde, neue Tickets und das Abnahme-Ticket für Sim, Balance und Gen
(genau eins, `docs/arbeitsweise.md` › Sim, Balance und Gen). Bis dahin `–`.
