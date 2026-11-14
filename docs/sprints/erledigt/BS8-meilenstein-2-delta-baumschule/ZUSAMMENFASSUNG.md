# BS8 · Zusammenfassung

Regelbasiert aus der Sprint-README erzeugt (`plan_digest`, B-696); maßgeblich bleibt [README.md](README.md).

- **Titel:** Meilenstein 2: Bots und Delta-Runde Baumschule gegen Baseline M1
- **Projekt:** BAS

## Ziel

**Meilenstein 2:** Bots roden und kaufen Baumschulen, und eine Delta-Runde justiert nur die Gruppe `holz` gegen Baseline `M1`, bis `coop2` standard wieder durchläuft; danach wird `M2` gesichert und `holz` eingefroren. **Am Ende sichtbar:** Delta-Bericht `baseline=M1`: `basis` ✅ unverändert, `holz` ✅; Basis-Lauf `M2` im Editor.

## Ergebnis

2026-10-29 (Claude, autonom): AC-01, AC-03 erfüllt (BS8.1, BS8.2), AC-04 teilweise (`M2` gesichert, `frozen` nur `basis`). **AC-02 verschoben: Lauf erreicht sein Ziel nicht, B-656.** Review: BS6.5. Abnahme 🧑 offen.

## Kriterien

| Kriterium | Status | Inhalt |
|---|---|---|
| AC-01 | – | Bots (`coop2` u. a.) roden Baumschul-Plätze und kaufen Baumschulen; Unit-Test (B-636). |
| AC-02 | verschoben | Delta-Lauf `coop2` standard `baseline=M1`: `basis` unverändert, `holz` grün, Lauf läuft du … |
| AC-03 | erfüllt | Geändert wurden nur Hebel der Gruppe `holz` (Diff geprüft). |
| AC-04 | teilweise | Baseline `M2` gesichert, `frozen: [basis, holz]`, im Editor sichtbar. |

## Tickets

| Ticket | Status | Kriterien | Titel |
|---|---|---|---|
| B-636 | eingeplant | AC-01 – | Die Farm heißt Baumschule und steht auf gerodeten Waldbauplätzen |
| B-640 | verworfen | – | Balancing nach Mechanik-Änderungen läuft als Delta-Runde gegen eine Baseline |
