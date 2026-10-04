# BAL3.3 · Sensitivitäts-Läufe und Kurven je Schwierigkeitsgrad

- **Status:** offen
- **Typ:** Umsetzung
- **Agent:** autonom
- **Branch:** bal3/3-sensitivitaet-kurven
- **Abhängig von:** BAL3.2
- **Tickets:** B-158
- **Kriterien:** AC-03, AC-04, AC-06

## Ziel

Ein Sensitivitäts-Lauf variiert einen Wert aus `data/*.json` um ±10 % und ±25 % und berichtet die gekippten Kennzahlen; der Bericht enthält je Schwierigkeitsgrad eine Kurve über die Tage.

## Kontext

- **Entsteht in BAL2:** Korridor-Daten und Bewertung (`BAL2.1`), `task balance` mit Bericht (JSON und Markdown) und Regressions-Vergleich, Test-Option des Läufers zum Ändern von Werten ohne Eingriff in `data/*.json` (`BAL2.2`). Die Sensitivität baut darauf auf: „gekippt“ = die Bewertung (im Korridor, knapp, verletzt) ändert sich gegenüber dem Lauf mit unverändertem Wert. Ort und Namen vor dem Start im Code nachsehen.
- **Variation:** Pfad zu einem Zahlenwert (Beispiel aus B-158: `data/economy.json` › `purse`, dort z. B. `startGold`), Stufen ±10 % und ±25 %. Existiert der Pfad nicht, Fehler mit Pfad, kein stiller Lauf. Es wird **nichts automatisch geändert**, nur berichtet; `data/*.json` bleiben unberührt. Welche Werte standardmäßig variiert werden: Beschluss BAL3.1.
- **Grad-Kurven:** Grade aus `data/difficulty.json` (`dev`, `easy`, `normal`, `hard`, `ultra`; Faktoren auf Wellengröße, Gegner-HP und -Schaden). Je Grad Kennzahlen über die Tage als Tabelle im Bericht. **Welche Kennzahlen und wie viele Tage:** Beschluss BAL3.1 (nicht erfinden). Die Grad-Option im Szenario liefert BAL1; fehlt sie, „noch nicht messbar“ und Ticket (SIM). Vergleichswerte: `docs/rules/zielkorridore.md` § 2 (Burg hält Nacht 1–5 je Grad).
- Bericht-Format wie in BAL2.2 (JSON mit fester Feldreihenfolge, Structs statt Maps, plus Markdown); Ausgabeort wie dort (Vorschlag `reports/`).
- Laufzeit: Szenarien × 4 Stufen; ein Flag für weniger Seeds bleibt nötig. Im Ergebnis die Dauer messen.

## Erlaubte Dateien

- `engine/balance/` (Sensitivität, Kurven, Bericht, Tests), `cmd/k3c-balance/` (Befehl und Flags)
- `Taskfile.yml` (nur Task für die Sensitivität, EXE mit festem Pfad, kein `go run`, kein Teil von `task check`)
- `docs/sprints/` (nur Status dieser Session), `docs/backlog/` (nur Status und neue Tickets)

## Nicht-Ziele

Automatische Wertsuche oder Optimierung, Änderung von `data/*.json`, neue Profile (BAL3.2), Abgleich mit echten Abenden (BAL4).

## Schritte

1. Branch anlegen, `Status: in Arbeit`. Beschluss BAL3.1 (Kennzahlen und Tage der Kurven) und Bericht aus BAL2.2 nachlesen.
2. Parameter-Variation über die Test-Option des Läufers aus BAL2.2 (Pfad → Wert ±10 % und ±25 %); unbekannter Pfad → Fehler mit Pfad.
3. Sensitivitäts-Bericht: je Variation die Liste der gekippten Kennzahlen mit Bewertung vorher und nachher.
4. Grad-Kurven: je Grad Tabelle der beschlossenen Kennzahlen über die Tage im Bericht.
5. Tests: Variation um +25 % auf einen Wert, der eine Kennzahl kippt (feste Werte), der Bericht nennt sie; unbekannter Pfad → Fehler; Kurven enthalten jeden Grad aus `data/difficulty.json`; gleiche Eingaben → byte-gleicher Bericht.
6. `task check:dev`. Ein Beispiel-Lauf (Laufzeit, Rechner) ins Ergebnis, `Status: fertig`, Tabelle der Sprint-README.

## Fertig, wenn

- [ ] AC-03: Test belegt, dass ein Lauf mit ±10 % und ±25 % eines Werts einen Bericht mit der Liste gekippter Kennzahlen erzeugt.
- [ ] AC-04: Test belegt, dass der Bericht je Grad aus `data/difficulty.json` eine Kurve über die Tage enthält.
- [ ] AC-06: `task check:dev` grün; `git status` zeigt keine Änderung an `data/*.json`.

## Prüfen

```bash
task check:dev
```

## Ergebnis

–
