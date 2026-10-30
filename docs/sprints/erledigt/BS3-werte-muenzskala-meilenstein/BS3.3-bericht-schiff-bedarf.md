# BS3.3 · Balancing-Bericht: Schiff bezahlbar ab Tag

- **Status:** fertig
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** DEV
- **Umgebung:** offline
- **Branch:** bs3/3-bericht-schiff-bedarf
- **Abhängig von:** BS3.2, BS1.4, BS11.5
- **Tickets:** B-638
- **Kriterien:** AC-04

## Ziel

Der Balancing-Bericht nennt je Schiff-Abschnitt „bezahlbar ab Tag“ mit Ampel gegen den Korridor – aus dem Vorratsverlauf, ohne Bau.

## Kontext

- Bericht: `tools/k3c-workbench/internal/balance/` (`summary.go`, `economy_measures.go`, Ampeln/Hinweise aus B-632), Hinweise `data/balance-hints.json`.
- Schiffskosten aus BS3.2; Korridor „Schiff fertig“ in `balance-targets.json`.
- Bedarf gegen Bestand: kumulierter Vorrat (Münzen + Material) je Tag gegen kumulierte Abschnittskosten.
- ⚠️ Go-Änderung an der Workbench → die laufende Workbench muss neu starten, bevor `sim_test` den neuen Bericht liefert.

## Erlaubte Dateien

- `tools/k3c-workbench/internal/balance/**`
- `data/balance-hints.json`

## Nicht-Ziele

Bots bauen das Schiff (BS5.4).

## Schritte

1. Kennzahl „Schiff bezahlbar ab Tag“ je Abschnitt aus dem Vorratsverlauf.
2. Ampel gegen Korridor, Tuning-Hinweis in `balance-hints.json`.
3. Unit-Test mit festem Verlauf.
4. `check_run task:check:diff`, Commit.
5. **⚠️ Workbench-Neustart nötig** melden: Ergebnis Zeile 1, `notify_ui` (warn), Chat an 🧑.

## Fertig, wenn

- [ ] AC-04: Unit-Test grün, Bericht-Zeile „Schiff (Bedarf): …“ mit Ampel.
- [ ] Neustart-Hinweis gemeldet.
- [ ] `task check:diff` grün.

## Prüfen

```bash
task check:diff
```

## Ergebnis

**⚠️ Workbench-Neustart nötig** (Go-Code der Workbench geändert, `sim_test` liefert die Zeile erst danach).

- AC-04 geprüft: `ship_need.go` rechnet je Abschnitt (Kiel, Rumpf, Segel) „bezahlbar ab Tag“ aus dem Verlauf (Startgold plus Einnahmen, kumulierter Zufluss des Insel-Vorrats gegen die kumulierten Kosten aus `data/ship.json`); `Metrics.ship` nur im Insel-Lauf. Der Bericht (`kennzahlen.ship`, Markdown) nennt „🟢 Schiff (Bedarf): Kiel bezahlbar T3, Rumpf bezahlbar T4, Segel bezahlbar T7 (Ziel T6–T9)“. Ampel gegen den Korridor aus `zielkorridore.md` § 6: grün im Fenster Endboss-Tag −2 … +1, gelb bis 2 Tage daneben, sonst rot; grau, solange der Endboss nicht besiegt ist. Tests: `TestSchiffBedarfAusDemVerlauf`, `TestSchiffAmpel`, `TestSchiffNurImInselLauf`. `task check:diff` grün.
- Abweichung 1: Der Tuning-Hinweis steht im Bericht-Text („Schiffskosten in data/ship.json erhöhen/senken“), nicht in `data/balance-hints.json`: Das Schema dort kennt nur Kennzahlen mit Bändern und Hebel je Ziel, ein Ziel „Schiff“ gibt es erst mit BS5.
- Abweichung 2 (Fund aus BS11): Das Markdown des Berichts zeigte die Tabellen der Insel-Kennzahlen (Tiefe, Boss, Endboss) bisher nicht, nur das JSON. Die Einbindung in `insights_markdown.go` war im BS11-Commit nie wirksam; jetzt verdrahtet.
- Hinweis: Der Bedarf ist eine Obergrenze („hätten bezahlen können“), weil Gold und Material im Lauf auch in andere Bauten gehen; den echten Tag liefert der Bau ab BS5/BS9.
- Neue Tickets: keine.
