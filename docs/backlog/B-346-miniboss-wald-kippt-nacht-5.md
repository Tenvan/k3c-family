# B-346 · Der Miniboss Wald kippt Nacht 5 in fast jedem Seed

- **Domäne:** REG
- **Typ:** Problem
- **Prio:** hoch
- **Umgebung:** offline
- **Status:** eingeplant
- **Sprint:** RG3
- **Projekt:** –
- **Erstellt:** 2026-10-07
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

Messung BR1.1 (2026-10-07, `zielkorridore.md` › „Balancing-Runde Wirtschaft (BR1)“): Im Standardszenario hält die Burg die Nächte 1–4 in 99 %, Nacht 5 nur in 4 %. In Welle 5 kommt der Miniboss Wald (`data/bosses.json › goblinLeader`, `hpFactor` 8, `summon` 2 Goblins alle 10 s). Grad-Kurven: Dev und Leicht halten Nacht 5 nur in 5 %, Hart in 59 %, Ultra in 26 %. Hart hält damit besser als Normal, was nicht monoton ist.

## Ziel

„Burg hält Nacht 1–5“ liegt je Grad im Korridor (`zielkorridore.md` § 1–2), der Miniboss bleibt eine spürbare Prüfung.

## Beteiligte und Zielgruppen

Familie als Spieler; 🧑 entscheidet über Werte (BR2).

## Anforderungen

- Ursache der Nacht 5 klären (Boss-HP, Beschwörung, fehlende Truppen des Bots) und Werte in `data/bosses.json` vorschlagen.
- Die nicht monotone Grad-Kurve (Hart besser als Normal in Nacht 5) erklären.

## Nicht-Ziele

Neue Mechaniken, Wirtschaftswerte (BR1).

## Regeln und Einschränkungen

Werte nur in `data/`, Begründung in `docs/rules/bosse.md`; Golden nach B-137.

## Beispiele

Normal, 2 Spieler, `saver`, Seed 1 → Burg fällt in Nacht 5 (`castleFallTick` ≈ Tag 5).

## Ausnahme- und Fehlerfälle

nicht relevant: Messbefund.

## Akzeptanzkriterien

- **AC-01** `task balance` zeigt „Burg hält Nacht 1–5“ im Korridor 75–90 % (Normal), die Grad-Kurven sind monoton.

## Offene Fragen

Gehört die Prüfung zu BR2 (Kampf und Bosse) oder vorher (🧑)?

## Notizen

Messbefehle: `task balance`, `k3c-balance --curves --seeds 100` (`reports/sensitivity-20261007-150222.md`).
