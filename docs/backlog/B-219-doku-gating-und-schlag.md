# B-219 · Game-Design und Ereignis-Doku nennen Tier-Gating 2/4/6 und den Schlag des Monarchen

- **Domäne:** REG
- **Typ:** Schuld
- **Prio:** niedrig
- **Umgebung:** offline
- **Status:** eingeplant
- **Sprint:** RG2
- **Projekt:** BAL
- **Erstellt:** 2026-10-04
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

Mit B-216 gilt Tier-Gating 2/4/6 gelernte Skills der Linie (`docs/rules/monarch.md` § 3 Zeile 30, `data/monarch.json` › `tierPoints` `[0, 2, 4, 6]`). `docs/game-design.md` Zeile 57 (Abschnitt Skill-System) nennt noch „Tier 2 ab 5, Tier 3 ab 10, Tier 4 (Ultimate) ab 15 Punkten“; `docs/rules/monarch.md` Zeile 57 führt den Widerspruch als offen. Der zweite Teil ist erledigt: `engine/sim/events.go` Zeile 12 beschreibt `strike` inzwischen für Gegner und Monarch („beim Monarchen zusätzlich hit“, B-321). Der Monarch hat seit dem Kampfsystem KSY keine HP mehr (`docs/rules/kampf.md` § 5), schlägt aber weiter.

## Ziel

Doku und Regelwerk widersprechen sich nicht.

## Beteiligte und Zielgruppen

Agenten und 🧑 beim Lesen der Doku.

## Anforderungen

- `docs/game-design.md` nennt Gating 2/4/6 gelernte Skills der Linie und verweist auf `docs/rules/monarch.md` § 3.
- `docs/rules/monarch.md` § „Offen“ nennt den Widerspruch zu `game-design.md` nicht mehr.
- `engine/sim/events.go` bleibt unverändert (schon erfüllt).

## Nicht-Ziele

Änderungen an Werten oder Code-Verhalten; übrige veraltete Stellen in `docs/game-design.md` (eigenes Ticket, falls gefunden).

## Regeln und Einschränkungen

`docs/rules/monarch.md` § 3 ist die Quelle. Nur Text.

- **Bereich:** `docs/game-design.md`, `docs/rules/monarch.md`
- **Abhängig von:** –
- **Braucht 🧑:** Freigabe
- **Größe:** S
- **Sperre:** –

## Beispiele

nicht relevant (Doku-Abgleich).

## Ausnahme- und Fehlerfälle

nicht relevant (Doku-Abgleich).

## Akzeptanzkriterien

- **AC-01** `grep -n "ab 5, Tier 3 ab 10" docs/game-design.md` und `grep -n "5/10/15" docs/rules/monarch.md` ohne Treffer; `docs/game-design.md` nennt 2/4/6 mit Verweis auf `rules/monarch.md` § 3; `engine/sim/events.go` nennt bei `strike` den Monarchen (heute Zeile 12, unverändert).

## Offene Fragen

keine

## Notizen

Gefunden in S1.1 (Sprint S1). Stand geprüft 2026-11-15: Teil `events.go` erledigt (B-321), Teil `game-design.md` offen.
