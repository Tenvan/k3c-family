# RG2 · REG · Regelwerk-Klärungen: Korridore, Kennzahl, Tier-Gating, Handwerker, Tiefe 3–4

- **Status:** aktiv
- **Projekt:** BAL
- **Domäne:** REG
- **Reife:** bereit
- **Tickets:** B-295, B-185, B-203, B-219, B-024
- **Start-Commit:** –
- **Spec:** freigegeben
- **Revision:** 2
- **Freigabe:** 🧑 2026-11-15

## Ausgangslage

Stand 2026-11-15, je Ticket gegen Regelwerk und Daten geprüft: Der Verlust-Korridor steht in `docs/rules/wirtschaft.md` § 3 noch anders als in `buerger.md`, `kampf.md` und `zielkorridore.md` (B-185). `docs/game-design.md` nennt Tier-Gating 5/10/15 statt 2/4/6 (B-219; der Teil `events.go` ist erledigt). Der Geselle (Beruf `craftsman`, vor B-950 „Handwerker“) steht im Text an Werkstatt, Schmiede und Rüstkammer, in den Daten nur an der Werkstatt (B-295). Die Kennzahl „erste Gold-Schwelle“ entfällt (B-203). Tiefe 3 und 4 sind inzwischen beschrieben und haben Biom-Daten (B-024, Belege im Ticket).

## Ziel

Regelwerk und Übersicht widersprechen sich in diesen Punkten nicht mehr. Am Ende sichtbar: die `grep`-Prüfungen der Kriterien ohne Treffer, `task check` grün.

## Beteiligte und Zielgruppen

🧑 bestätigt die Defaults (Geselle nur Werkstatt, Kennzahl entfällt) und gibt frei; ein Agent schreibt `docs/rules/` und `docs/game-design.md`.

## Anforderungen

`B-185 › Anforderungen`; `B-219 › Anforderungen`; `B-295 › Anforderungen`; `B-203 › Anforderungen`; `B-024 › Anforderungen` (schon erfüllt, nur Gegenprüfung).

## Nicht-Ziele

Code; Werte in `data/`; Simulation und Balancing (gesperrt); Schalter `--gold-threshold` aus dem Tester entfernen.

## Regeln und Einschränkungen

REG, nur Doku. `docs/glossar.md` als Sammeldatei im letzten Commit.

- **Bereich:** `docs/rules/wirtschaft.md`, `docs/rules/zielkorridore.md`, `docs/rules/buerger.md`, `docs/rules/monarch.md`, `docs/game-design.md`
- **Abhängig von:** –
- **Braucht 🧑:** Entscheidung
- **Sperre:** –

## Beispiele

Ein Leser von `wirtschaft.md` § 3 sieht 25 % der Kämpfer wie in `buerger.md` § 3; `game-design.md` nennt Tier 2 ab 2 gelernten Skills.

## Ausnahme- und Fehlerfälle

🧑 bestätigt einen Default nicht → das Ticket verlässt den Sprint, die übrigen laufen weiter.

## Akzeptanzkriterien

- **AC-01** Regeltext und Daten nennen den Gesellen nur an der Werkstatt (`B-295/AC-01`).
- **AC-02** Wirtschaft nennt denselben Verlust-Korridor je Welle wie die Bürger (`B-185/AC-01`).
- **AC-03** Die Kennzahl „erste Gold-Schwelle“ entfällt und steht in keinem Regeltext (`B-203/AC-01`).
- **AC-04** Game-Design nennt Tier-Gating 2/4/6; die Ereignisliste nennt den Schlag des Monarchen (`B-219/AC-01`).
- **AC-05** Tiefe 3 und 4 sind beschrieben und haben Biom-Daten (`B-024/AC-01`, `B-024/AC-02`; schon erfüllt, Gegenprüfung in RG2.2).

## Offene Fragen

- Geselle nur an der Werkstatt, auch nicht am Bogner? → Default: ja (vorläufig, 🧑 bestätigt; B-627, 2026-10-31).
- Kennzahl „erste Gold-Schwelle“ entfällt, B-203 verworfen? → Default: ja (vorläufig, 🧑 bestätigt).
- B-024 ohne weitere Arbeit erledigt? → Default: ja, Belege in B-024 › Notizen (vorläufig, 🧑 bestätigt).

## Sessions

| Nr. | Datei | Typ | Agent | Status |
|---|---|---|---|---|
| RG2.1 | `RG2.1-regeltexte-angleichen.md` | Umsetzung | autonom | fertig |
| RG2.2 | `RG2.2-review.md` | Review | autonom | fertig |

RG2.1 (REG, haiku) erfüllt AC-01 bis AC-04; RG2.2 (REG, sonnet) prüft alle, auch AC-05, gleicht das Glossar an und schließt ab. Kein Abnahme-Ticket: keine Sim, keine Grafik, keine Werte.

## Abnahme

2026-11-15 RG2.2 (Claude): AC-01 bis AC-05 mit grep und Datei:Zeile geprüft, alle erfüllt; Glossar angeglichen.
`task check` grün (1802 Tests), `task check:diff` grün; kein Sim, Balance, Gen (Doku-Sprint), daher kein Abnahme-Ticket.
B-185, B-219, B-295, B-024 erledigt. Offen bei 🧑: B-203 verwerfen (Kennzahl entfällt). Folgeticket: Anzeigename `craftsman` in `data/troops.json`.
