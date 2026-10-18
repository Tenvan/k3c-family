# B-041 · Wails-Starter für Windows existiert

- **Domäne:** DEV
- **Typ:** Idee
- **Prio:** niedrig
- **Umgebung:** live
- **Status:** eingeplant
- **Sprint:** BT1
- **Projekt:** REL
- **Erstellt:** 2026-09-30
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** 2026-10-18, Chat (Ralf), Revision 1

## Ausgangslage

Am Windows-PC startet der Server per Konsole (`bin/k3c-server`, `task start`). Wails-Fenster gibt es schon für `tools/k3c-dev` (Workbench) und `tools/k3c-studio` (Asset-Studio, eigenes Go-Modul, `replace` auf das Hauptmodul, EXE in `bin/`, Wails-Build in der CI). Ein Starter für den Spielserver fehlt. Beschluss 🧑 (2026-10-18): Der Starter liegt als `tools/k3c-starter`, die Session gehört zur Domäne DEV.

## Ziel

Wails-Starter für Windows existiert. Nutzen: Einfacher Start am Windows-PC per Doppelklick.

## Beteiligte und Zielgruppen

🧑 betreibt den Server im Heimnetz (PC, später Pi); Spieler verbinden sich mit PC und Xbox; Umsetzung durch Entwickler oder Agent.

## Anforderungen

- Wails-Fenster `tools/k3c-starter` (eigenes Go-Modul wie k3c-dev und Asset-Studio) startet den Spielserver aus dem Kern von `engine/` und `cmd/k3c-server` (keine Kopie der Logik) und zeigt Status und Adresse im LAN. Der QR-Code entfällt seit 2026-10-11 (Beschluss 🧑: Handy ist kein Gerät mehr).
- Task `k3c-starter:build` baut die EXE nach `bin/k3c-starter.exe` (fester Pfad, damit die Windows-Firewall nicht jedes Mal fragt); die CI baut sie wie k3c-dev mit `wails build -s`.

## Nicht-Ziele

Pflicht für den Betrieb (optional laut Entscheidung 001); Docker-Betrieb.

## Regeln und Einschränkungen

Domäne DEV (`tools/`); Go-Server bleibt die einzige Engine (Entscheidung 001), Standardbibliothek zuerst, Schichtgrenzen und Komplexitäts-Budget aus `docs/arbeitsweise.md`. Keine neue Abhängigkeit außer Wails (bereits im Repo). Feste EXE mit festem Pfad, Proben nur auf Loopback.

## Beispiele

Doppelklick auf `bin/k3c-starter.exe` → Fenster zeigt „läuft“ und die Adresse (`http://<LAN-IP>:8080`), das Spiel ist von PC und Xbox aus erreichbar.

## Ausnahme- und Fehlerfälle

Port belegt → Meldung im Fenster statt stillem Absturz.

## Akzeptanzkriterien

- **AC-01** `tools/k3c-starter` startet Server und Fenster; `go test ./...` und `golangci-lint` im Modul grün, `wails build -s` läuft in der CI.
- **AC-02** Doppelklick auf `bin/k3c-starter.exe` zeigt Status und Adresse, Port belegt → Meldung im Fenster (Abnahme durch 🧑 am PC, Umgebung live).

## Offene Fragen

keine

## Notizen

Optional laut Entscheidung 001.
