# BAL2 · SIM · Zielkorridor-Prüfung und `task balance`

- **Status:** aktiv
- **Domäne:** SIM
- **Reife:** bereit
- **Einschiebbar:** ja
- **Tickets:** B-157
- **Start-Commit:** 1fa9529
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** 2026-10-04, Chat (Ralf), Revision 1, durch 🧑; umfasst B-157 und die Domänen-Ausnahmen `tools/k3c-dev/` und `.github/`; mit Änderungen aus dem Spec-Review (F1-Frage gestrichen, „knapp“ = 5 pp / 10 % bestätigt)

## Ausgangslage

Nach BAL1 liefert der Tester Kennzahlen, aber keine Bewertung. Die Zielkorridore stehen als Prosa in `docs/rules/*.md`; Zahlen kommen aus F1 (B-134). Details in B-157.

## Ziel

`task balance` bewertet jede Kennzahl gegen den Zielkorridor und nennt die Seeds verletzter Ziele. Am Ende sichtbar: ein Bericht mit Pass/Fail je Kennzahl für 100 feste Seeds und ein Vergleich „Wertänderung → welche Ziele kippen“.

## Beteiligte und Zielgruppen

🧑 beschließt Zahlen der Korridore (REG); Entwickler und Agenten führen `task balance` aus, Review-Sessions der Phasen 2 und 3 lesen den Bericht.

## Anforderungen

B-157 › Anforderungen. Erfüllt zugleich B-099/AC-03, AC-04 und AC-05.

## Nicht-Ziele

Neue Bot-Profile und Sensitivität (BAL3), Abgleich mit echten Abenden (BAL4), Pflicht-Gate in der CI.

## Regeln und Einschränkungen

Deterministisch; Datei ≤ 400 Zeilen, Funktion ≤ 60; Aufgaben nur über `task`; Werte in `data/` ändert nur REG mit Beschluss. Voraussetzung: BAL1 und F1. Breite von „knapp“ (🧑 2026-10-04): 5 Prozentpunkte bei Anteilen, 10 % der Korridorbreite bei Medianen.

**Domänen-Ausnahme (Freigabe dieser Spec erlaubt sie):** Das Balance-Werkzeug liegt laut BAL1 in k3c-dev (`tools/k3c-dev/internal/`, SRV); BAL2.1 und BAL2.2 dürfen dort ändern. BAL2.3 darf für den CI-Lauf `.github/workflows/` (INF) ändern.

## Beispiele

`task balance` → „Überleben Welle 3: 64 %, Ziel 70–95 %: Fail, Seeds 12, 31, 77“.

## Ausnahme- und Fehlerfälle

Ziel ohne Kennzahl → Ladefehler. Fehlende Baseline → Hinweis. Lauf bricht ab → „ungültig“ im Bericht.

## Akzeptanzkriterien

- **AC-01** Korridor-Datei wird geladen, Ziel ohne Kennzahl ergibt Ladefehler (B-157/AC-01).
- **AC-02** Bewertung im Korridor, knapp und verletzt ist korrekt (B-157/AC-02).
- **AC-03** `task balance` läuft über 100 feste Seeds und schreibt Pass/Fail je Kennzahl als JSON und Markdown (B-157/AC-03).
- **AC-04** Der Bericht nennt zu jedem verletzten Ziel die Seeds zum Nachspielen (B-157/AC-04).
- **AC-05** Der Regressions-Vergleich nennt die durch eine Wertänderung gekippten Kennzahlen (B-157/AC-05).
- **AC-06** Ein CI-Lauf mit kleiner Seed-Menge erzeugt den Bericht, ohne das Gate zu brechen (B-157/AC-06).
- **AC-07** `task check:go` ist grün.

## Offene Fragen

keine (Zahlen: `docs/rules/zielkorridore.md`, F1 erledigt, Q02)

## Sessions

| Nr. | Datei | Typ | Agent | Status |
|---|---|---|---|---|
| BAL2.1 | `BAL2.1-korridore-bewertung.md` | Umsetzung | autonom | fertig |
| BAL2.2 | `BAL2.2-task-balance-bericht.md` | Umsetzung | autonom | offen |
| BAL2.3 | `BAL2.3-ci-lauf.md` | Umsetzung | autonom | offen |
| BAL2.4 | `BAL2.4-review.md` | Review | autonom | offen |

Bei `Reife: Entwurf` genügen Stichpunkte mit den Kriterien in Klammern statt Session-Dateien. Vor dem Aktivieren
wird jede Session als eigene Datei nach `docs/vorlagen/session.md` geschrieben.

## Abnahme

–
