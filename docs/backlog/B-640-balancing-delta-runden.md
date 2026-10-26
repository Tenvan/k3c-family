# B-640 · Balancing nach Mechanik-Änderungen läuft als Delta-Runde gegen eine Baseline

- **Domäne:** DEV
- **Typ:** Idee
- **Prio:** hoch
- **Umgebung:** offline
- **Status:** eingeplant
- **Sprint:** BS7
- **Projekt:** BAS
- **Erstellt:** 2026-10-26
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** 2026-10-26, Chat, durch 🧑 mit BS7

## Ausgangslage

Nach Meilenstein 1 (BS3) ändern Baumschule (BS4) und Hafen/Schiff (BS5) die Wirtschaft erneut. Ohne Struktur wäre jedes Mal eine komplette Werte-Runde über alle Größen nötig. Das Balancing kennt schon eine Baseline (`tools/k3c-workbench/internal/balance/insights.go` › `BaselineView`, Trend gegen Korridor) und eine Sensitivitätsanalyse (`sensitivity.go`).

## Ziel

Nach jeder Mechanik-Änderung reicht eine **Delta-Runde**: Werte sind in Gruppen geteilt, nach einem Meilenstein sind die abgenommenen Gruppen eingefroren, justiert werden nur die Gruppen der neuen Mechanik, und der Bericht vergleicht gegen die Baseline des letzten Meilensteins.

## Beteiligte und Zielgruppen

Claude (Werte-Runden), 🧑 (Abnahme je Meilenstein).

## Anforderungen

- **Werte-Gruppen** in `data/balance-targets.json` (Vorschlag `groups`): `basis` (Mauern, Türme, Truppen, Hub, Kampf, Nächte), `holz` (Bäume, `markCost` Holz, Baumschule), `schiff` (Schiffskosten, Endspiel-Korridor). Jedes Ziel und jeder Hebel in `balance-hints.json` gehört zu genau einer Gruppe.
- **Baseline je Meilenstein:** `sim_test` nimmt `baseline` (Bericht-ID oder Name wie `M1`); ein durchgelaufener Bericht wird unter festem Namen gesichert (`reports/baseline/<name>/`, mit Lauf-Konfiguration: Bots, Welt, Seeds, Seed-Modus).
- **Delta-Bericht:** je Gruppe Ampel und Trend gegen die Baseline; eingefrorene Gruppen nur, wenn sie kippen (Warnung „eingefrorene Gruppe kippt“).
- Tuning-Hinweise nennen nur Hebel der freien Gruppen (`frozen` in `balance-targets.json`).
- Kippt eine eingefrorene Gruppe, ist das ein Befund (Ticket, 🧑 fragen), keine stille Voll-Runde.
- **Basis-Läufe im Balancing-Editor:** Der Dialog „Beispiele“ (`tools/k3c-workbench/frontend/src/balancing/Examples.tsx`, heute nur feste `EXAMPLES` aus `help.ts`) zeigt zusätzlich einen Abschnitt „Basis-Läufe“ mit den gesicherten, durchgelaufenen Läufen (Name, Datum, Konfiguration, Ergebnis). „Übernehmen“ setzt deren Konfiguration und die Baseline in den Editor.
- Alles über `sim_test` (MCP), Seed-Modus aus B-634.

## Nicht-Ziele

Automatisches Tuning; neue Bot-Profile.

## Regeln und Einschränkungen

Domäne DEV für Werkzeug; Gruppen-Zuordnung der Ziele ist REG-Inhalt, wird aber als Schema hier angelegt. ⚠️ Go-Änderung → Workbench-Neustart deutlich melden.

## Beispiele

Nach BS4: `sim_test start bots=coop2 world=standard baseline=M1` → Bericht: „basis ✅ unverändert, holz 🟡 Holz Tag 3 +40 % (Hebel: nursery.growSeconds, nursery.cost), schiff ✅“.

## Ausnahme- und Fehlerfälle

Baseline fehlt → Lauf ohne Vergleich mit Hinweis. Ziel ohne Gruppe → Schema-Test schlägt fehl.

## Akzeptanzkriterien

- **AC-01** Jedes Ziel und jeder Hebel hat eine Gruppe; Schema-Test prüft das.
- **AC-02** `sim_test` nimmt `baseline`; ein Bericht lässt sich als Baseline unter Namen sichern (mit Konfiguration); Tool-Beschreibung und `instructions.md` erklären die Delta-Runde.
- **AC-03** Delta-Bericht zeigt Ampel und Trend je Gruppe, eingefrorene Gruppen nur beim Kippen; Unit-Test.
- **AC-04** Tuning-Hinweise nennen nur Hebel freier Gruppen.
- **AC-05** Im Balancing-Editor listet „Beispiele“ die gesicherten Basis-Läufe; „Übernehmen“ setzt Konfiguration und Baseline (Frontend-Test mit Mock, API-Test im Backend).

## Offene Fragen

- Zuschnitt der Gruppen (Vorschlag oben) → 🧑, Default: Vorschlag.

## Notizen

Wunsch 🧑 2026-10-26: „Nach BS4 und BS5 muss das Balancing neu überarbeitet werden … optimal, dass nicht jedes Mal ein komplett neues Balancing gemacht werden muss.“ Ergänzt: „Durchlaufende Simulationen im Balancing-Editor bei den Beispielen als Basics auswählbar.“
