# B-1295 · Abnahme am PC: Rust-Workbench und Rust-Studio (Fenster, Dev-Betrieb, MCP, Bauzeit)

- **Domäne:** DEV
- **Typ:** Frage
- **Prio:** mittel
- **Umgebung:** offline
- **Status:** offen
- **Sprint:** RU71
- **Projekt:** RSU
- **Erstellt:** 2026-11-26
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** Klasse A, Claude
- **Duplikat von:** –
- **Ersetzt:** –

## Ausgangslage

Workbench und Studio haben mit RU69 und RU70 ein Rust-Backend auf Tauri 2 (Entscheidung 014). Der Second Worker setzt ohne Haltepunkte um (🧑 2026-11-26); RU71 löscht danach Go. Was nur am PC prüfbar ist, sammelt dieses Ticket.

## Ziel

Ralf hat Rust-Workbench und Rust-Studio im Alltag geprüft; Befunde sind Tickets.

## Beteiligte und Zielgruppen

🧑 prüft am PC (Gerät: PC); der Hauptrechner legt aus Befunden Tickets an.

## Anforderungen

1. `task k3c-workbench:run` und `task k3c-studio:run`: Fenster öffnen, Icon, Größe und Position bleiben nach Neustart, nur eine Instanz.
2. Dev-Betrieb: `task k3c-workbench` und `task k3c-studio` (Browser mit Vite) und `:window` (Tauri dev): Hot Reload im Frontend, Neustart bei Rust-Änderung.
3. Claude Code verbindet sich mit `k3c-workbench` (5180) und `k3c-studio` (5181); Stichprobe `plan_list`, `check_run task:check:diff`, `world_list`, `assets_list`.
4. Seiten der Workbench (Dienste, Tasks, Planung, Git, MCP, Monitoring, Spieltest) und des Studios (Welten, Hub, Assets, Balancing) zeigen echte Daten.
5. Bauzeit kalt und warm (`task buildall`) und Speicher im Leerlauf notieren.

## Nicht-Ziele

Neue Funktionen; Rückbau nach Go (Go ist nach RU71 entfernt, Rückfall nur über Git).

## Regeln und Einschränkungen

Manuelle Abnahme nur durch 🧑.

- **Bereich:** –
- **Abhängig von:** RU71
- **Braucht 🧑:** Abnahme
- **Größe:** S
- **Sperre:** –

## Beispiele

Fenster schließen und neu öffnen → gleiche Größe und Position.

## Ausnahme- und Fehlerfälle

Startet eine App nicht: Log unter `logs/` an den Hauptrechner, Ticket mit Prio hoch.

## Akzeptanzkriterien

- **AC-01** 🧑 hat Anforderungen 1 bis 4 für Workbench und Studio mit „ok“ oder Befund eingetragen.
- **AC-02** Bauzeit und Speicher (Anforderung 5) stehen in den Notizen.

## Offene Fragen

keine

## Notizen

–
