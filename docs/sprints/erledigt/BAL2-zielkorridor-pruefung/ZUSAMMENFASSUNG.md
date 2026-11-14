# BAL2 · Zusammenfassung

Regelbasiert aus der Sprint-README erzeugt (`plan_digest`, B-696); maßgeblich bleibt [README.md](README.md).

- **Titel:** Zielkorridor-Prüfung und `task balance`
- **Projekt:** –

## Ziel

`task balance` bewertet jede Kennzahl gegen den Zielkorridor und nennt die Seeds verletzter Ziele. Am Ende sichtbar: ein Bericht mit Pass/Fail je Kennzahl für 100 feste Seeds und ein Vergleich „Wertänderung → welche Ziele kippen“.

## Ergebnis

2026-10-04, Agent (Claude Opus 5.5) in BAL2.4, leichtes Review. AC-01, AC-02: Ergebnis BAL2.1; AC-03 bis AC-05: Ergebnis BAL2.2; AC-06: Ergebnis BAL2.3, CI-Lauf auf dem Sprint-Branch grün, Artefakt `k3c-balance` vorhanden (https://github.com/Tenvan/k3c-family/actions/runs/37204084366); AC-07: `task check`, `task check:go`, `task check:dev` grün. Schwere Befunde: keine; `data/` nur um die neue Datei `data/balance-targets.json` ergänzt (Grenzen laut `TestGrenzenWieTabelle` wie `zielkorridore.md`), keine Werte geändert. Nicht messbare Ziele stehen in BAL2.1. B-157 archiviert, B-099 bleibt eingepl …

## Kriterien

| Kriterium | Status | Inhalt |
|---|---|---|
| AC-01 | – | Korridor-Datei wird geladen, Ziel ohne Kennzahl ergibt Ladefehler (B-157/AC-01). |
| AC-02 | – | Bewertung im Korridor, knapp und verletzt ist korrekt (B-157/AC-02). |
| AC-03 | – | `task balance` läuft über 100 feste Seeds und schreibt Pass/Fail je Kennzahl als JSON und … |
| AC-04 | – | Der Bericht nennt zu jedem verletzten Ziel die Seeds zum Nachspielen (B-157/AC-04). |
| AC-05 | – | Der Regressions-Vergleich nennt die durch eine Wertänderung gekippten Kennzahlen (B-157/AC … |
| AC-06 | – | Ein CI-Lauf mit kleiner Seed-Menge erzeugt den Bericht, ohne das Gate zu brechen (B-157/AC … |
| AC-07 | geprüft | `task check:go` ist grün. |

## Tickets

| Ticket | Status | Kriterien | Titel |
|---|---|---|---|
| B-157 | erledigt | AC-01 –, AC-02 –, AC-03 –, AC-04 –, AC-05 –, AC-06 – | Der Balancing-Tester prüft Zielkorridore und meldet Pass oder Fail je Kennzahl |
