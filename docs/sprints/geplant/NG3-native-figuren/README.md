# NG3 · CLI · Natives Spiel: Figuren als Rig, Bein-Bänder und Gegnerzahl

- **Status:** geplant
- **Projekt:** NAT
- **Domäne:** CLI
- **Reife:** bereit
- **Tickets:** B-796, B-791, B-780
- **Start-Commit:** – (wird beim Aktivieren gesetzt: `git rev-parse --short origin/develop`)
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** 2026-11-11 🧑 Chat (Ralf-Queue natives Spiel)

## Ausgangslage

Nach NG1 zeichnet `native/k3c-game/actors` nur Platzhalter. Rig-Dateien für alle Figuren mit Teile-Baum und das Zeichenpaket `native/k3c-rig/paint` gibt es seit NV3; bei Elefant, Wolf und Alphawolf klappen die Bein-Bänder um (B-791). Wie viele Rig-Gegner gleichzeitig laufen, ist offen (B-780). Einzelheiten: B-796, B-791, B-780.

## Ziel

Alle Figuren laufen im nativen Spiel aus dem Spielzustand, mit Teile-Baum als Rig, `requisite` als Sheet; die Grenze Rig gegen Sheet am Entwicklungs-PC ist gemessen. Am Ende sichtbar: zwei Monarchen auf Reittieren, Bürger, Truppen und eine Gegner-Welle als Rig, Messbericht unter `reports/`.

## Beteiligte und Zielgruppen

Familie, Claude (Umsetzung, Messung), 🧑 Ralf (Entscheidung zu B-780, Sichtprüfung in NG9).

## Anforderungen

`B-796 › Anforderungen`, `B-791 › Anforderungen`, `B-780 › Anforderungen` (Teil Entwicklungs-PC; Teil Mini-PC in NG9).

## Nicht-Ziele

`B-796 › Nicht-Ziele`; Messung am Mini-PC (NG9, B-786); Lasttest über `k3c-load` oder `sim_test` (gesperrt).

## Regeln und Einschränkungen

Rig-Format `native/k3c-rig/RIG.md`, Entscheidung 005 und 008. Vertrag `view` fest. Messläufe je Fassung einmal.

Planungsangaben für die Arbeitswellen (`docs/arbeitsweise.md` › Parallelität und Wellen), je eine Zeile, nur IDs und Pfade:

- **Bereich:** `native/k3c-game/actors/**`, `native/k3c-rig/rig/**`
- **Abhängig von:** NG1
- **Braucht 🧑:** Freigabe, Entscheidung
- **Sperre:** –

## Beispiele

`B-796 › Beispiele`, `B-791 › Beispiele`, `B-780 › Beispiele`.

## Ausnahme- und Fehlerfälle

`B-796 › Ausnahme- und Fehlerfälle`, `B-780 › Ausnahme- und Fehlerfälle`.

## Akzeptanzkriterien

- **AC-01** Bein-Bänder falten nie, `knownFolds` leer (`B-791/AC-01`), Spec-Auffälligkeiten geklärt (`B-791/AC-02`).
- **AC-02** Monarch und Reittier aus dem Zustand (`B-796/AC-01`).
- **AC-03** Jeder Figurentyp hat Rig oder Sheet (`B-796/AC-02`).
- **AC-04** Messbericht Rig gegen Sheet mit 2, 50, 200 Gegnern am Entwicklungs-PC (`B-796/AC-03`, `B-780/AC-01` Teil Entwicklungs-PC) und Empfehlung zur Reihenfolge (`B-780/AC-02`, Entscheidung 🧑 offen).

## Offene Fragen

- B-791: Kurve am Band-Ende begrenzen oder Spec des Elefanten ändern? Vorschlag: Kurve begrenzen, Spec unverändert. Entscheidet 🧑.
- B-780: Maßstab (Ziel-Bildrate, größte Gegnerzahl) und Waffen/Rüstungen (`B-780 › Offene Fragen`).

## Sessions

| Nr. | Datei | Typ | Agent | Status |
|---|---|---|---|---|
| NG3.1 | `NG3.1-bein-baender.md` | Umsetzung | autonom | offen |
| NG3.2 | `NG3.2-monarch-reittier.md` | Umsetzung | autonom | offen |
| NG3.3 | `NG3.3-figuren-alle.md` | Umsetzung | autonom | offen |
| NG3.4 | `NG3.4-messung-gegnerzahl.md` | Umsetzung | autonom | offen |
| NG3.5 | `NG3.5-review.md` | Review | autonom | offen |

Bei `Reife: Entwurf` genügen Stichpunkte mit den Kriterien in Klammern statt Session-Dateien. Vor dem Aktivieren
wird jede Session als eigene Datei nach `docs/vorlagen/session.md` geschrieben. Keine eigenen Sim- oder
Balance-Sessions vor der Abnahme: Sim, Balance, Gen und `check:race` laufen im Abnahme-Ticket (B-742).

## Abnahme

Wird von der Review-Session (Doku-Sprint: letzte Session) ausgefüllt, höchstens fünf Zeilen: Datum, Kriterien
(Verweis auf Session-Ergebnisse), behobene Befunde, neue Tickets und das Abnahme-Ticket für Sim, Balance und Gen
(genau eins, `docs/arbeitsweise.md` › Sim, Balance und Gen). Bis dahin `–`.
