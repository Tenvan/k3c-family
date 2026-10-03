# B-134 · Jede Kennzahl des Spiels hat einen Zielkorridor als Zahl

- **Domäne:** REG
- **Typ:** Idee
- **Prio:** hoch
- **Status:** eingeplant
- **Sprint:** F1
- **Erstellt:** 2026-10-02
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

Die Regeln in `docs/rules/wirtschaft.md` und `docs/rules/stufen.md` nennen Zielkorridore nur verstreut und teils als Startziel („Burg hält Nacht 1–5“, „Höhle vor Tag 6 in ≥ 80 %“). Eine gemeinsame Tabelle mit Kennzahl, Szenario, Ober- und Untergrenze fehlt. Ohne sie gibt es für die Regelwerke R2 bis R4 keine Abnahme, und der Balancing-Tester (B-099) hat nichts zu prüfen.

## Ziel

Eine Tabelle `docs/rules/zielkorridore.md` nennt je Kennzahl das Szenario und den Korridor als Zahl. Nutzen: Balancing wird Pass/Fail statt Geschmack; BAL2 (`task balance`) kann die Tabelle als Prüfung lesen.

## Beteiligte und Zielgruppen

🧑 entscheidet die Zahlen (Workshop F1); der Agent liefert einen Vorschlag aus `docs/rules/*.md` und `docs/rules/ist-abgleich.md` › Messgrößen.

## Anforderungen

- Je Zeile: Kennzahl, Szenario (Biom, Grad, Spieleranzahl, Bot-Profil, Seeds), Untergrenze, Obergrenze, Quelle der Regel (`docs/rules/…` §).
- Mindestens diese Kennzahlen: Überleben der Nacht n (Beispiel des Vorschlags: Nacht 3, Grad Normal, 2 Spieler, Bot „sparsam“, Überleben ≥ 80 % der Seeds), Gold am Morgen je Spieler, Zeit bis zur ersten Mauer (Bauzeit), Zeit bis zur Höhle, Abstand der Wellen unter Tage, Tick-Dauer p99.
- Nur Kennzahlen, die der Simulator messen kann oder mit B-099/B-157 bekommt; fehlende Messgrößen stehen in einer eigenen Liste mit Verweis auf B-099.
- Alle Zahlen sind Startwerte und werden durch Balancing-Runden (B-155, B-156) angepasst; Änderungen erhöhen die Revision der Datei im Kopf.

## Nicht-Ziele

Die Prüfung selbst (B-157), Werte in `data/*.json` ändern (B-155, B-156), neue Regeln für Mechaniken.

## Regeln und Einschränkungen

Domäne REG (`docs/rules/`, `docs/game-design.md`). Standardszenario laut `wirtschaft.md`: Wald, 2 Spieler, Bot „sparsam“, je 100 Seeds, wenn nichts anderes steht. Zahlen mit Einheit und Szenario, kein „schnell“ oder „viel“.

## Beispiele

Zeile „Überleben Nacht 3 · Wald, Normal, 2 Spieler, Bot sparsam, 100 Seeds · ≥ 80 % · `wirtschaft.md` § 4“ → der Tester meldet Pass bei 83 von 100 Seeds, Fail bei 74.

## Ausnahme- und Fehlerfälle

Eine Kennzahl ist (noch) nicht messbar → sie steht mit Vermerk „Messgröße fehlt (B-099)“ in der Liste der fehlenden Messgrößen, nicht mit erfundener Zahl in der Tabelle.

## Akzeptanzkriterien

- **AC-01** Datei `docs/rules/zielkorridore.md` existiert; jede Tabellenzeile hat Kennzahl, Szenario, Untergrenze, Obergrenze (Zahlen mit Einheit) und eine Quelle in `docs/rules/` (Sichtprüfung der Datei).
- **AC-02** Die Tabelle enthält mindestens die sechs Kennzahlen aus den Anforderungen, jede mit Zahl (Sichtprüfung).
- **AC-03** 🧑 hat die Zahlen bestätigt; die Bestätigung steht mit Datum im Kopf der Datei (Beschluss Q02, `docs/fragenkatalog.md`).
- **AC-04** `docs/game-design.md` verweist auf `docs/rules/zielkorridore.md` (Suche nach dem Dateinamen); `task check` grün.

## Offene Fragen

Konkrete Zahlen je Kennzahl, besonders Überlebensquote, Gold pro Tag und Bauzeit bis zur Mauer: `docs/fragenkatalog.md` Q02, entscheidet 🧑.

## Notizen

Aus Plan Phase 0 (F1) und Lücke 9 der Lückenliste („Balancing-Ziele sind keine Zahlen“). Spätere Prüfung: B-157.
