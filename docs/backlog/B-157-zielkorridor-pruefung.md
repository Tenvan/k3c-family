# B-157 · Der Balancing-Tester prüft Zielkorridore und meldet Pass oder Fail je Kennzahl

- **Domäne:** SIM
- **Typ:** Idee
- **Prio:** mittel
- **Status:** eingeplant
- **Sprint:** BAL2
- **Erstellt:** 2026-10-02
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** 2026-10-04, Chat (Ralf), Revision 1, durch 🧑 (mit BAL2)

## Ausgangslage

B-099 beschreibt den Balancing-Tester als Ganzes; BAL1 liefert den Kern (Szenario-Matrix, Bots, Kennzahlen als JSON, Replay B-159). Die Zielkorridore stehen als Prosa in `docs/rules/wirtschaft.md`, `stufen.md`, `materialien-gebaeude.md`, `monarch.md`, `buerger.md`, `gegner.md` und `bosse.md`; als Daten gibt es sie noch nicht. `Taskfile.yml` kennt keine Aufgabe `balance`.

## Ziel

Die Zielkorridore liegen als Daten vor, der Tester bewertet jede Kennzahl mit Pass/Fail (mit Ampel knapp) und zeigt bei einer Wertänderung in `data/`, welche Ziele kippen. Nutzen: Balancing-Beschlüsse (BR1, BR2) sind messbar abnehmbar statt Geschmackssache.

## Beteiligte und Zielgruppen

🧑 beschließt die Zahlen der Korridore (REG, Q02 in `docs/fragenkatalog.md`); Entwickler und Agenten führen `task balance` aus; die Review-Sessions der Phasen 2 und 3 lesen den Bericht.

## Anforderungen

- Zielkorridore als Daten (z. B. `data/balance-targets.json`, Format legt die Session fest): je Kennzahl Szenario, Untergrenze und Obergrenze für p10–p90 über die Seeds, Bezug auf die Regel-Datei.
- Lauf über 100 feste Seeds (Liste in den Daten, keine Zufallsauswahl); Ergebnis je Kennzahl: im Korridor, knapp (Randbereich, Breite in den Daten), verletzt.
- Bericht maschinenlesbar (JSON) und lesbar (Markdown), je verletztem Ziel Kennzahl, Szenario und die auslösenden Seeds zum Nachspielen (Replay B-159).
- Aufgabe `task balance` in `Taskfile.yml`, Ergebnis nach `reports/` oder `.work/` (Ort legt die Session fest); zunächst kein Pflicht-Gate in `task check`.
- Regressions-Vergleich: Baseline der Kennzahlen (wie die Golden-Dateien) und Ausgabe „Wertänderung → welche Ziele kippen“.

## Nicht-Ziele

Neue Kennzahlen und Bot-Profile (B-158), Abgleich mit echten Spieleabenden (B-160), automatisches Ändern von `data/*.json`, Pflicht-Gate in der CI (später).

## Regeln und Einschränkungen

Deterministisch (`engine/rng`, keine Wanduhr), Bots nur über `PlayerCommand`; Datei ≤ 400 Zeilen, Funktion ≤ 60 (`docs/arbeitsweise.md`); Aufgaben nur über `task`. Voraussetzung: BAL1 (B-099, B-159), Zahlen der Korridore aus F1 (B-134).

## Beispiele

`task balance` → Tabelle „Erste Mauer: Median Tag 1,6, Ziel höchstens Tag 2: Pass · Überleben Welle 3: 64 %, Ziel 70–95 %: Fail, Seeds 12, 31, 77“. Danach `data/economy.json` ändern und erneut ausführen → Vergleich nennt die gekippten Kennzahlen.

## Ausnahme- und Fehlerfälle

Ziel ohne passende Kennzahl → Fehler beim Laden der Daten. Lauf bricht ab → Lauf „ungültig“ im Bericht mit Seed, nicht übergangen. Fehlende Baseline → Hinweis statt Fehler.

## Akzeptanzkriterien

- **AC-01** Test: Die Korridor-Datei wird geladen; ein Ziel ohne passende Kennzahl liefert beim Laden einen Fehler.
- **AC-02** Test: Für eine Beispielkennzahl bewertet der Tester im Korridor, knapp und verletzt korrekt (drei Fälle mit festen Werten).
- **AC-03** `task balance` läuft über 100 feste Seeds und schreibt je Kennzahl Pass/Fail als JSON und als Markdown.
- **AC-04** Der Bericht nennt zu jedem verletzten Ziel Kennzahl, Szenario und die Seeds zum Nachspielen.
- **AC-05** Test: Eine Wertänderung in `data/` ergibt im Regressions-Vergleich die Liste der gekippten Kennzahlen gegenüber der Baseline.
- **AC-06** Ein CI-Lauf mit kleiner Seed-Menge erzeugt den Bericht, ohne bei Verletzung das Gate zu brechen.

## Offene Fragen

keine (Zahlen: `docs/rules/zielkorridore.md`, F1, Q02; Breite „knapp“: 5 pp bei Anteilen, 10 % der Korridorbreite bei Medianen, 🧑 2026-10-04)

## Notizen

Erfüllt die Kriterien AC-03, AC-04 und AC-05 von B-099. Quelle: `docs/plan-weiterentwicklung.md` Schiene B, BAL2.
