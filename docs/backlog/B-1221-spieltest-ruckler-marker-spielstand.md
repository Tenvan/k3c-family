# B-1221 · Spieltest-Modus setzt bei Rucklern selbst Marker und legt je Marker einen Spielstand ab

- **Domäne:** CLI
- **Typ:** Idee
- **Prio:** mittel
- **Umgebung:** offline
- **Status:** offen
- **Sprint:** WZ66
- **Projekt:** WZG
- **Erstellt:** 2026-11-22
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** Ralf, Chat 2026-11-22
- **Duplikat von:** –
- **Ersetzt:** –

## Ausgangslage

Mit B-1220 setzen Spieler Marker per Pad. Ruckler fühlt man am Sofa, sagt aber selten genau, wann; Befunde lassen sich im Nachgang schwer nachstellen, weil der Spielzustand des Moments fehlt.

## Ziel

Ruckler sind belegt statt gefühlt (Messen statt raten), und jeder Pad-Marker kann im Spiel nachgestellt werden.

## Beteiligte und Zielgruppen

Ralf (🧑) und Claude bei Auswertung und Nachstellen von Befunden.

## Anforderungen

- Ruckler-Automarker: Reißt ein Frame die Schwelle (Standard 25 ms, einstellbar per `--playtest-jank-ms`), schreibt das Spiel einen Marker `kind: "ruckler"` mit Frame-Zeit und Kontext wie B-1220; höchstens einer je 5 s.
- Spielstand am Marker: Bei jedem Pad-Marker legt das Spiel über die vorhandene Speicher-Funktion einen Spielstand nach `<ordner>/saves/marker-<nr>` ab; die Marker-Zeile nennt den Dateinamen (`save`).
- Speichern darf den Frame nicht blockieren (Schreiben außerhalb des Frame-Takts); bei Ruckler-Markern kein Spielstand.
- Funktioniert mit 2 Spielern; Datei ≤ 400 Zeilen, Funktion ≤ 60 Zeilen.

## Nicht-Ziele

- Änderung am Spielstand-Format.
- Laden eines Marker-Spielstands aus der Workbench (später eigenes Ticket, falls gebraucht).

## Regeln und Einschränkungen

- Frame-Budget 60 Hz: das Messen selbst kostet keine messbare Zeit.
- Spielstand nur mit vorhandener Funktion schreiben; keine Rücksicht auf alte Spielstände nötig.

- **Bereich:** `rust/crates/k3c-game/src/playtest/**`
- **Abhängig von:** B-1220
- **Braucht 🧑:** Freigabe
- **Größe:** S
- **Sperre:** –

## Beispiele

- Beim Welleneinbruch dauert ein Frame 41 ms → Marker `ruckler` mit `frame_ms: 41`, kein Spielstand.
- Spieler 1 setzt Back+Y → Marker mit `save: "saves/marker-3"`, Datei liegt im Session-Ordner.

## Ausnahme- und Fehlerfälle

- Dauerruckeln → höchstens ein Ruckler-Marker je 5 s, der nächste nennt die Zahl übersprungener Frames.
- Speichern schlägt fehl → Marker ohne `save`, Log ❌, Spiel läuft weiter.
- Ladebildschirm oder Stufenwechsel → keine Ruckler-Marker (Schwelle nur im laufenden Spiel).

## Akzeptanzkriterien

- **AC-01** Unit-Test mit künstlichen Frame-Zeiten: Schwelle, Drossel 5 s, keine Marker beim Laden.
- **AC-02** Unit-Test: Pad-Marker schreibt Spielstand und trägt `save` ein, Ruckler-Marker nicht.
- **AC-03** `--measure-frames` mit und ohne `--playtest` zeigt keinen messbaren Unterschied beim schlechtesten Frame (Wert im Ergebnis der Session).
- **AC-04** `check_run task:check:diff` grün.

## Offene Fragen

keine

## Notizen

–
