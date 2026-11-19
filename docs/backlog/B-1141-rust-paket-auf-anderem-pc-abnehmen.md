# B-1141 · Abnahme: Das Rust-Paket dist/k3c-game startet auf einem anderen PC und die Lizenzdatei genügt

- **Domäne:** INF
- **Typ:** Schuld
- **Prio:** mittel
- **Umgebung:** live
- **Status:** offen
- **Sprint:** RU64
- **Projekt:** RSU
- **Erstellt:** 2026-11-19
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** autonom, Claude, 2026-11-19
- **Duplikat von:** –
- **Ersetzt:** –

## Ausgangslage

RU57 liefert `dist/k3c-game/` mit EXE (statische C-Laufzeit), data, Grafik, Ton, Schrift, Rig-Teilen, LICENSE, LICENSE-ASSETS, `LIZENZEN.txt` (132 Crates) und `README.txt`. Sichtprüfung nur auf dem Entwicklungs-PC.

## Ziel

🧑 Ralf bestätigt am anderen PC, dass das Paket startet, und entscheidet über die Lizenzdatei.

## Beteiligte und Zielgruppen

🧑 Ralf.

## Anforderungen

- `dist\k3c-game` auf einen PC ohne Entwicklungswerkzeuge kopieren, `k3c-game.exe` per Doppelklick starten (Grafik, Ton, Menü, Neues Spiel), danach aus einer Konsole in einem fremden Ordner.
- `LIZENZEN.txt` und `README.txt` lesen.
- Entscheiden: genügt der Hinweis statt Volltext bei den 14 Crates ohne Lizenzdatei (u. a. `symphonia-*` MPL-2.0); soll der Go-Kopf der Liste (B-1139, RU37) vor der ersten Weitergabe weichen; soll das Konsolenfenster verschwinden (`windows_subsystem`).

## Nicht-Ziele

Neue Funktionen.

## Regeln und Einschränkungen

Manuelle Abnahme nur durch 🧑.

- **Bereich:** `docs/messung/rust-abnahme.md`
- **Abhängig von:** RU57
- **Braucht 🧑:** Abnahme
- **Größe:** S
- **Sperre:** –

## Beispiele

Paket auf zweitem PC: Hauptmenü erscheint, Ton läuft, `saves\` entsteht neben der EXE.

## Ausnahme- und Fehlerfälle

Startet nicht: Meldung oder Windows-Fehlercode notieren, Ticket Problem.

## Akzeptanzkriterien

- **AC-01** Das Paket startet am zweiten PC aus Doppelklick und aus fremdem Ordner.
- **AC-02** 🧑 hat über Lizenzhinweis, Go-Kopf und Konsolenfenster entschieden.

## Offene Fragen

siehe Anforderungen.

## Notizen

Station 70–72 in `docs/messung/rust-abnahme.md`.
**Teil Second Worker (Sprint RU64, Session RU64.7):** Der zweite Rechner kann das Paket in einen fremden Ordner kopieren und aus einer Konsole starten (Dateiliste, Start, `saves\\` neben der EXE, Beendigung ohne Fehler, Lizenzdateien vorhanden), nicht aber Doppelklick, Ton am Gerät oder die Entscheidungen. Ergebnis `docs/messung/abnahme-b-1141.md`. Doppelklick, Ton, Mini-PC und AC-02 bleiben bei Ralf; das Ticket bleibt offen.
