# B-185 · Wirtschaft nennt denselben Verlust-Korridor je Welle wie die Bürger

- **Domäne:** REG
- **Typ:** Schuld
- **Prio:** niedrig
- **Umgebung:** offline
- **Status:** eingeplant
- **Sprint:** RG2
- **Erstellt:** 2026-10-03
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

`docs/rules/wirtschaft.md` § 3 nennt als Zielkorridor „Verluste je Welle: Median höchstens die Hälfte der Truppen“, `docs/rules/buerger.md` § 3 „höchstens 25 % der Kämpfer“. Im Workshop F1.4 (2026-10-03) hat 🧑 entschieden: Es gilt **höchstens 25 % der Kämpfer** (`docs/rules/zielkorridore.md` § 4). Die Session F1.4 durfte `wirtschaft.md` nicht ändern.

## Ziel

Beide Regelwerke nennen denselben, von 🧑 entschiedenen Korridor; der Balancing-Tester (B-099) prüft gegen eine einzige Zahl.

## Beteiligte und Zielgruppen

🧑 hat entschieden; ein Agent gleicht den Text an. BAL2 nutzt die Zahl.

## Anforderungen

- Die Zeile in `wirtschaft.md` § 3 nennt „höchstens 25 % der Kämpfer“ und verweist auf `zielkorridore.md` § 4.

## Nicht-Ziele

Messgröße und Prüfung (B-099, B-157); keine Änderung an `data/*.json`.

## Regeln und Einschränkungen

Domäne REG. Keine neue Zahl; nur die Entscheidung aus F1.4 übernehmen.

## Beispiele

Ein Leser von `wirtschaft.md` § 3 sieht 25 % der Kämpfer, wie in `buerger.md` § 3 und `zielkorridore.md` § 4.

## Ausnahme- und Fehlerfälle

nicht relevant: reine Textangleichung.

## Akzeptanzkriterien

- **AC-01** `docs/rules/wirtschaft.md` § 3 nennt „Median höchstens 25 % der Kämpfer“ mit Verweis auf `zielkorridore.md`; `grep -n "Hälfte der Truppen" docs/rules/wirtschaft.md` findet nichts.

## Offene Fragen

keine

## Notizen

Entscheidung: Workshop F1.4, 2026-10-03.
