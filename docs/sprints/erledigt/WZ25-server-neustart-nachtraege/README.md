# WZ25 · SRV, DEV · Nachträge zum Server-Neustart: Räume bleiben offen, Assets ohne Build, Werkzeuge passen dazu

- **Status:** erledigt
- **Projekt:** WZG
- **Domäne:** SRV, DEV
- **Reife:** bereit
- **Tickets:** B-508, B-509, B-510, B-512, B-511
- **Start-Commit:** – (wird beim Aktivieren gesetzt: `git rev-parse --short origin/develop`)
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** 2026-10-22, autonom, Claude (Auftrag Ralf: autonom abarbeiten), Revision 1; Defaults: B-509 public/ vor dist/ mit Caching nach Inhalt (Last-Modified, no-cache), B-508 Merkdatei restart-rooms.state im Spielstand-Ordner

## Ausgangslage

Seit TS1.4 (B-507) beobachtet der Spielserver `data/` und `public/` und startet nach Änderungen selbst neu. Danach landen Geräte in der Raumliste statt im Raum (B-508). Ausgeliefert wird `dist/`, im Studio geänderte Grafiken und Sounds erreichen die Clients erst nach `task build` (B-509). k3c-dev startet den Spielserver zusätzlich selbst neu und umgeht Prüfung und Hinweis (B-510); einen übernommenen Server beendet es nach dessen Selbst-Neustart nicht mehr (neue PID, `stopAdopted`, B-512). Nicht alle Schreibwege des Studios in beobachtete Ordner sind atomar (z. B. `assets.ImportFile`, B-511).

## Ziel

Ein Neustart des Servers nach einer Studio-Änderung unterbricht das Spiel nur kurz: Räume sind danach wieder offen, neue Grafiken und Sounds sind sofort da, und die Werkzeuge spielen sauber mit. Am Ende sichtbar: Grafik im Studio ändern → Geräte kehren nach dem Neustart in ihren Raum zurück und zeigen die neue Grafik ohne `task build`.

## Beteiligte und Zielgruppen

🧑 beim Arbeiten im Studio neben laufendem Spiel; Spielende am TV; Agent in SRV und DEV; 🧑 gibt die Spec frei.

## Anforderungen

B-508, B-509, B-510, B-512, B-511 › Ausgangslage (Specs noch Entwurf, Anforderungen beim Bereitmachen ausformulieren).

## Nicht-Ziele

Hot Reload ohne Neustart im Server; Änderungen am Supervisor über das hinaus, was B-512 braucht; neue Studio-Funktionen.

## Regeln und Einschränkungen

Reihenfolge SRV → DEV. Entscheidung 004 (Studio und Server getrennt, Schnittstelle sind die Dateien). Gemerkte Räume nicht in beobachteten Ordnern. Wiederverbinden binnen 60 s (`docs/rules/bedienung.md` › Verbindungsverlust). Release-Server ohne Dev-Reste. Firewall: feste EXE-Pfade, Proben nur auf Loopback.

## Beispiele

Raum `ABCD` offen, Studio speichert `data/hub.json` → Server prüft, startet neu, öffnet `ABCD` wieder, Geräte verbinden sich ohne Raumliste. k3c-dev mit übernommenem Server: Stopp nach Selbst-Neustart beendet Supervisor und Kind.

## Ausnahme- und Fehlerfälle

Ungültiger Stand → alter Server läuft weiter, Räume bleiben, nichts wird neu geöffnet. Gemerkter Raum ohne Spielstand → wird nicht geöffnet, Log 🚫. Absturz statt geplantem Neustart → keine gemerkten Räume.

## Akzeptanzkriterien

- **AC-01** Nach einem Neustart wegen geänderter Dateien sind die vorher offenen Räume wieder offen, Geräte kehren binnen 60 s ohne Raumliste zurück (Go-Test, B-508).
- **AC-02** Eine im Studio geänderte Grafik oder ein Sound erreicht die Clients nach dem Neustart ohne `task build`, Caching verhindert keine neue Fassung (B-509).
- **AC-03** k3c-dev startet den Spielserver bei Änderungen in `data/` nicht mehr selbst neu (B-510).
- **AC-04** k3c-dev beendet einen übernommenen Spielserver auch nach dessen Selbst-Neustart vollständig (B-512).
- **AC-05** Jede Datei, die das Studio in `data/` oder `public/` schreibt, entsteht per Temp-Datei und Umbenennen (Test je Schreibweg, B-511).

## Offene Fragen

🧑: B-509 Auslieferung aus `public/` vor `dist/` (mit Caching nach Inhalt) oder Kopie beim Start? B-508 Wo liegen die gemerkten Räume (Vorschlag: neben `saves/`)?

## Sessions

| Nr. | Datei | Typ | Agent | Status |
|---|---|---|---|---|
| WZ25.1 | `WZ25.1-raeume-wieder-oeffnen.md` | Umsetzung | autonom | fertig |
| WZ25.2 | `WZ25.2-public-ohne-build.md` | Umsetzung | autonom | fertig |
| WZ25.3 | `WZ25.3-k3c-dev-server.md` | Umsetzung | autonom | fertig |
| WZ25.4 | `WZ25.4-studio-schreibt-atomar.md` | Umsetzung | autonom | fertig |
| WZ25.5 | `WZ25.5-review.md` | Review | autonom | fertig |

Entwurf. Vor dem Aktivieren jede Session als Datei nach `docs/vorlagen/session.md` schreiben, die Kriterien in Klammern werden ihr Feld `Kriterien`.

- WZ25.1 (SRV) Räume nach geplantem Neustart wieder öffnen (AC-01).
- WZ25.2 (SRV) `public/` ohne neuen Build ausliefern (AC-02).
- WZ25.3 (DEV) k3c-dev: Neustart dem Server überlassen, übernommenen Server sicher beenden (AC-03, AC-04).
- WZ25.4 (DEV) Studio schreibt atomar (AC-05).
- WZ25.5 (DEV) Review (Code-Sprint): alle Kriterien prüfen.

## Abnahme

2026-10-22: AC-01 bis AC-05 nachgewiesen (WZ25.1–WZ25.5), Go-Tests und Vitest grün, Reviewer ohne blockierende Befunde (Merkdatei-Alter, Punkt-Ordner, Temp-Dateien im Watcher behoben). Für Ralf im Nachgang: B-509 `public/` vor `dist/` mit Last-Modified/no-cache; B-508 Merkdatei `restart-rooms.state` (2 min gültig); Rückkehr der Geräte in den Raum am echten Gerät nicht geprüft (manuelle Abnahme).
