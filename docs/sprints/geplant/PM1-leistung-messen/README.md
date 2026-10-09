# PM1 · CLI · Leistung messen: Diagnose-Zeile und Performance-Modus

- **Status:** geplant
- **Projekt:** LST
- **Domäne:** CLI
- **Reife:** Entwurf
- **Tickets:** B-351, B-334
- **Start-Commit:** –
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

FPS, Zustands-Abstand, Puffer und Latenz zeigt nur das Debug-Overlay (`src/scenes/debugOverlay.ts`) zum Ablesen. `sim_test` mit Clients (TR1.3) findet deshalb keine Diagnose-Zeilen im Client-Log (B-351), und Einbrüche wie bei N2.4 lassen sich weder festhalten noch einer Ursache zuordnen (B-334).

## Ziel

Leistungswerte der Clients landen ohne Ablesen beim Server: regelmäßig im Client-Log für `sim_test` und auf Wunsch als Bericht mit Einbrüchen. Am Ende sichtbar: Der Status von `sim_test` zeigt Client-FPS und Latenz, und nach einem Spiel im Performance-Modus liegt ein Bericht in `reports/` (`report_read`).

## Beteiligte und Zielgruppen

Agenten und 🧑 bei Testläufen (`sim_test`, `reports_list`, `report_read`); PF1 (B-194) misst später mit diesen Werten. 🧑 klärt die offenen Fragen aus B-334 und nimmt am PC ab.

## Anforderungen

B-351 › Anforderungen; B-334 › Anforderungen. Sprint-eigen: Diagnose-Zeile und Performance-Modus verwenden dieselbe Sammel-Funktion; die Werte sind dieselben wie im Debug-Overlay.

## Nicht-Ziele

Ruckeln beheben (B-194, PF1); Server-Kennzahlen (MON1/MON2, `/api/metrics`); Dauer-Telemetrie im normalen Spiel; neue Anzeige im Spiel.

## Regeln und Einschränkungen

- Domäne CLI (`src/online/`, `src/scenes/`). Braucht der Bericht einen neuen Berichtstyp im Server, wird das ein SRV-Ticket und kein Teil dieses Sprints (B-334 › Regeln).
- Die Drosselung von `clientLog` darf die Diagnose-Zeile nicht schlucken (B-351 › Regeln).
- Sammel-Logik Phaser-frei mit Vitest-Tests, kein `Math.random()`. Logs mit Emoji (🐢, 📄). Datei ≤ 400, Funktion ≤ 60 Zeilen, keine neue Abhängigkeit.
- Ein Spieler und zwei Spieler (Split-Screen) messen gleich.

## Beispiele

B-351 › Beispiele; B-334 › Beispiele.

## Ausnahme- und Fehlerfälle

B-351 › Ausnahme- und Fehlerfälle; B-334 › Ausnahme- und Fehlerfälle (Server nicht erreichbar: begrenzt puffern; Tab im Hintergrund: Messung pausiert).

## Akzeptanzkriterien

- **AC-01** Die Diagnose-Meldung entsteht im Takt mit `ctx` `{fps, latencyMs, bufferMs}` (`B-351/AC-01`, Test).
- **AC-02** `sim_test` mit einem Client zeigt FPS und Latenz im Status (`B-351/AC-02`, Nachweis im Browser durch 🧑).
- **AC-03** Ohne Performance-Modus wird nichts gemessen oder gesendet (`B-334/AC-01`, Test).
- **AC-04** Im Performance-Modus entsteht ein Bericht in `reports/` mit Frame-Zeit (p50, p95, p99), Zustands-Abstand, Puffer, Latenz und Einbrüchen (`B-334/AC-02`).
- **AC-05** Ein Einbruch steht mit Begleitwerten im Bericht (`B-334/AC-03`, Test).
- **AC-06** 🧑 hat einen Messlauf am PC gemacht und den Bericht gesehen (`B-334/AC-04`).

## Offene Fragen

- Wie schaltet man den Performance-Modus ein (URL-Parameter, Dev-Menü, eigene Testseite), und welche festen Messläufe gibt es? 🧑, blockiert PM1.2.
- Reicht `/api/report`, oder braucht es einen eigenen Berichtstyp (dann ein SRV-Ticket vorab)? Klärt PM1.2 beim Planen.
- Soll PM1 vor PF1.1 laufen, damit PF1 mit Berichten misst? 🧑

## Sessions

Entwurf. Vor dem Aktivieren jede Session als Datei nach `docs/vorlagen/session.md` schreiben, die Kriterien in Klammern werden ihr Feld `Kriterien`.

- PM1.1 Sammel-Funktion und Diagnose-Zeile im Client-Log (AC-01).
- PM1.2 Performance-Modus mit Bericht und Einbrüchen (AC-03, AC-04, AC-05).
- PM1.3 Review (Code-Sprint): alle Kriterien prüfen.
- PM1.4 Workshop (🧑): `sim_test` mit Client und Messlauf am PC (AC-02, AC-06).

## Abnahme

–
