# BS7 · Zusammenfassung

Regelbasiert aus der Sprint-README erzeugt (`plan_digest`, B-696); maßgeblich bleibt [README.md](README.md).

- **Titel:** Delta-Balancing: Werte-Gruppen, Baseline je Meilenstein, Basis-Läufe im Editor
- **Projekt:** BAS

## Ziel

Nach jeder Mechanik-Änderung reicht eine Delta-Runde: abgenommene Werte-Gruppen bleiben eingefroren, nur die Gruppe der neuen Mechanik wird justiert, verglichen mit der Baseline des letzten Meilensteins. Durchgelaufene Läufe sind im Balancing-Editor unter „Beispiele“ als Basis-Läufe wählbar. **Am Ende sichtbar:** Baseline `M1` gesichert; `sim_test start bots=coop2 world=standard baseline=M1` liefe …

## Ergebnis

–

## Kriterien

| Kriterium | Status | Inhalt |
|---|---|---|
| AC-01 | – | Jedes Ziel und jeder Hebel hat genau eine Werte-Gruppe, `frozen` steuerbar; Schema-Test (B … |
| AC-02 | – | `sim_test` nimmt `baseline`; ein durchgelaufener Bericht lässt sich mit Konfiguration als … |
| AC-03 | – | Delta-Bericht: Ampel und Trend je Gruppe, eingefrorene nur beim Kippen; Hinweise nur für f … |
| AC-04 | – | Balancing-Editor › „Beispiele“ listet Basis-Läufe; „Übernehmen“ setzt Konfiguration und Ba … |
| AC-05 | – | Baseline `M1` aus dem Meilenstein-Lauf von BS3.4 ist gesichert, Gruppe `basis` eingefroren … |

## Tickets

| Ticket | Status | Kriterien | Titel |
|---|---|---|---|
| B-640 | verworfen | AC-01 –, AC-02 –, AC-03 –, AC-04 – | Balancing nach Mechanik-Änderungen läuft als Delta-Runde gegen eine Baseline |
