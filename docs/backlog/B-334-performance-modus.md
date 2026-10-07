# B-334 · Der Client misst Leistung in einem Performance-Modus automatisch und überträgt die Werte an den Server

- **Domäne:** CLI
- **Typ:** Idee
- **Prio:** hoch
- **Umgebung:** live
- **Status:** offen
- **Sprint:** –
- **Erstellt:** 2026-10-07
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

Bei der Abnahme N2.4 (2026-10-07, PC, Tastatur, 1 Spieler, `task load` mit 2 Räumen im Hintergrund) lief das Spiel für 🧑 generell flüssig, mit sporadischen Einbrüchen; die Ursache kann auch ein paralleler Build gewesen sein. Leistungswerte (FPS, Snapshot-Abstand, Puffer, Latenz Mittel/p95) zeigt heute nur das Debug-Overlay (`src/scenes/debugOverlay.ts`) zum Ablesen; 🧑: „Manuell macht das keinen Sinn.“ Einbrüche lassen sich so weder festhalten noch einer Ursache zuordnen.

## Ziel

Leistungsprüfungen laufen ohne Ablesen: Ein Performance-Modus sammelt die Werte im Client und legt sie als Bericht beim Server ab, sodass Einbrüche mit Zeitpunkt und Begleitwerten nachlesbar sind (Grundlage für B-194, PF1).

## Beteiligte und Zielgruppen

🧑 als Tester am PC, TV und Xbox; Agenten, die Berichte über k3c-dev (`reports_list`, `report_read`) auswerten.

## Anforderungen

- Ein eigener Modus (z. B. URL-Parameter oder Dev-Schalter) aktiviert die Messung; im normalen Spiel kostet sie nichts.
- Gesammelt je Gerät: Frame-Zeiten (FPS, Ausreißer/Einbrüche mit Zeitpunkt), Abstand der Zustände, Puffer-Verzögerung, Latenz (Mittel, p95), Anzahl Kameras/Spieler, Gerät/Browser.
- Einbrüche werden mit den Begleitwerten im selben Moment festgehalten (z. B. Snapshot-Lücke oder lange Frame-Zeit).
- Übertragung an den Server als Bericht (vorhandener Weg `/api/report` → `reports/`), periodisch oder am Ende, ohne das Spiel zu stören.
- Funktioniert mit 2 Spielern (Split-Screen).
- Optional: feste Messläufe („Performance-Modi“, z. B. Split-Screen 2 Spieler, Nacht mit Welle) für vergleichbare Werte.

## Nicht-Ziele

Behebung von Ruckeln selbst (B-194, PF1); Server-Kennzahlen (MON1/MON2, `/api/metrics`); Dauer-Telemetrie im normalen Spiel.

## Regeln und Einschränkungen

Domäne CLI (Messung im Client); braucht der Server einen neuen Berichtstyp, wird das ein SRV-Ticket bzw. eine eigene Session. Kein `Math.random()`, Logik Phaser-frei in `src/online/` oder `src/scenes/` mit Vitest-Tests. Logs mit Emoji (🐢 zu langsam, 📄 Bericht). Datei ≤ 400, Funktion ≤ 60 Zeilen.

## Beispiele

🧑 startet das Spiel im Performance-Modus, spielt 5 Minuten → in `reports/` liegt ein Bericht mit FPS-Verlauf, p95/p99 der Frame-Zeit, Latenz und einer Liste von Einbrüchen (Zeitpunkt, Frame-Zeit, Snapshot-Abstand). Ein Agent liest ihn mit `report_read`.

## Ausnahme- und Fehlerfälle

Server nicht erreichbar → Werte im Client puffern (begrenzt) und später senden oder verwerfen, nie das Spiel blockieren. Tab im Hintergrund (`document.hidden`) → Messung pausiert, damit keine Schein-Einbrüche entstehen.

## Akzeptanzkriterien

- **AC-01** Ohne Performance-Modus wird nichts gemessen oder gesendet (Test).
- **AC-02** Im Performance-Modus entsteht nach einem Spiel ein Bericht in `reports/` mit FPS/Frame-Zeit (p50, p95, p99), Zustands-Abstand, Puffer, Latenz und Einbrüchen mit Zeitpunkt (Test der Sammel-Funktion + Bericht über `reports_list`).
- **AC-03** Ein Einbruch (lange Frame-Zeit oder Zustands-Lücke) steht mit Begleitwerten im Bericht (Test).
- **AC-04** 🧑 hat einen Messlauf am PC gemacht und den Bericht gesehen.

## Offene Fragen

- Wie wird der Modus eingeschaltet (URL-Parameter, Dev-Menü, eigene Testseite) und welche festen Messläufe gibt es? 🧑
- Reicht `/api/report` oder braucht es einen eigenen Berichtstyp (SRV)? Beim Einplanen klären.
- Verhältnis zu PF1 (B-194): vor PF1.1 einplanen, damit PF1 mit Berichten misst? 🧑

## Notizen

Anmerkung 🧑 bei N2.4 (2026-10-07): Solche Werte sollen automatisch in speziellen Modi im Client gesammelt und übertragen werden; für Performance-Messung braucht es Automatismen und spezielle Performance-Modi.
