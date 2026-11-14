# BS1 · Zusammenfassung

Regelbasiert aus der Sprint-README erzeugt (`plan_digest`, B-696); maßgeblich bleibt [README.md](README.md).

- **Titel:** Schneller Balancing-Lauf mit Seed k3c-dev
- **Projekt:** BAS

## Ziel

Ein Balancing-Lauf `coop2` standard startet sauber als MCP-Tool (`sim_test`) und rechnet bis zur Abnahme der neuen Werte nur den Seed `k3c-dev`, in wenigen Minuten; die Prompt-Buttons der Workbench schicken kurze Prompts. **Am Ende sichtbar:** `sim_test start bots=coop2 world=standard` (offline) → Bericht „Seed-Modus: dev (1 Seed: k3c-dev)“; Laufzeit vorher/nachher in B-634; Button an einer Sessio …

## Ergebnis

2026-10-26, Claude (Review BS1.4, auf `feature/bas-remake`, kein PR). AC-01–AC-06 siehe BS1.1/BS1.2 (Laufzeit k3c-dev 14 min → 4 min, parallele Läufe), AC-07/AC-08 siehe BS1.3. `world` für `sim_test` → BS11 (B-642). `check:race` lokal übersprungen. Behoben: flackernder Test `TestSimTestOfflineBewertetZiele`. Neue Tickets: B-642.

## Kriterien

| Kriterium | Status | Inhalt |
|---|---|---|
| AC-01 | – | `seeds[0]` ist `k3c-dev`, ein Test prüft das (B-634/AC-01). |
| AC-02 | – | `seedMode: dev` → `sim_test`-Lauf rechnet genau einen Seed, Bericht nennt den Modus (B-634 … |
| AC-03 | – | Laufzeit `coop2` standard vorher/nachher notiert, nachher deutlich kürzer (B-634/AC-03). |
| AC-04 | – | `seedMode: all` verhält sich wie bisher, Tests grün (B-634/AC-04). |
| AC-05 | teilweise | `sim_test` nimmt offline `bots`, `seedList`, `world`; Tool-Beschreibung und `instructions. … |
| AC-06 | – | Workbench-Neustart deutlich gemeldet, Messlauf erst danach über das neue Tool (B-634/AC-06 … |
| AC-07 | – | Prompt-Buttons schicken eine Zeile ohne Anweisungen, Tests prüfen die Beispiele (B-639/AC- … |
| AC-08 | – | Skill `k3c-session` erkennt die Kurzform; Neustart gemeldet (B-639/AC-02, AC-03). |

## Tickets

| Ticket | Status | Kriterien | Titel |
|---|---|---|---|
| B-634 | erledigt | AC-01 –, AC-02 –, AC-03 –, AC-04 –, AC-05 teilweise, AC-06 – | Balancing läuft bis zur Abnahme nur mit Seed k3c-dev und ist schneller |
| B-639 | erledigt | AC-07 –, AC-08 – | Prompt-Buttons der Workbench schicken nur kurze Prompts ohne Anweisungen |
