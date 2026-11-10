# LG1 · SRV, DEV, CLI · Remote-Logs vom Pi und Xbox mit WebGL 1 und 130 MiB

- **Status:** aktiv
- **Projekt:** LST
- **Domäne:** SRV, DEV, CLI
- **Reife:** bereit
- **Tickets:** B-770, B-771, B-772, B-773
- **Start-Commit:** cbb93dc0
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** autonom, Claude (Nutzer: „LST ganz abarbeiten“, 2026-11-10)

## Ausgangslage

Der Pi (`pi-gaming:8080`, v0.23.2) beantwortet `/api/dev/status` und `/api/dev/reports`, aber `/api/dev/log` meldet „Log aus“ (kein Log-Ordner), und es liest nur das Server-Log, nicht das Client-Log. Die Log-Tools des MCP-Servers kennen nur lokale Logs. Eine erste WebGL-Analyse (aus der Erinnerung von 🧑) legt nahe: Die Xbox läuft mit WebGL 1, der Texturspeicher soll auf 130 MiB begrenzt werden. Die Textur-Testseite passt nicht auf einen Screen. Details: B-770 bis B-773.

## Ziel

Alle Logs eines entfernten Servers (Pi) sind per Endpunkt und MCP-Tool lesbar, und die Xbox läuft mit WebGL 1 unter 130 MiB Texturspeicher. Am Ende sichtbar: `logs_errors {server: "pi"}` zeigt Server- und Client-Zeilen vom Pi, das Client-Log eines Xbox-Laufs nennt WebGL 1 und den Verbrauch unter 130 MiB, die Textur-Testseite passt ohne Scrollen auf den TV.

## Beteiligte und Zielgruppen

🧑 betreibt den Pi (Log-Ordner setzen) und nimmt auf der Xbox ab (LG1.6); Claude setzt um und liest danach die Logs selbst.

## Anforderungen

B-770 › Anforderungen (alle Logs im Endpunkt); B-771 › Anforderungen (MCP-Log-Tools mit `server`); B-772 › Anforderungen (WebGL 1, 130 MiB Texturen); B-773 › Anforderungen (Seite auf einen Screen). Reihenfolge fest: Endpunkt vor Tools vor Auswertung; das Budget (B-772) wird nach dem ersten Log-Lauf am Pi gegen die echten Werte geprüft.

## Nicht-Ziele

Schreibende Zugriffe auf den Pi, Dienststeuerung aus der Ferne, WebGL 2 auf der Xbox, neue Grafik, Streaming der Logs.

## Regeln und Einschränkungen

- Domänen: LG1.1 SRV (`engine/net`), LG1.2 DEV (`tools/k3c-workbench`), LG1.3 und LG1.4 CLI (`src/`).
- Nur lesende GET-Endpunkte im Dev-Mode; feste Dateinamen, keine Pfade aus der URL.
- Budget-Werte in `data/`, nicht im Code. Kein Spiel-Zustand im Client.
- Keine Sim-, Balance- und Gen-Läufe in den Sessions (B-742), sie laufen im Abnahme-Ticket.
- Review vor dem Commit bei LG1.1 und LG1.2 (SRV, Pfad- und Eingabe-Code, Netzwerkzugriff).

## Beispiele

`logs_errors {server: "pi"}` → Fehlerzeilen vom Pi samt Quelle. Xbox-Start → Client-Log „🚀 WebGL 1, Texturen 96 von 130 MiB“. Textur-Testseite bei 1920×1080 → kein Scrollbalken.

## Ausnahme- und Fehlerfälle

Pi nicht erreichbar oder „Log aus“ → klare Meldung mit Adresse und Hinweis auf `K3C_LOG_DIR`. Logwerte weichen von der Erinnerung ab (kein WebGL 1, anderes Budget) → B-772 wird angepasst, Sprint nicht blind abgenommen.

## Akzeptanzkriterien

- **AC-01** Die Endpunkte listen und lesen alle Log-Dateien, filtern, nutzen Cursor und lehnen Pfade ab (`B-770/AC-01` bis `AC-03`, Go-Test).
- **AC-02** `logs_*` und `server_status` lesen mit `server` einen entfernten Server, Fehler werden klar gemeldet (`B-771/AC-01` bis `AC-03`, Test).
- **AC-03** Wahl von WebGL 1 und Budgetprüfung (`B-772/AC-01`, `AC-02`, Test).
- **AC-04** Textur-Testseite ohne Seitenscroll bei drei Auflösungen (`B-773/AC-01`, `AC-02`, Test).
- **AC-05** Abnahme (🧑): Log-Ordner am Pi gesetzt, `pi-gaming:8080/api/dev/log?file=client` liefert Zeilen (`B-770/AC-04`).
- **AC-06** Abnahme (🧑): `logs_errors {server: "pi"}` zeigt echte Pi-Zeilen (`B-771/AC-04`).
- **AC-07** Abnahme (🧑): Xbox-Lauf ohne Kontextverlust, Log zeigt WebGL 1 und Texturen unter 130 MiB (`B-772/AC-03`).
- **AC-08** Abnahme (🧑): Textur-Testseite passt auf der Xbox auf einen Screen (`B-773/AC-03`).

## Offene Fragen

- Wie setzt 🧑 den Log-Ordner am Pi (Compose-Datei, `K3C_LOG_DIR`)? Default: 🧑 setzt ihn vor LG1.6, die Doku nennt die Variable.
- Weichen die Logwerte von der Erinnerung (WebGL 1, 130 MiB) ab, passt Claude B-772 an und meldet es.

## Sessions

| Nr. | Datei | Typ | Agent | Status |
|---|---|---|---|---|
| LG1.1 | `LG1.1-dev-endpunkt-alle-logs.md` | Umsetzung | autonom | fertig |
| LG1.2 | `LG1.2-mcp-log-tools-remote.md` | Umsetzung | autonom | offen |
| LG1.3 | `LG1.3-xbox-webgl1-textur-budget.md` | Umsetzung | autonom | offen |
| LG1.4 | `LG1.4-texturtest-responsive.md` | Umsetzung | autonom | fertig |
| LG1.5 | `LG1.5-review.md` | Review | autonom | offen |
| LG1.6 | `LG1.6-abnahme-pi-xbox.md` | Workshop | Mensch | offen |

Bei `Reife: Entwurf` genügen Stichpunkte mit den Kriterien in Klammern statt Session-Dateien. Vor dem Aktivieren
wird jede Session als eigene Datei nach `docs/vorlagen/session.md` geschrieben. Keine eigenen Sim- oder
Balance-Sessions vor der Abnahme: Sim, Balance, Gen und `check:race` laufen im Abnahme-Ticket (B-742).

## Abnahme

Wird von der Review-Session (Doku-Sprint: letzte Session) ausgefüllt, höchstens fünf Zeilen: Datum, Kriterien
(Verweis auf Session-Ergebnisse), behobene Befunde, neue Tickets und das Abnahme-Ticket für Sim, Balance und Gen
(genau eins, `docs/arbeitsweise.md` › Sim, Balance und Gen). Bis dahin `–`.
