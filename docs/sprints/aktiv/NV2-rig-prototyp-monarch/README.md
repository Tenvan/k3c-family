# NV2 · CLI · Rig-Prototyp: Monarch aus Teilen in Ebitengine

- **Status:** aktiv
- **Projekt:** NAT
- **Domäne:** CLI
- **Reife:** bereit
- **Tickets:** B-778
- **Start-Commit:** cfd95463
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** 2026-11-11 🧑 Chat („Danach NV2 freigegeben“)

## Ausgangslage

Entscheidung 008: natives Spiel in Go + Ebitengine. Der Monarch belegt 195 Sheets und 363 von 580 MiB RGBA aller Grafik (B-778 › Ausgangslage). Zielbild sind Rig-Modelle für alle Figuren mit optionalen Waffen und Rüstungen; ob das bei gleicher Optik geht, ist nicht belegt.

## Ziel

Ein Prototyp unter `native/` zeichnet Monarch und Pferd aus Teil-Texturen und dem Rig aus `tools/paper`, kombiniert beide am Sattel und zeigt Waffe und Rüstung als optionale Elemente. Am Ende sichtbar: Fenster mit Rig und Sheet nebeneinander, Differenzbild und ein JSON-Bericht unter `reports/`.

## Beteiligte und Zielgruppen

🧑 Ralf (Sichtprüfung, Mini-PC, Entscheidung), Claude (Bau, Referenzmessung).

## Anforderungen

B-778 › Anforderungen.

## Nicht-Ziele

B-778 › Nicht-Ziele; keine Gegner, Truppen, Bosse (B-780), keine Simulation, kein HUD; `gen:paper` bleibt unverändert.

## Regeln und Einschränkungen

B-778 › Regeln und Einschränkungen. Domäne CLI; Dateien nur unter `native/`, `tools/paper` wird nur gelesen.

## Beispiele

B-778 › Beispiele.

## Ausnahme- und Fehlerfälle

B-778 › Ausnahme- und Fehlerfälle.

## Akzeptanzkriterien

- **AC-01** Teile-Export für Monarch und Pferd, Rig-Datei mit Plätzen, byte-gleich bei Wiederholung (`B-778/AC-01`).
- **AC-02** Rig-Rechnung in Go gleich `tools/paper` für Figur und Vierbeiner (`B-778/AC-02`).
- **AC-03** Fenster mit Rig und Sheet, Umhang, Reiter auf Rig-Pferd in drei Gangarten, Waffe und Rüstung tauschbar (`B-778/AC-03`).
- **AC-04** Bildvergleich gegen das Sheet mit Zahl und Differenzbild (`B-778/AC-04`).
- **AC-05** Messbericht Entwicklungs-PC mit 2, 50 und 200 Rig-Figuren (`B-778/AC-05`).
- **AC-06** Sichtprüfung und Messwerte Mini-PC (`B-778/AC-06`, 🧑).

## Offene Fragen

B-778 › Offene Fragen (Schatten, Schärfe, Beine des Pferds, Farbvarianten, Platzhalter; je mit Default).

## Sessions

| Nr. | Datei | Typ | Agent | Status |
|---|---|---|---|---|
| NV2.1 | `NV2.1-teile-export-rig-daten.md` | Umsetzung | autonom | fertig |
| NV2.2 | `NV2.2-rig-laufzeit-go.md` | Umsetzung | autonom | fertig |
| NV2.3 | `NV2.3-bildvergleich-messung.md` | Umsetzung | autonom | fertig |
| NV2.4 | `NV2.4-sichtpruefung-mini-pc.md` | Review | Mensch | blockiert |

Bei `Reife: Entwurf` genügen Stichpunkte mit den Kriterien in Klammern statt Session-Dateien. Vor dem Aktivieren
wird jede Session als eigene Datei nach `docs/vorlagen/session.md` geschrieben. Keine eigenen Sim- oder
Balance-Sessions vor der Abnahme: Sim, Balance, Gen und `check:race` laufen im Abnahme-Ticket (B-742).

## Abnahme

Wird von der Review-Session (Doku-Sprint: letzte Session) ausgefüllt, höchstens fünf Zeilen: Datum, Kriterien
(Verweis auf Session-Ergebnisse), behobene Befunde, neue Tickets und das Abnahme-Ticket für Sim, Balance und Gen
(genau eins, `docs/arbeitsweise.md` › Sim, Balance und Gen). Bis dahin `–`.
