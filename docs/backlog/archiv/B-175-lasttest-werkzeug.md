# B-175 · Ein Lasttest-Werkzeug misst Tick-Dauer und CPU gegen das Pi-Ziel

- **Domäne:** SRV
- **Typ:** Idee
- **Prio:** mittel
- **Umgebung:** live
- **Status:** erledigt
- **Sprint:** LT1
- **Projekt:** –
- **Erstellt:** 2026-10-03
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** 2026-10-04, Chat (Ralf), Revision 1, durch 🧑 (mit LT1)

## Ausgangslage

Die Lastmessung auf dem Pi (SP11.3, Ziel aus B-042: 2 Räume × 3 Spieler, Tick-p99 < 10 ms bei 30 Hz) ist heute Handarbeit: Räume per Testseite anlegen, `GET /api/status` von Hand abfragen (`engine/net/status.go`), `top` auf dem Pi lesen. Erste Handmessung am 2026-10-03: tagsüber p99 8,0 bis 9,5 ms, nachts 10,2 bis 10,3 ms (knapp über dem Ziel, ein Messpunkt). 🧑 hat die Messung zurückgestellt, bis ein Werkzeug sie wiederholbar macht. `/api/status` kennt keine CPU-Last.

## Ziel

Ein Werkzeug startet Räume mit scriptbaren Bots gegen einen laufenden Server (z. B. dem Pi), misst über eine Zeit oder eine Nacht Tick-Dauer und CPU, schreibt einen Bericht und bewertet ihn gegen das Ziel. Nutzen: Lastmessungen sind wiederholbar, vergleichbar und vor und nach Änderungen (F3 Feedback-Events, W-Sprints) mit einem Befehl möglich.

## Beteiligte und Zielgruppen

🧑 und Agenten (Messläufe am Pi und lokal), Domäne SRV (`cmd/`, `engine/net/status.go`).

## Anforderungen

- Befehl `task load -- -url <Server> -token <T> -rooms 2 -players 3 -duration 15m` (Werkzeug `cmd/k3c-load`, Go, nur Standardbibliothek plus die vorhandene WebSocket-Bibliothek).
- Die Bots sprechen **das aktuelle Protokoll über `/ws`** (hello, create, input) wie ein Client (Version aus `ProtocolVersion`, heute v3), nie über interne Abkürzungen. Ihre Eingaben sind deterministisch aus einem Seed (`engine/rng`), mindestens: Laufen mit Richtungswechsel, Sprint, Münzen geben; Räume mit Präfix `test-` (werden vom Server aufgeräumt).
- Das Werkzeug fragt `/api/status` alle 5 s ab und führt je Raum eine Reihe aus Zeit, Phase (Tag, Dämmerung, Nacht), Tick, Tick-Dauer last und p99.
- `/api/status` liefert zusätzlich die CPU-Auslastung des Server-Prozesses (Feld `cpu`, Prozent einer CPU, gemittelt über das letzte Intervall); die Quelle ist plattformabhängig (Linux `/proc/self/stat`, sonst Prozesszeit), fehlt sie, bleibt das Feld weg.
- Bericht als JSON (Zeitreihe) und als Markdown-Tabelle: je Raum und Phase p99 min, Mittel, max, CPU Mittel und Spitze, Bewertung gegen das Ziel (Parameter `-target-p99 10`): `erreicht` (max < Ziel), `knapp` (Mittel < Ziel ≤ max), `verfehlt` (Mittel ≥ Ziel); Exit-Code 0, 1 bei verfehlt.
- Läuft mit `-duration night`: bis zum Ende der ersten Nacht plus einer Dämmerung, mit Obergrenze.

## Nicht-Ziele

Lasttest mit echten Browsern oder Geräten, Netzwerkanalyse, Bandbreitenmessung (B-140), automatisches Absenken des Ziels.

## Regeln und Einschränkungen

Schichtgrenzen aus `docs/arbeitsweise.md`: `cmd/k3c-load` importiert nur `engine/net`-Protokolltypen und `engine/rng`, nichts aus `engine/room`/`sim`. Datei ≤ 400 Zeilen, Funktion ≤ 60. Keine neue Abhängigkeit ohne Ticket. Das Werkzeug legt nur Räume mit Präfix `test-` an und löscht nichts außerhalb davon. Ein Token wird nie geloggt.

## Beispiele

Gegen den Pi: 2 Räume × 3 Spieler, 15 Minuten → Tabelle mit p99 je Raum und Phase und der Aussage „knapp: Nacht 10,3 ms, CPU Spitze 62 %“.

## Ausnahme- und Fehlerfälle

Server nicht erreichbar oder Token falsch → Fehlermeldung und Exit-Code 2, keine Räume. Abbruch mit Strg+C → Bots trennen sich, der Bericht entsteht aus den bisherigen Werten. `/api/status` ohne `cpu` (Windows) → Spalte „CPU“ bleibt leer.

## Akzeptanzkriterien

- **AC-01** Test: Das Werkzeug startet gegen einen In-Prozess-Server (`httptest`) 2 Räume × 2 Bots für 5 s und erzeugt einen Bericht mit Tick-Reihe je Raum.
- **AC-02** Test: Zwei Läufe mit gleichem Seed senden dieselbe Eingabefolge (aufgezeichnet an der Verbindung).
- **AC-03** Test: Die Bewertung liefert `erreicht`, `knapp` und `verfehlt` für Beispielreihen, der Exit-Code folgt der Bewertung.
- **AC-04** Test: `GET /api/status` enthält `cpu` unter Linux (Probe über eine austauschbare Quelle), ohne Quelle fehlt das Feld.
- **AC-05** Nach dem Lauf sind keine Test-Räume mehr offen (Test) und kein Token steht in der Ausgabe.
- **AC-06** 🧑 hat einen Messlauf am Pi (2 Räume × 3 Spieler über eine Nacht) gestartet; die Tabelle und die Bewertung stehen im Session-Ergebnis und in B-042.

## Offene Fragen

keine (Messdauer „Nacht“ und Obergrenze legt der Sprint fest; CPU als Prozentwert einer CPU: Vorschlag).

## Notizen

Anlass: Handmessung SP11.3 vom 2026-10-03. Verwandt: B-099 (Balancing-Tester, Bots in der Simulation), B-140 (Bandbreite), B-042.
