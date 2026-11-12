# NG14 · CLI · Natives Spiel: Fehler- und Ladeanzeige, Shader-Umschalter

- **Status:** geplant
- **Projekt:** NAT
- **Domäne:** CLI
- **Reife:** bereit
- **Tickets:** B-810, B-816
- **Start-Commit:** – (wird beim Aktivieren gesetzt: `git rev-parse --short origin/develop`)
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** autonom, Claude, 2026-11-12

## Ausgangslage

Das native Spiel beendet sich bei einem Startfehler oder fehlender Grafik still und friert beim Stufenwechsel ohne Anzeige ein (synchrones Laden in `native/k3c-game/game/levels.go:47-61`). Der Fluss-Shader (NG11) lässt sich nur über `K3C_RIVER` umschalten, Bildrate und Rechenzeit sind nur im Messmodus sichtbar. Einzelheiten: B-810, B-816.

## Ziel

Wer das Spiel startet, sieht Fehler und Ladevorgänge im Fenster; Fluss-Darstellung und Leistungsanzeige lassen sich im Spiel umschalten. Am Ende sichtbar: Tests in `game`, `world`, `view`, `local` grün, Notiz zu stderr der EXE in B-810, Abnahme 🧑 am PC (NG14.5).

## Beteiligte und Zielgruppen

Spieler am PC und Mini-PC ohne Konsole; Claude (Umsetzung, Review); 🧑 Ralf (Abnahme am PC: Grafikordner umbenennen, Meldung sehen).

## Anforderungen

`B-810 › Anforderungen`, `B-816 › Anforderungen` (Revision 2: Tasten F3 und F2, Werte in den Einstellungen).

## Nicht-Ziele

Fehler des Ton-Geräts (B-805, NG15), asynchrones Laden der Grafik, neue Shader-Effekte, Messläufe (gesperrt), Optionen-Menü (NG13).

## Regeln und Einschränkungen

Entscheidung 008; Verträge `view` und `world` nur additiv, `view.Settings` nur additiv; Tasten belegen weder `View + Menu` noch `B`; keine Wanduhr und keine Allokation je Bild bei ausgeschalteter Anzeige; Logging mit Emojis. Funktion ≤ 60 Zeilen.

Planungsangaben für die Arbeitswellen (`docs/arbeitsweise.md` › Parallelität und Wellen), je eine Zeile, nur IDs und Pfade:

- **Bereich:** `native/k3c-game/game/**`, `native/k3c-game/main.go`, `native/k3c-game/world/**`, `native/k3c-game/view/**`, `native/k3c-game/local/settings.go`, `native/k3c-game/local/settings_test.go`
- **Abhängig von:** –
- **Braucht 🧑:** Abnahme
- **Sperre:** –

## Beispiele

`B-810 › Beispiele` (Grafik fehlt → „Grafik fehlt: <Pfad>“, Stufenwechsel → Ladeanzeige), `B-816 › Beispiele` (F3 → Fluss wechselt ohne Neustart).

## Ausnahme- und Fehlerfälle

`B-810 › Ausnahme- und Fehlerfälle` (Fenster öffnet nicht → stderr, Exit ≠ 0), `B-816 › Ausnahme- und Fehlerfälle` (Shader-Fehler → Kacheln). Abnahme am PC nicht möglich → NG14.5 bleibt offen, der Sprint bleibt aktiv (Hardware entkoppelt).

## Akzeptanzkriterien

- **AC-01** Fehlende Grafik liefert Fehlerzustand mit Text statt stillem Exit (`B-810/AC-01`).
- **AC-02** Stufenwechsel setzt vor dem Laden „lädt“ und zeigt die Anzeige (`B-810/AC-02`).
- **AC-03** stderr der gebauten EXE geprüft, Ergebnis in B-810 › Notizen (`B-810/AC-03`).
- **AC-04** Abnahme 🧑 am PC: Grafikordner umbenannt, Meldung gesehen (`B-810/AC-04`).
- **AC-05** Umschalten ändert den Zustand und zeichnet im anderen Modus (`B-816/AC-01`).
- **AC-06** Anzeige aus → keine Zeitnahme im Bildpfad (`B-816/AC-02`).
- **AC-07** Anzeige an → Bildrate und Rechenzeit im Frame-Text (`B-816/AC-03`).

## Offene Fragen

keine (entschieden autonom, Claude, 2026-11-12: Tasten F3/F2, Werte in den Einstellungen, siehe `B-816 › Offene Fragen`).

## Sessions

| Nr. | Datei | Typ | Agent | Status |
|---|---|---|---|---|
| NG14.1 | `NG14.1-fehleranzeige.md` | Umsetzung | autonom | offen |
| NG14.2 | `NG14.2-ladeanzeige.md` | Umsetzung | autonom | offen |
| NG14.3 | `NG14.3-shader-umschalter.md` | Umsetzung | autonom | offen |
| NG14.4 | `NG14.4-review.md` | Review | autonom | offen |
| NG14.5 | `NG14.5-abnahme-pc.md` | Review | Mensch | offen |

Bei `Reife: Entwurf` genügen Stichpunkte mit den Kriterien in Klammern statt Session-Dateien. Vor dem Aktivieren
wird jede Session als eigene Datei nach `docs/vorlagen/session.md` geschrieben. Keine eigenen Sim- oder
Balance-Sessions vor der Abnahme: Sim, Balance, Gen und `check:race` laufen im Abnahme-Ticket (B-742).

## Abnahme

Wird von der Review-Session (Doku-Sprint: letzte Session) ausgefüllt, höchstens fünf Zeilen: Datum, Kriterien
(Verweis auf Session-Ergebnisse), behobene Befunde, neue Tickets und das Abnahme-Ticket für Sim, Balance und Gen
(genau eins, `docs/arbeitsweise.md` › Sim, Balance und Gen). Bis dahin `–`.
