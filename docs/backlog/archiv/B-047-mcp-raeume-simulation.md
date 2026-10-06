# B-047 · MCP-Tools zeigen laufende Räume und rechnen Level und Simulationen

- **Domäne:** SRV
- **Typ:** Idee
- **Prio:** mittel
- **Umgebung:** live
- **Status:** erledigt
- **Sprint:** M6
- **Erstellt:** 2026-09-30
- **Spec:** freigegeben
- **Revision:** 2
- **Freigabe:** 2026-10-01 🧑 Chat („M6 freigeben“), Revision 2

## Ausgangslage

Ab SP07 rechnet der Go-Server mehrere Räume; `/api/status` (Token, B-027) zeigt Räume und Tick-Dauer. `engine/level` und `engine/sim` gibt es ab SP04 bzw. SP06. Das Entwickler-Werkzeug `tools/k3c-dev` (B-046) kennt davon nichts.

## Ziel

MCP-Tools zeigen laufende Räume und rechnen Level und Simulationen. Nutzen: Fehler im Online-Spiel und Balancing-Fragen lassen sich ohne Browser und ohne Rohdaten nachvollziehen.

## Beteiligte und Zielgruppen

Entwickler und Coding-Agenten; Balancing-Workshops (REG) nutzen `sim_run` für Vergleiche.

## Anforderungen

- `server_status` und `rooms_list`: über `/api/status` des laufenden Servers, verdichtet (Räume, Geräte, Spieler, Tick-Dauer).
- `room_snapshot {room}`: kompakter Zustand eines Raums (Tag/Nacht, Gold, Einheiten, Spieler) über die Status-Schnittstelle.
- `level_generate {seed, biome}`: rechnet in-process mit `engine/level`, eine Textzeile je Abschnitt.
- `sim_run {seed, ticks, inputs?}`: deterministischer Lauf mit `engine/sim`, Zusammenfassung (Tag, Gold, Verluste, Ende); `ticks` höchstens 100 000 (30 Ticks/s, gut drei Tag/Nacht-Zyklen).
- Adresse und Token kommen aus den Variablen des Servers: Token `K3C_STATUS_TOKEN`, Adresse `K3C_SERVER_URL`, sonst `http://127.0.0.1:<K3C_HTTP_PORT oder 8080>`.

## Nicht-Ziele

Eingriffe in laufende Räume (Gerät trennen, Eingaben senden) bleiben bei der TUI (B-002).

## Regeln und Einschränkungen

Go-Server ist die einzige Engine (Entscheidung 001), Standardbibliothek zuerst; Schichtgrenzen und Komplexitäts-Budget aus `docs/arbeitsweise.md`. Nur lesend gegenüber dem Server; `tools/k3c-dev` spricht wie die TUI nur mit `/api/status`; `engine/*` bindet es als eigenes Modul per `replace k3c => ../..` ein. In-process-Aufrufe sind deterministisch. Schichtgrenze: `engine/*` importiert nichts aus `cmd/` oder `tools/`.

## Beispiele

- `sim_run {"seed":"abc","ticks":3000}` zweimal → identische Zusammenfassung.
- `rooms_list` bei 2 laufenden Räumen → 2 Zeilen mit Raum, Spielern und Tick-Dauer p99.

## Ausnahme- und Fehlerfälle

- Server nicht erreichbar → Meldung mit Adresse; die In-process-Tools laufen weiter.
- Token fehlt oder ist falsch → Meldung „401, Token prüfen“.
- `ticks` über 100 000 → Ablehnung mit der Grenze.

## Akzeptanzkriterien

- **AC-01** `server_status`, `rooms_list` und `room_snapshot` liefern gegen einen Test-Server (`httptest`) verdichteten Text (Tests).
- **AC-02** Server weg oder 401 → verständliche Meldung, kein Absturz (Tests).
- **AC-03** `level_generate` liefert für die Golden-Seeds dasselbe Level wie `testdata/golden/` (Test).
- **AC-04** `sim_run` ist deterministisch (zwei Läufe identisch) und lehnt `ticks` über 100 000 ab (Tests).

## Offene Fragen

keine (entschieden 2026-10-01 🧑 Chat: Namen des Servers übernehmen, `ticks` höchstens 100 000; Revision 2).

## Notizen

Setzt B-046 (M1), SP04, SP06 und SP07.2 voraus.
