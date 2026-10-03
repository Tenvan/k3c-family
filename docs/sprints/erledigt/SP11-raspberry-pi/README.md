# SP11 · SRV · Raspberry Pi

- **Status:** erledigt
- **Domäne:** SRV
- **Reife:** bereit
- **Einschiebbar:** nein
- **Tickets:** B-028, B-035
- **Start-Commit:** 35de802
- **Spec:** freigegeben
- **Revision:** 2
- **Freigabe:** 2026-10-03, Chat (Ralf), Revision 2 (AC-04 nach LT1 verschoben); vorher 2026-10-02, Chat (Ralf), Revision 1; Registry-Frage und genaues Pi-Modell nicht beantwortet, es gelten die Annahmen unter Offene Fragen. Auflage: kein Docker-Build und kein Test am Pi von diesem Rechner aus

## Ausgangslage

Der Server läuft nur auf dem Windows-PC; Docker-Image und Sicherungen gibt es ab SP03. `compose.yaml` baut das Image selbst (`build: .`), ein veröffentlichtes Image für `docker compose pull` fehlt.

## Ziel

Der Server läuft dauerhaft auf dem Raspberry Pi. Am Ende sichtbar: 2er- und 3er-Spiel parallel auf dem Pi, Lastmessung gegen das Ziel aus B-042: **2 Räume × 3 Spieler, Tick-Dauer p99 < 10 ms** bei 30 Hz.

## Beteiligte und Zielgruppen

🧑 richtet den Pi ein und betreibt ihn; die Familie spielt.

## Anforderungen

B-028 (Pi-Teil), B-035 und B-042 › Anforderungen.

## Nicht-Ziele

Server-Suche per QR/mDNS (B-040).

## Regeln und Einschränkungen

Revision 2: Die Lastmessung (AC-04, ehemals SP11.3) ist nach LT1 verschoben; SP11.4 schließt den Sprint ohne sie ab, B-042 bleibt offen (Sprint LT1). Von diesem Rechner aus wird nichts gebaut und der Pi nicht getestet (🧑, 2026-10-02): SP11.1 ändert nur Dateien, geprüft wird mit `task check`. Die Einrichtung und die Lastmessung am Pi macht 🧑 (Sessions mit `Agent: Mensch`). Modell laut 🧑: Pi 3 oder älter; das Image gibt es nur für amd64 und arm64, der Pi braucht also ein 64-Bit-Betriebssystem. **Domänen-Ausnahme (INF):** SP11.1 darf `.github/workflows/release.yml` ändern, sonst bleibt `docker compose pull` unerfüllbar; die Freigabe dieser Spec erlaubt das.

## Beispiele

2er- und 3er-Spiel parallel auf dem Pi → Tick-Dauer p99 im Ziel aus B-042.

## Ausnahme- und Fehlerfälle

Ziel verfehlt → Messwerte und Befund als Ticket, keine stille Absenkung des Ziels. Ein Pi 3 ist deutlich schwächer als ein PC; ein Verfehlen ist möglich (ungeprüft).

## Akzeptanzkriterien

- **AC-01** Nach einem Neustart des Pi ist der Server erreichbar (B-035/AC-01).
- **AC-02** Ein Update per `docker compose pull` bringt die neue Version (B-035/AC-02).
- **AC-03** Die Spielstände liegen im Volume und werden gesichert (B-028/AC-03).
- **AC-04** *verschoben* nach LT1 (Revision 2, 2026-10-03): Die Lastmessung mit 2er- und 3er-Spiel parallel gegen das Ziel aus B-042 wartet auf das Lasttest-Werkzeug (B-175, Sprint LT1, B-175/AC-06). Grund: Die Handmessung ist aufwendig und nicht wiederholbar; die erste Handmessung (Tag 8,0 bis 9,5 ms, Nacht 10,2 bis 10,3 ms) liegt knapp am Ziel.

## Offene Fragen

- **Registry (Annahme bis zur Antwort von 🧑):** Vorschlag `ghcr.io/tenvan/k3c-family`. Ist das Paket öffentlich (Pi zieht ohne Anmeldung) oder bleibt es privat (Pi braucht `docker login` mit Token)? Entscheidet 🧑, Sichtbarkeit des Repos ungeprüft.
- **Pi-Modell genau:** „Pi 3 oder älter“ (🧑 2026-10-02). Ein Pi 2 oder älter ist mit arm64-Image nicht möglich; gemeint ist vermutlich Pi 3 (B/B+) mit 64-Bit-Betriebssystem. 🧑 bestätigt mit der Freigabe.

## Sessions

| Nr. | Datei | Typ | Agent | Status |
|---|---|---|---|---|
| SP11.1 | `SP11.1-image-und-compose.md` | Umsetzung | autonom | fertig |
| SP11.2 | `SP11.2-pi-einrichten.md` | Workshop | Mensch | fertig |
| SP11.4 | `SP11.4-review.md` | Review | autonom | fertig |

## Abnahme

2026-10-03: AC-01 bis AC-03 geprüft durch 🧑 am Pi (SP11.2-Ergebnis), AC-02 vorbereitet in SP11.1. AC-04 `verschoben` nach LT1 (B-175, Revision 2), B-042 bleibt offen.
Review ohne schweren Befund: `GITHUB_TOKEN` nur für `docker/login-action`, kein Secret im Image oder in `compose.yaml`, Volume `/data`, Rückfall im README. Neue Tickets: keine.
