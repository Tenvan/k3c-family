# BAL6 · SIM · Balancing-Tester misst die Wirtschaft

- **Status:** geplant
- **Projekt:** BAL
- **Domäne:** SIM
- **Prio:** hoch
- **Reife:** Entwurf
- **Einschiebbar:** ja
- **Tickets:** B-347
- **Start-Commit:** –
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

BR1.1 konnte nur 3 von 10 Wirtschafts-Korridoren messen: Der Bot `saver` baut nur Mauern, und Messgrößen für Turm, Hub-Stufe, Kämpfer je Hub sowie Gold und Holz bei `dawn` fehlen (B-347).

## Ziel

Alle Wirtschafts-Korridore aus `zielkorridore.md` › BR1 sind messbar, und der Bot des Standardszenarios spielt die Wirtschaft. Am Ende sichtbar: `task balance` zeigt je Wirtschafts-Korridor eine bewertete Zeile.

## Beteiligte und Zielgruppen

REG (BR1.3, BR2, RG3) nutzt die Berichte; ein Agent baut in `tools/k3c-dev/internal/balance/`; 🧑 gibt die Spec frei.

## Anforderungen

B-347 › Anforderungen.

## Nicht-Ziele

Wertänderungen in `data/` außer `balance-targets.json` (BR1.3); Profil „Mauern zuerst“ und Sensitivität (BAL5); Miniboss Nacht 5 (RG3).

## Regeln und Einschränkungen

Domäne SIM. Deterministisch, 100 feste Seeds, nur `engine/rng`. Baseline nur mit Begründung (`task balance:baseline`). Testläufe nur über `sim_test`, nie über `task balance` in der Shell (B-348). Datei ≤ 400, Funktion ≤ 60 Zeilen.

## Beispiele

B-347 › Beispiele.

## Ausnahme- und Fehlerfälle

B-347 › Ausnahme- und Fehlerfälle (Kennzahl ohne Ereignis: aus dem Zustand ableiten, im Bericht vermerken).

## Akzeptanzkriterien

- **AC-01** `task balance` bewertet alle Zeilen der Tabelle „Korridore der Wirtschaft“ (BR1) (`B-347/AC-01`).

## Offene Fragen

- Ein neues Standardprofil statt `saver` umbauen? 🧑, blockiert BAL6.1.

## Sessions

Entwurf. Vor dem Aktivieren jede Session als Datei nach `docs/vorlagen/session.md` schreiben, die Kriterien in Klammern werden ihr Feld `Kriterien`.

- BAL6.1 Messgrößen der Wirtschaft, Bot spielt Wirtschaft, Ziele in `balance-targets.json` (AC-01).
- BAL6.2 Review (Code-Sprint): alle Kriterien prüfen.

## Abnahme

–
