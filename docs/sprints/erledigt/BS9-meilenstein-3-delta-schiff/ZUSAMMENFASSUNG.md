# BS9 · Zusammenfassung

Regelbasiert aus der Sprint-README erzeugt (`plan_digest`, B-696); maßgeblich bleibt [README.md](README.md).

- **Titel:** Meilenstein 3: Bots bauen das Schiff, Delta-Runde Schiff gegen Baseline M2
- **Projekt:** BAS

## Ziel

**Meilenstein 3:** Bots bauen das Schiff, der Bericht nennt den Bau-Tag je Abschnitt, und eine Delta-Runde justiert nur die Gruppe `schiff` gegen `M2`, bis `coop2` standard Insel 1 per Schiff verlässt; danach `M3` sichern. **Am Ende sichtbar:** Delta-Bericht `baseline=M2`: `basis` und `holz` unverändert, `schiff` ✅, Inselwechsel im Lauf; Basis-Lauf `M3` im Editor.

## Ergebnis

2026-10-29 (Claude, autonom): AC-01, AC-03 erfüllt (BS9.1, BS9.2), AC-04 teilweise (`M3` gesichert). **AC-02 verschoben: `schiff` wird nicht grün, Lauf steckt in Stufe 4, B-656.** Review: BS6.5. Abnahme 🧑 offen.

## Kriterien

| Kriterium | Status | Inhalt |
|---|---|---|
| AC-01 | – | Bots bauen Schiff-Abschnitte; Bericht nennt Bau-Tag je Abschnitt mit Ampel (B-638/AC-03). |
| AC-02 | verschoben | Delta-Lauf `coop2` standard `baseline=M2`: `basis`, `holz` unverändert, `schiff` grün, Ins … |
| AC-03 | erfüllt | Geändert wurden nur Hebel der Gruppe `schiff`. |
| AC-04 | teilweise | Baseline `M3` gesichert, alle Gruppen eingefroren, im Editor sichtbar. |

## Tickets

| Ticket | Status | Kriterien | Titel |
|---|---|---|---|
| B-638 | verworfen | AC-01 – | Das Balancing misst die Ressourcen für das Boot |
| B-640 | verworfen | – | Balancing nach Mechanik-Änderungen läuft als Delta-Runde gegen eine Baseline |
