# B-203 · Die Kennzahl „erste Gold-Schwelle“ hat eine feste Schwelle und Bedeutung

- **Domäne:** REG
- **Typ:** Frage
- **Prio:** niedrig
- **Umgebung:** offline
- **Status:** eingeplant
- **Sprint:** RG2
- **Projekt:** BAL
- **Erstellt:** 2026-10-04
- **Spec:** Entwurf
- **Revision:** 2
- **Freigabe:** –

## Ausgangslage

B-099 verlangte die Kennzahl „Tick der ersten Gold-Schwelle“ ohne Schwelle. Der Balancing-Tester misst sie als ersten Tick, an dem das Gold aller Monarchen zusammen mindestens `--gold-threshold` erreicht (`engine/simcli/balance.go` Zeile 84, Standard 0 = nicht messen; `engine/balance/metrics.go` Zeile 126). Jeder Monarch startet mit vollem Beutel, jede Schwelle bis Beutel × Spieler ist in Tick 1 erreicht. Entschieden am 2026-10-31 (Claude im Auftrag von Ralf, B-627 › Notizen): **Die Kennzahl entfällt**; die Aussage trifft „Gold am Morgen je Spieler“ (`docs/rules/zielkorridore.md`). Kein Regeltext nennt die Kennzahl (Suche nach „Gold-Schwelle“ in `docs/` außerhalb der Planung ohne Treffer, 2026-11-15).

## Ziel

Die Frage ist entschieden und nirgends mehr offen: Es gibt keine Kennzahl „erste Gold-Schwelle“ im Regelwerk.

## Beteiligte und Zielgruppen

🧑 bestätigt; kein Agent muss etwas ändern.

## Anforderungen

- Kein Dokument unter `docs/rules/` und `docs/game-design.md` nennt die Kennzahl als Zielgröße.
- Der Schalter `--gold-threshold` bleibt mit Standard 0 (nicht messen); kein Code-Umbau.

## Nicht-Ziele

Schalter und Feld `goldThresholdTick` aus dem Tester entfernen (eigenes Schuld-Ticket, falls 🧑 es will; Balancing ist gesperrt).

## Regeln und Einschränkungen

Keine Änderung an Dateien nötig.

- **Bereich:** `docs/rules/zielkorridore.md`
- **Abhängig von:** –
- **Braucht 🧑:** Entscheidung
- **Größe:** S
- **Sperre:** –

## Beispiele

nicht relevant: Kennzahl entfällt.

## Ausnahme- und Fehlerfälle

🧑 will die Kennzahl doch → neue Spec mit Bedeutung und Wert, Messung erst nach Ende der Balancing-Sperre.

## Akzeptanzkriterien

- **AC-01** `grep -rn "Gold-Schwelle" docs/rules docs/game-design.md` ohne Treffer; die Entscheidung „entfällt“ steht in diesem Ticket.

## Offene Fragen

- Entfällt die Kennzahl? → Default: ja (vorläufig, 🧑 bestätigt; B-627 vom 2026-10-31).

## Notizen

Stand geprüft 2026-11-15: AC-01 ist schon erfüllt (kein Treffer in `docs/rules`, `docs/game-design.md`); Tester-Schalter Standard 0 (`engine/simcli/balance.go`:84). **Vorschlag: Status verworfen** (Kennzahl entfällt), sobald 🧑 die Entscheidung bestätigt. Revision 2: Frage durch Entscheidung ersetzt.
