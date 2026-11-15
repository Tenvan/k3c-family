# B-185 · Wirtschaft nennt denselben Verlust-Korridor je Welle wie die Bürger

- **Domäne:** REG
- **Typ:** Schuld
- **Prio:** niedrig
- **Umgebung:** offline
- **Status:** eingeplant
- **Sprint:** RG2
- **Projekt:** BAL
- **Erstellt:** 2026-10-03
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** 🧑 2026-11-15

## Ausgangslage

`docs/rules/wirtschaft.md` § 3 (Zeile 37, Stand 2026-11-15) nennt als Zielkorridor weiter „Verluste je Welle: Median höchstens die Hälfte der Truppen (Normal)“. `docs/rules/buerger.md` § 3 (Zeile 52), `docs/rules/kampf.md` § 4 (Zeile 53) und `docs/rules/zielkorridore.md` § 4 (Zeile 53) nennen „höchstens 25 % der Kämpfer“. Im Workshop F1.4 (2026-10-03) hat 🧑 entschieden: Es gilt **höchstens 25 % der Kämpfer**. `zielkorridore.md` Zeile 53 verweist noch auf den abweichenden Wert und auf dieses Ticket.

## Ziel

Beide Regelwerke nennen denselben, von 🧑 entschiedenen Korridor; der Balancing-Tester (B-099) prüft gegen eine einzige Zahl.

## Beteiligte und Zielgruppen

🧑 hat entschieden; ein Agent gleicht den Text an. BAL2 nutzt die Zahl.

## Anforderungen

- Die Zeile in `wirtschaft.md` § 3 nennt „Median höchstens 25 % der Kämpfer“ und verweist auf `zielkorridore.md` § 4.
- Der Nebensatz in `zielkorridore.md` § 4 über den abweichenden Wert in `wirtschaft.md` und B-185 entfällt.
- Gilt für 1 bis 4 Spieler gleich (Kennzahl je Welle, nicht je Spieler); keine Wirkung auf Simulation oder Determinismus.

## Nicht-Ziele

Messgröße und Prüfung (B-099, B-157); keine Änderung an `data/*.json`.

## Regeln und Einschränkungen

Domäne REG. Keine neue Zahl; nur die Entscheidung aus F1.4 übernehmen. Nur Text, kein Sim- oder Balancing-Lauf.

- **Bereich:** `docs/rules/wirtschaft.md`, `docs/rules/zielkorridore.md`
- **Abhängig von:** –
- **Braucht 🧑:** Freigabe
- **Größe:** S
- **Sperre:** –

## Beispiele

Ein Leser von `wirtschaft.md` § 3 sieht 25 % der Kämpfer, wie in `buerger.md` § 3 und `zielkorridore.md` § 4.

## Ausnahme- und Fehlerfälle

nicht relevant: reine Textangleichung.

## Akzeptanzkriterien

- **AC-01** `docs/rules/wirtschaft.md` § 3 nennt „Median höchstens 25 % der Kämpfer“ mit Verweis auf `zielkorridore.md`; `grep -n "Hälfte der Truppen" docs/rules/wirtschaft.md docs/rules/zielkorridore.md` findet nichts.

## Offene Fragen

keine

## Notizen

Entscheidung: Workshop F1.4, 2026-10-03; bestätigt in B-627 › Notizen (2026-10-31). Stand geprüft 2026-11-15: weiter offen (`wirtschaft.md`:37).
