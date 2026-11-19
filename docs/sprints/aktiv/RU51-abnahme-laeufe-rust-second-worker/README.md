# RU51 · SIM, SRV, CLI · Abnahme- und Messläufe der Rust-Umstellung auf dem Second Worker

- **Status:** aktiv
- **Projekt:** RSU
- **Domäne:** SIM, SRV, CLI
- **Reife:** bereit
- **Tickets:** B-1093, B-1098, B-1101, B-1104, B-1105, B-1096, B-1112, B-1119, B-1108
- **Start-Commit:** – (wird beim Aktivieren gesetzt: `git rev-parse --short origin/develop`)
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** autonom, Claude, 2026-11-19

## Ausgangslage

Mehrere Rust-Sprints (RU14, RU18, RU19, RU21, RU22, RU26, RU29, RU30, RU36) sind ohne Sim-, Balance-, Gen-, Race- und Messläufe abgeschlossen worden, weil der Hauptrechner gesperrt ist. Die Abnahme-Tickets tragen Lane zweit und liegen offen. Der Second Worker (starke CPU/GPU/RAM) ist idle.

## Ziel

Alle offenen Abnahme- und Messlauf-Tickets der Rust-Umstellung sind auf dem Second Worker gefahren, belegt und für Ralf zusammengefasst; Rotes steht im Ergebnis, nichts wird nebenbei behoben.

## Beteiligte und Zielgruppen

Second Worker fährt die Sessions; 🧑 liest die Ergebnisse und führt manuelle Abnahmen am Pad selbst.

## Anforderungen

Läufe nur über `sim_test`/`check_run`, blockierend gewartet. Messen statt raten (Wiederholungen, Median, Streuung, schlechtester Frame). Keine Code-Änderungen; Gate-Messung B-1074 ist nicht Teil des Sprints.

## Nicht-Ziele

Fixes roter Läufe (kommen als Ticket vom Hauptrechner), Optimierung (RU43), Gate-Messung B-1074, Balancing-Änderungen, manuelle Abnahmen am Pad.

## Regeln und Einschränkungen

Sim-Sperre gilt nur für den Hauptrechner (🧑 2026-11-18). Keine Spielregel-, Protokoll- oder Spielstand-Änderung. RU51.8 startet erst nach RU32 (gleiche Testdateien, Abhängigkeit in der Session).

Planungsangaben für die Arbeitswellen (`docs/arbeitsweise.md` › Parallelität und Wellen), je eine Zeile:

- **Bereich:** `docs/messung/abnahme-ru14.md`, `docs/messung/abnahme-ru18.md`, `docs/messung/abnahme-ru19.md`, `docs/messung/abnahme-ru21.md`, `docs/messung/abnahme-ru26.md`, `docs/messung/abnahme-ru29-ru30.md`, `docs/messung/rust-sim-abgleich.md`
- **Abhängig von:** –
- **Braucht 🧑:** –
- **Sperre:** –

## Beispiele

RU51.4 fährt die fünf Läufe aus B-1104 und trägt je Lauf Befehl, Ergebnis und Dauer in `docs/messung/abnahme-ru19.md` ein.

## Ausnahme- und Fehlerfälle

Ein Lauf ist rot: Ausgabe und Vermutung („ungeprüft“) ins Ergebnis unter „Für den Hauptrechner“, Session trotzdem `fertig`, Ticket bleibt offen. Ein Lauf braucht ein Gerät oder Fenster: in die Prüfliste für Ralf.

## Akzeptanzkriterien

- **AC-01** Die Abnahme-Tickets B-1093, B-1098, B-1101, B-1104, B-1105, B-1096, B-1112, B-1119 haben je ein belegtes Ergebnis (Lauf, Zahl, Dauer) oder einen begründeten Verschiebegrund.
- **AC-02** Der Bildvergleich mit gerenderter Grafik ist grün, ignorierte Marken sind aktuell und der Rundlauf Speichern/Laden läuft (B-1108).

## Offene Fragen

keine

## Sessions

| Nr. | Datei | Typ | Agent | Status |
|---|---|---|---|---|
| RU51.1 | `RU51.1-abnahme-ru14-welt-schritt.md` | Umsetzung | autonom | fertig |
| RU51.2 | `RU51.2-abnahme-ru21-store-room.md` | Umsetzung | autonom | fertig |
| RU51.3 | `RU51.3-abnahme-ru18-sim-wirtschaft.md` | Umsetzung | autonom | fertig |
| RU51.4 | `RU51.4-abnahme-ru19-events-geschichte.md` | Umsetzung | autonom | offen |
| RU51.5 | `RU51.5-messung-ru22-durchsatz.md` | Umsetzung | autonom | offen |
| RU51.6 | `RU51.6-abnahme-ru26-spielschleife.md` | Umsetzung | autonom | offen |
| RU51.7 | `RU51.7-abnahme-ru29-ru30-messen.md` | Umsetzung | autonom | offen |
| RU51.8 | `RU51.8-abnahme-bildvergleich-ignore-marken.md` | Umsetzung | autonom | offen |

Bei `Reife: Entwurf` genügen Stichpunkte mit den Kriterien in Klammern statt Session-Dateien. Vor dem Aktivieren
wird jede Session als eigene Datei nach `docs/vorlagen/session.md` geschrieben. Keine eigenen Sim- oder
Balance-Sessions vor der Abnahme: Sim, Balance, Gen und `check:race` laufen im Abnahme-Ticket (B-742).

## Abnahme

Wird von der Review-Session (Doku-Sprint: letzte Session) ausgefüllt, höchstens fünf Zeilen: Datum, Kriterien
(Verweis auf Session-Ergebnisse), behobene Befunde, neue Tickets und das Abnahme-Ticket für Sim, Balance und Gen
(genau eins, `docs/arbeitsweise.md` › Sim, Balance und Gen). Bis dahin `–`.
