# B-300 · Das Profil „Mauern zuerst“ spielt messbar anders als „sparsam“

- **Domäne:** SIM
- **Typ:** Frage
- **Prio:** mittel
- **Umgebung:** offline
- **Status:** eingeplant
- **Sprint:** BAL5
- **Projekt:** BAL
- **Erstellt:** 2026-10-05
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

BAL3.2 hat das Profil `walls` („Mauern zuerst“) nach Beschluss BAL3.1 gebaut: nur Mauern samt Ausbau, dann anwerben. Mauer-Stufe n braucht Hub-Stufe n (`data/buildings.json` › `wall.levels`), kein Profil zahlt den Hub-Ausbau. Deshalb baut `walls` nie aus und liefert in 100 Seeds (2 Spieler, 5 Tage) dieselben Kennzahlen wie `saver` (`tools/k3c-dev/internal/balance/bots.go`).

## Ziel

„Mauern zuerst“ deckt eine eigene Spielweise ab, sonst bringt das Profil den Berichten nichts.

## Beteiligte und Zielgruppen

🧑 entscheidet das Verhalten; Agenten setzen es im Tester um; REG nutzt die Berichte in BR1.

## Anforderungen

- Je nach Beschluss: `walls` zahlt auch den Hub-Ausbau, oder es bleibt bei der Gleichheit mit `saver` (dann Profil zusammenlegen oder so dokumentieren).

## Nicht-Ziele

Änderung von `engine/sim` oder von Werten in `data/`.

## Regeln und Einschränkungen

Bots nur über `PlayerCommand`, deterministisch, kein `math/rand`.

## Beispiele

`task balance:run -- --seeds 100 --bots walls,saver --players 2 --days 5` → heute 97/100 Welle 3 bei beiden; nach Umsetzung unterscheiden sich die Kennzahlen.

## Ausnahme- und Fehlerfälle

nicht relevant: reine Verhaltensfrage eines Bots.

## Akzeptanzkriterien

- **AC-01** Der Beschluss von 🧑 steht in `tools/k3c-dev/internal/balance/README.md`; bei „Hub-Ausbau zahlen“ belegt ein Test, dass `walls` eine Mauer auf Stufe 2 ausbaut.

## Offene Fragen

- Soll „Mauern zuerst“ den Hub-Ausbau zahlen? Entscheidet 🧑.

## Notizen

Gefunden in BAL3.2 und im Review BAL3.4 (2026-10-05).
