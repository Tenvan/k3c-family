# BS3.3 · Balancing-Bericht: Schiff bezahlbar ab Tag

- **Status:** offen
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** DEV
- **Umgebung:** offline
- **Branch:** bs3/3-bericht-schiff-bedarf
- **Abhängig von:** BS3.2, BS1.3
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

Wird am Ende der Session ausgefüllt: Nachweis je Kriterium (`AC-01 geprüft: task check grün`,
`AC-02 verschoben: Grund, B-0NN`), wer manuell geprüft hat, Abweichungen vom Plan, neue Tickets. Bis dahin `–`.
