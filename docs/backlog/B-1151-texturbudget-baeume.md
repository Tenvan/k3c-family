# B-1151 · Texturbudget der Bäume um 15 MiB senken

- **Domäne:** PAP
- **Typ:** Schuld
- **Prio:** niedrig
- **Umgebung:** offline
- **Status:** offen
- **Sprint:** LA1
- **Projekt:** RSU
- **Erstellt:** 2026-11-19
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –
- **Duplikat von:** –
- **Ersetzt:** –

## Ausgangslage

12 Bäume belegen 21,3 MiB statt vorher 4 Bäume mit 6,1 MiB (+15,3 MiB, rund 3 % des Richtwerts, BB1.3). Das Wald-Level lädt derzeit nur tanne, pappel, birke (4,2 MiB). Die Tanne hat Ping-Pong-Frames (5 von 8 Hashes), Sparpotenzial.

## Ziel

Das Texturbudget der Bäume wächst höchstens um den Wert, den die Anbindung (B-1146) wirklich lädt; doppelte Frames entfallen.

## Beteiligte und Zielgruppen

Spieler am TV (Frame-Budget, Mini-PC); Umsetzung Claude (PAP).

## Anforderungen

- Messen vor Ändern: geladene Baum-Texturen im Wald-Level.
- Doppelte Frames der Tanne und Verwandter als Ping-Pong im Manifest statt als Bild.

## Nicht-Ziele

Scharfe Zeichnung opfern; Texeldichte nur anheben, wenn Unschärfe sichtbar (dann `hoehe_px` per `assets_set`).

## Regeln und Einschränkungen

Messen statt raten; Gen im Abnahme-Ticket. Planungsangaben:
- **Bereich:** `public/grafik/objekte/**`, `data/paper.json`
- **Abhängig von:** B-1146
- **Braucht 🧑:** –
- **Größe:** S
- **Sperre:** golden

## Beispiele

Tanne: 8 Frames, 5 verschiedene; nach dem Ping-Pong-Umbau 5 Bilder im Sheet.

## Ausnahme- und Fehlerfälle

Messung zeigt unkritisch (unter 1 % Richtwert): Ticket mit Messwert verwerfen.

## Akzeptanzkriterien

- **AC-01** Messbericht: Baum-Texturen im Wald-Level vorher und nachher in MiB.
- **AC-02** Golden der `objekte` nach gewollter Änderung aktualisiert und begründet.

## Offene Fragen

keine

## Notizen

Von BB1.5 angelegt (Befund BB1.3).
