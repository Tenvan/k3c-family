# BAL2 · SIM · Zielkorridor-Prüfung und `task balance`

- **Status:** geplant
- **Domäne:** SIM
- **Reife:** Entwurf
- **Einschiebbar:** ja
- **Tickets:** B-157
- **Start-Commit:** –
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

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

Deterministisch; Datei ≤ 400 Zeilen, Funktion ≤ 60; Aufgaben nur über `task`; Werte in `data/` ändert nur REG mit Beschluss. Voraussetzung: BAL1 und F1.

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

- Welche Zahlen haben die Korridore? Entscheidet 🧑 in F1 (`docs/fragenkatalog.md` Q02); blockiert die Freigabe, bis F1 abgeschlossen ist.

## Sessions

Entwurf. Vor dem Aktivieren jede Session als Datei nach `docs/vorlagen/session.md` schreiben, die Kriterien in Klammern werden ihr Feld `Kriterien`.

- BAL2.1 Korridor-Daten laden und Bewertung (im Korridor, knapp, verletzt) (AC-01, AC-02).
- BAL2.2 `task balance`, Bericht JSON und Markdown mit Seeds, Baseline und Regressions-Vergleich (AC-03, AC-04, AC-05).
- BAL2.3 CI-Lauf mit kleiner Seed-Menge (AC-06).
- BAL2.4 Review (AC-07).

## Abnahme

–
