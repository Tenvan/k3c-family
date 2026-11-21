# B-1197 · Bürger und Soldaten in den unteren Ebenen holen Ausrüstung von ganz oben oder von fallen gelassenen Stücken

- **Domäne:** SIM
- **Typ:** Idee
- **Prio:** mittel
- **Umgebung:** offline
- **Status:** offen
- **Sprint:** ECO1
- **Projekt:** WIR
- **Erstellt:** 2026-11-21
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** autonom, Claude, 2026-11-21
- **Duplikat von:** –
- **Ersetzt:** –

## Ausgangslage

Bauern holen Waffen und Hämmer aus den Regalen der Werkstatt, Bogner und Rüstkammer im selben Hub (`buerger.md`). Mit B-1196 gibt es unten keine produzierenden Gebäude mehr. 🧑-Idee 2026-11-21: Bürger und Soldaten holen sich Gegenstände von ganz oben oder von fallen gelassenen.

## Ziel

In den unteren Ebenen beschaffen Bürger und Soldaten Ausrüstung (Waffe, Bogen, Gürtel) entweder aus dem Lager ganz oben (sie reisen über die Treppen) oder von Stücken, die auf der Ebene liegen (fallen gelassen, B-929).

## Beteiligte und Zielgruppen

🧑 entscheidet über Weg und Dauer; Spieler sehen, woher Ausrüstung kommt.

## Anforderungen

- Regel: Ohne Regal auf der Ebene holt eine Figur Ausrüstung nur vom Boden der Ebene oder vom Regal ganz oben; der Weg nach oben nutzt die vorhandenen Treppen (Reisedauer aus den Daten).
- Nur ohne Gegner in 8 Units (wie B-929), deterministisch, 2+ Spieler.
- Truppen-Limit und Berufe bleiben wie heute.

## Nicht-Ziele

Gebäude-Beschränkung (B-1196), neue Ausrüstungsarten.

## Regeln und Einschränkungen

Regeln aus `CLAUDE.md`, nur Rust.

Planungsangaben für die Arbeitswellen:

- **Bereich:** `rust/crates/k3c-sim/src/unit/**`, `rust/crates/k3c-sim/src/building/**`, `rust/crates/k3c-sim/src/island/**`
- **Abhängig von:** B-1196
- **Braucht 🧑:** Abnahme
- **Größe:** M
- **Sperre:** golden | spielstand

## Beispiele

Typische Situation → erwartetes Ergebnis. Passt nichts: `nicht relevant` mit Grund.

## Ausnahme- und Fehlerfälle

Ungültige oder seltene Situation → gewolltes Verhalten. Passt nichts: `nicht relevant` mit Grund.

## Akzeptanzkriterien

- **AC-01** Unten ohne Regal holt ein Bürger eine liegende Waffe, sonst wandert er zum Regal ganz oben (Test in `k3c-sim`).
- **AC-02** Nur ohne Gegner in 8 Units.
- **AC-03** Regeltext `buerger.md` angepasst.

## Offene Fragen

Defaults (autonom entscheidbar): Die Reise nach oben nutzt die Treppen wie ein Bauer sie schon nutzt; Spieler können Ausrüstung auch selbst fallen lassen (B-929 gilt). 🧑 bestätigt bei der Freigabe. Abhängigkeit: B-1196.

## Notizen

Links, Messwerte, verworfene Ansätze. Darf leer bleiben (`–`).
