# BAS · Neue Basis: Münzskala, Baumschule, Hafen

- **Status:** aktiv
- **Rang:** 1
- **Ziel-Tickets:** –

## Ziel

Das Spiel steht auf einer neuen, K2C-artigen Basis: Münzskala 2–30, Baumschule auf gerodeten Waldbauplätzen statt Farm, Hafen-Chunk mit Schiff zum Verlassen der Insel, Schiffs-Ressourcen im Balancing.

**Meilensteine** (je ein durchlaufender Lauf `coop2`, Welt standard, über `sim_test`, als Baseline gesichert):
- **M1** (BS3): neue Werte, Seed `k3c-dev`, Endboss Insel 1 besiegt.
- **M2** (BS8): mit Baumschule, Delta-Runde nur Gruppe `holz` gegen `M1`.
- **M3** (BS9): mit Schiff, Delta-Runde nur Gruppe `schiff` gegen `M2`, Insel 1 per Schiff verlassen.
- **M4** (BS6): Client fertig, Abschluss-Lauf über alle Seeds gegen `M3`.

Zum Schluss bekommen Hafen und Schiff eigene Grafiken (BS10).

Erledigt, wenn M4 erreicht ist und BS6 und BS10 am Gerät abgenommen sind.

## Sprints

| Sprint | Thema | Status |
|---|---|---|
| BS1 | Schneller Balancing-Lauf mit Seed k3c-dev | geplant |
| BS2 | Regelwerk der neuen Basis: Münzskala, Baumschule, Hafen und Schiff | aktiv |
| BS3 | Werte auf der Münzskala: Meilenstein-Lauf coop2 standard läuft durch | geplant |
| BS7 | Delta-Balancing: Werte-Gruppen, Baseline je Meilenstein, Basis-Läufe im Editor | geplant |
| BS4 | Baumschule auf gerodeten Waldbauplätzen (Server) | geplant |
| BS8 | **Meilenstein 2:** Bots und Delta-Runde Baumschule | geplant |
| BS5 | Hafen-Chunk, Schiff bauen, Inselwechsel am Hafen (Server) | geplant |
| BS9 | **Meilenstein 3:** Bots bauen das Schiff, Delta-Runde Schiff | geplant |
| BS6 | Client: Baumschule, Hafen und Schiff, neue Münzskala, **Meilenstein 4** | geplant |
| BS10 | Grafiken für Hafen und Schiff | geplant |

## Nicht-Ziele

Eigenes Balancing für Events, geplante/manuelle Chunks und Bosse (später, eigenes Ticket); weitere neue Grafiken außer Hafen und Schiff (GEB); Spieleabende (BAL).

## Notizen

- Angelegt 2026-10-26 auf Wunsch 🧑, Rang 1. Kette: Basis (BS1, BS2 parallel) → Werte M1 (BS3) → Delta-Werkzeug (BS7) → Mechanik + Meilenstein im Wechsel (BS4 → BS8, BS5 → BS9) → Client M4 (BS6).
- **Kein komplettes Neu-Balancing je Mechanik:** Nach jedem Meilenstein wird die abgenommene Werte-Gruppe eingefroren und der Lauf als Baseline gesichert; der nächste Meilenstein justiert nur die Gruppe seiner Mechanik und vergleicht gegen die letzte Baseline (B-640). Kippt eine eingefrorene Gruppe, ist das ein Befund für 🧑, keine stille Voll-Runde.
- **Alle Simulationen über `sim_test` (MCP).** Wo das Tool fehlt, wird es erweitert (BS1, BS7: Parameter, Beschreibung, `instructions.md`). Durchgelaufene Läufe stehen im Balancing-Editor unter „Beispiele › Basis-Läufe“.
- **⚠️ Workbench-Neustart:** Jede Session, die Go-Code, Instructions oder die eingebettete Oberfläche der Workbench ändert (BS1.1–BS1.3, BS3.3, BS7.1–BS7.3, BS8.1, BS9.1, BS6.4), meldet den Neustart deutlich: erste Ergebniszeile, `notify_ui` (warn), Chat. Folgeläufe prüfen erst `workbench_status` und blockieren sonst.
- **Konflikt:** Aktive BAL-Sprints RG1 und BR2 messen auf der alten Skala; nach BS3 sind ihre Werte hinfällig. Entscheidung 🧑: pausieren/verwerfen oder nach BS3 neu aufsetzen.
- Keine Rücksicht auf alte Spielstände.
