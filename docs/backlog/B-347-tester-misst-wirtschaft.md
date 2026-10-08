# B-347 · Der Balancing-Tester misst die Wirtschafts-Kennzahlen

- **Domäne:** SIM
- **Typ:** Schuld
- **Prio:** hoch
- **Umgebung:** offline
- **Status:** eingeplant
- **Sprint:** BAL6
- **Projekt:** –
- **Erstellt:** 2026-10-07
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

BR1.1 (2026-10-07) konnte nur 3 der 10 Wirtschafts-Korridore messen. Der Bot `saver` baut nur die Mauern: kein Turm, keine Farm, kein Hub-Ausbau, keine Rekrutierung. Das Gold steht ab Tag 2 in 99 % der Läufe am Maximum. Messgrößen fehlen für Turm gebaut, Hub-Stufe, Kämpfer je Hub zu Tagesbeginn sowie Gold und Holz bei `dawn` (heute nur zur Dämmerung). Code: `tools/k3c-dev/internal/balance/` (`bots.go`, `metrics.go`), Ziele in `data/balance-targets.json`.

## Ziel

Alle Wirtschafts-Korridore aus `zielkorridore.md` › „Balancing-Runde Wirtschaft (BR1)“ sind mit `task balance` messbar, und der Bot des Standardszenarios spielt die Wirtschaft so, wie es das Standardszenario meint.

## Beteiligte und Zielgruppen

REG (BR1.3, BR2) als Nutzer; 🧑 gibt die Spec frei.

## Anforderungen

- Messgrößen: erster Turm (`built` Turm), Hub-Stufe 2/3 (Zeit), Kämpfer je Hub zu Tagesbeginn, Gold je Spieler und Holz bei `dawn`.
- Bot `saver` (oder ein neues Standardprofil): baut Turm und Farm, rekrutiert, baut den Hub aus, sobald er es bezahlen kann.
- Ziele in `data/balance-targets.json` für alle Wirtschafts-Korridore.

## Nicht-Ziele

Wertänderungen in `data/` außer `balance-targets.json` (BR1.3).

## Regeln und Einschränkungen

Deterministisch, 100 feste Seeds; Baseline nur mit Begründung (`task balance:baseline`).

## Beispiele

`task balance` → Bericht mit einer Zeile je Wirtschafts-Korridor.

## Ausnahme- und Fehlerfälle

Kennzahl ohne Ereignis → Ableitung aus dem Zustand zum Tick, im Bericht vermerkt.

## Akzeptanzkriterien

- **AC-01** `task balance` bewertet alle Zeilen der Tabelle „Korridore der Wirtschaft“ (BR1).

## Offene Fragen

Ein neues Profil statt `saver` umbauen (🧑)?

## Notizen

Ohne dieses Ticket kann BR1.3 Vorschläge nur gegen 3 Kennzahlen prüfen.
