# NG12 · CLI · Natives Spiel: Anzeigen an Figuren, Bäume und Geschosse

- **Status:** geplant
- **Projekt:** NAT
- **Domäne:** CLI
- **Reife:** bereit
- **Tickets:** B-806, B-811
- **Start-Commit:** – (wird beim Aktivieren gesetzt: `git rev-parse --short origin/develop`)
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** autonom, Claude, 2026-11-12

## Ausgangslage

Der Funktionsabgleich NG9.1 zeigt: Die Figuren des nativen Spiels zeichnen keine Lebensbalken, keinen Monarchen-Punkt, kein Beute-Gold und keine Last; freie Monarchen sind sichtbar. Plantagen-Bäume zeigen beim Fällen weder Balken noch Schrumpfen, gegnerische Geschosse sind nicht rot. Einzelheiten: B-806, B-811.

## Ziel

Figuren, Bäume und Geschosse des nativen Spiels zeigen dieselben Anzeigen wie der Browser-Client. Am Ende sichtbar: Tests in `actors`, `props`, `fx` grün, `task native:game:e2e` mit zwei gleichen Läufen, Bilder unter `reports/native-game-e2e/`.

## Beteiligte und Zielgruppen

Spieler am PC und Mini-PC; Claude (Umsetzung, Review); 🧑 Ralf (Sichtprüfung bei der nächsten PC-Abnahme, nicht blockierend).

## Anforderungen

`B-806 › Anforderungen`; `B-811 › Anforderungen` nur Baum beim Fällen und Geschossfarbe (die Anleitung mit Münze und Mond läuft in NG13).

## Nicht-Ziele

Anleitungsbilder in `hud` (B-811/AC-03, NG13), Menü-Optik (B-812), neue Assets, Sim- und Balance-Läufe.

## Regeln und Einschränkungen

Entscheidung 008; Importregel aus `native/k3c-game/view/doc.go` (Fachpakete nur `view`, `art`, `engine/sim` lesend, `data`); kein Zeichnen im `Update`, keine Allokation je Bild, Werte nur aus dem Frame. Datei ≤ 400 Zeilen, Funktion ≤ 60 Zeilen.

Planungsangaben für die Arbeitswellen (`docs/arbeitsweise.md` › Parallelität und Wellen), je eine Zeile, nur IDs und Pfade:

- **Bereich:** `native/k3c-game/actors/**`, `native/k3c-game/props/**`, `native/k3c-game/fx/**`
- **Abhängig von:** –
- **Braucht 🧑:** –
- **Sperre:** –

## Beispiele

`B-806 › Beispiele`, `B-811 › Beispiele` (Baum, Geschoss).

## Ausnahme- und Fehlerfälle

`B-806 › Ausnahme- und Fehlerfälle`. Figur ohne Lebenswert im Frame → kein Balken, kein Absturz.

## Akzeptanzkriterien

- **AC-01** Lebensbalken je Figurentyp wie im Browser (`B-806/AC-01`).
- **AC-02** Goldkreis, Monarchen-Punkt, freier Sitz unsichtbar (`B-806/AC-02`).
- **AC-03** Last der Bauern (`B-806/AC-03`).
- **AC-04** Bildvergleich zweier Läufe gleich, Bilder abgelegt (`B-806/AC-04`).
- **AC-05** Gefällter Baum mit Balken und Schrumpffaktor (`B-811/AC-01`).
- **AC-06** Geschossfarbe nach Seite, Gegner rot (`B-811/AC-02`).

## Offene Fragen

keine (entschieden autonom, Claude, 2026-11-12: Sichtprüfung läuft bei der nächsten PC-Abnahme mit, siehe `B-806 › Offene Fragen`).

## Sessions

| Nr. | Datei | Typ | Agent | Status |
|---|---|---|---|---|
| NG12.1 | `NG12.1-balken-punkt-gold.md` | Umsetzung | autonom | offen |
| NG12.2 | `NG12.2-last-bildvergleich.md` | Umsetzung | autonom | offen |
| NG12.3 | `NG12.3-baum-geschoss.md` | Umsetzung | autonom | offen |
| NG12.4 | `NG12.4-review.md` | Review | autonom | offen |

Bei `Reife: Entwurf` genügen Stichpunkte mit den Kriterien in Klammern statt Session-Dateien. Vor dem Aktivieren
wird jede Session als eigene Datei nach `docs/vorlagen/session.md` geschrieben. Keine eigenen Sim- oder
Balance-Sessions vor der Abnahme: Sim, Balance, Gen und `check:race` laufen im Abnahme-Ticket (B-742).

## Abnahme

Wird von der Review-Session (Doku-Sprint: letzte Session) ausgefüllt, höchstens fünf Zeilen: Datum, Kriterien
(Verweis auf Session-Ergebnisse), behobene Befunde, neue Tickets und das Abnahme-Ticket für Sim, Balance und Gen
(genau eins, `docs/arbeitsweise.md` › Sim, Balance und Gen). Bis dahin `–`.
