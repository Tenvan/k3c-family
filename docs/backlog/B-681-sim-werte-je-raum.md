# B-681 · Die Simulation rechnet mit den Werten ihrer Welt statt mit globalen Datei-Variablen

- **Domäne:** SIM
- **Typ:** Idee
- **Prio:** hoch
- **Umgebung:** offline
- **Status:** eingeplant
- **Sprint:** BAL8
- **Projekt:** BAL
- **Erstellt:** 2026-10-31
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** 2026-10-31 autonom, Claude (Wunsch 🧑 im Chat)

## Ausgangslage

Die Sim lädt `data/*.json` einmal in Paket-Variablen (z. B. `goals`, `islandSizes`, `engine/sim/data.go` › `load`). Ein Raum kann deshalb keine eigenen Werte haben; mehrere Räume mit verschiedenen Welten laufen aber gleichzeitig (`welten.md` § 6).

## Ziel

Jeder Raum rechnet mit dem aufgelösten Wertesatz seiner Welt (Basis + Overrides aus B-680); Räume mit verschiedenen Welten beeinflussen sich nicht.

## Beteiligte und Zielgruppen

🧑 Ralf entwirft Welten und balanciert sie; Sessions der Domänen REG, SIM, DEV setzen um.

## Anforderungen

- Wertesatz wird beim Anlegen/Laden des Raums aufgelöst und im Raum gehalten, nicht global verändert.
- Deterministisch, 2 Spieler wie 1; Leistung: kein Mehraufwand je Tick, Auflösung einmal je Raum.
- Abhängig von B-680 (Format), liefert die Plumbing für alle Wertearten.
- Spielstand lädt mit dem Wertesatz der Welt; ändert sich die Welt-Datei, gilt `welten.md` § 5.

## Nicht-Ziele

Format und Prüfung (B-680); Oberflächen (B-682, B-683).

## Regeln und Einschränkungen

Determinismus (`engine/rng`), 2 Spieler gleichzeitig, `docs/rules/welten.md`, Datei ≤ 400 Zeilen, Funktion ≤ 60. Zusammenhang: B-680 → B-681 → B-682 → B-683.

## Beispiele

Welt „Nordmeer“ setzt nur `overrides` für die Kosten einer Brücke; alle anderen Werte kommen aus `data/`, die Standard-Welt bleibt unverändert.

## Ausnahme- und Fehlerfälle

Ungültiger Override → Welt wird nicht geladen, Meldung nennt Welt, Datei und Pfad.

## Akzeptanzkriterien

- **AC-01** Go-Test: zwei Räume mit verschiedenen Welten rechnen parallel mit ihren eigenen Kosten und beeinflussen sich nicht.
- **AC-02** Go-Test: Raum ohne Overrides liefert byte-gleiche Ergebnisse (Golden unverändert).
- **AC-03** `task check:race` grün.

## Offene Fragen

keine

## Notizen

–
