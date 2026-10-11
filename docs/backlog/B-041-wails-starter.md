# B-041 · Wails-Starter für Windows existiert

- **Domäne:** SRV
- **Typ:** Idee
- **Prio:** niedrig
- **Umgebung:** live
- **Status:** eingeplant
- **Sprint:** BT1
- **Projekt:** REL
- **Erstellt:** 2026-09-30
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

Am Windows-PC startet der Server per Konsole.

## Ziel

Wails-Starter für Windows existiert. Nutzen: Einfacher Start am Windows-PC per Doppelklick.

## Beteiligte und Zielgruppen

🧑 betreibt den Server im Heimnetz (PC, später Pi); Spieler verbinden sich mit PC und Xbox; Umsetzung durch Entwickler oder Agent.

## Anforderungen

- Desktop-Fenster (Wails) um denselben Go-Kern, mit Status und Adresse. Der QR-Code entfällt seit 2026-10-11 (Beschluss 🧑: Handy ist kein Gerät mehr).

## Nicht-Ziele

Pflicht für den Betrieb (optional laut Entscheidung 001); Docker-Betrieb.

## Regeln und Einschränkungen

Go-Server ist die einzige Engine (Entscheidung 001), Standardbibliothek zuerst; Schichtgrenzen und Komplexitäts-Budget aus `docs/arbeitsweise.md`.

## Beispiele

Doppelklick auf `k3c-desktop.exe` → Fenster zeigt Status und Adresse, das Spiel ist erreichbar.

## Ausnahme- und Fehlerfälle

Port belegt → Meldung im Fenster statt stillem Absturz.

## Akzeptanzkriterien

- **AC-01** `cmd/k3c-desktop` startet Server und Fenster.

## Offene Fragen

keine

## Notizen

Optional laut Entscheidung 001.
