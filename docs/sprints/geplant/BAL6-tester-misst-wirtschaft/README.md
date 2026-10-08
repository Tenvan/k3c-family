# BAL6 · SIM · Balancing-Tester misst die Wirtschaft

- **Status:** geplant
- **Projekt:** BAL
- **Domäne:** SIM
- **Prio:** hoch
- **Reife:** Entwurf
- **Einschiebbar:** ja
- **Tickets:** B-347, B-300, B-301
- **Start-Commit:** –
- **Spec:** Entwurf
- **Revision:** 2
- **Freigabe:** –

## Ausgangslage

BR1.1 konnte nur 3 von 10 Wirtschafts-Korridoren messen: Der Bot `saver` baut nur Mauern, und Messgrößen für Turm, Hub-Stufe, Kämpfer je Hub sowie Gold und Holz bei `dawn` fehlen (B-347).

## Ziel

Alle Wirtschafts-Korridore aus `zielkorridore.md` › BR1 sind messbar, und der Bot des Standardszenarios spielt die Wirtschaft. Am Ende sichtbar: `task balance` zeigt je Wirtschafts-Korridor eine bewertete Zeile.

## Beteiligte und Zielgruppen

REG (BR1.3, BR2, RG1) nutzt die Berichte; ein Agent baut in `tools/k3c-dev/internal/balance/`; 🧑 gibt die Spec frei.

## Anforderungen

B-347 › Anforderungen; B-300 › Anforderungen; B-301 › Anforderungen (B-300, B-301 aus BAL5, PJ3).

## Nicht-Ziele

Wertänderungen in `data/` außer `balance-targets.json` (BR1.3); Miniboss Nacht 5 (RG1, AC-04). Profil „Mauern zuerst“ und Sensitivität gehören seit PJ3 zu diesem Sprint (AC-02, AC-03).

## Regeln und Einschränkungen

Domäne SIM. Deterministisch, 100 feste Seeds, nur `engine/rng`. Baseline nur mit Begründung (`task balance:baseline`). Testläufe nur über `sim_test`, nie über `task balance` in der Shell (B-348). Datei ≤ 400, Funktion ≤ 60 Zeilen.

## Beispiele

B-347 › Beispiele.

## Ausnahme- und Fehlerfälle

B-347 › Ausnahme- und Fehlerfälle (Kennzahl ohne Ereignis: aus dem Zustand ableiten, im Bericht vermerken).

## Akzeptanzkriterien

- **AC-01** `task balance` bewertet alle Zeilen der Tabelle „Korridore der Wirtschaft“ (BR1) (`B-347/AC-01`).
- **AC-02** Das Profil „Mauern zuerst“ spielt messbar anders als „sparsam“ (B-300/AC-01; aus BAL5 AC-01, PJ3).
- **AC-03** Ein Sensitivitäts-Pfad ohne Wirkung ergibt einen Fehler (B-301/AC-01; aus BAL5 AC-02, PJ3).

## Offene Fragen

- Ein neues Standardprofil statt `saver` umbauen? 🧑, blockiert BAL6.1.

## Sessions

Entwurf. Vor dem Aktivieren jede Session als Datei nach `docs/vorlagen/session.md` schreiben, die Kriterien in Klammern werden ihr Feld `Kriterien`.

- BAL6.1 Messgrößen der Wirtschaft, Bot spielt Wirtschaft, Ziele in `balance-targets.json` (AC-01).
- BAL6.2 Review (Code-Sprint): alle Kriterien prüfen.
- BAL6.3 Profil „Mauern zuerst“ schärfen, Sensitivität prüft Wirkung (AC-02, AC-03; aus BAL5.1, PJ3). Beim Bereitmachen vor das Review ordnen.

## Abnahme

–
