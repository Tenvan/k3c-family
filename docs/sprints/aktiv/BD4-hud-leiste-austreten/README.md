# BD4 · CLI · Gemeinsame HUD-Leiste oben, Fluss unten, lokale Spieler treten einzeln aus

- **Status:** aktiv
- **Projekt:** BED
- **Domäne:** CLI
- **Reife:** bereit
- **Tickets:** B-411, B-413
- **Start-Commit:** 013dcb34
- **Spec:** freigegeben
- **Revision:** 2
- **Freigabe:** autonom, Claude, 2026-10-22 (Ralf: autonom abarbeiten)

## Ausgangslage

Lokale Spieler können beitreten, aber nicht einzeln austreten (B-411, Spec noch leer). Werte für alle Spieler (Vorrat, Hub, Tageszeit, Kampf) belegen Platz in den Spielerfeldern, der Kompass steht unten, der Boden ist eine einfarbige Fläche (B-413). POC zu B-413 auf Branch `poc/hud-leiste-fluss`.

## Ziel

Eine gemeinsame Leiste oben zeigt die Werte aller Spieler, die Spielerfelder nur Spielerwerte, unten fließt ein Fluss mit gespiegeltem Hintergrund; jeder lokale Spieler kann einzeln austreten. Am Ende sichtbar: neues HUD am TV, `hudLayout.test.ts` grün.

## Beteiligte und Zielgruppen

Spielende im Split-Screen (1–4 Spieler); Agent in CLI; 🧑 entscheidet die offenen Punkte des POC und nimmt am Gerät ab.

## Anforderungen

B-411 › Anforderungen (vor der Freigabe ausfüllen), B-413 › Anforderungen. Der POC (`poc/hud-leiste-fluss`) ist Vorlage, nicht Ergebnis: übernommen wird nur, was die Kriterien verlangen.

## Nicht-Ziele

Spiegelung der Figuren im Fluss; neue Grafiken (Projekt GRA); Server-Änderungen – braucht das Austreten eine Protokoll-Nachricht, kommt sie als eigenes SRV-Ticket.

## Regeln und Einschränkungen

Domäne CLI; `src/scenes` zeichnet nur Snapshots (`noSim.test.ts`); Layout als reine Funktionen mit Test; 2 Spieler gleichzeitig; Home-Button oben mittig und Touch-Flächen freihalten; B nicht belegen, View + Menu reserviert. Datei ≤ 400 Zeilen, Funktion ≤ 60.

## Beispiele

3 Spieler im Split-Screen → Vorrat, Hub, Tageszeit, Kampf nur in der Leiste oben, je Feld oben der Kompass. Spieler 2 tritt aus → sein Feld verschwindet, Spieler 1 und 3 spielen weiter.

## Ausnahme- und Fehlerfälle

Letzter lokaler Spieler meldet sich ab → ohne Rückfrage wie „Spiel verlassen“ (B-411). `removeSlot` abgelehnt → Feld bleibt, Warnung im Log. Touch-Modus (`?touch=1`) → Leiste weicht den Touch-Flächen aus.

## Akzeptanzkriterien

- **AC-01** Bei 1 bis 4 Spielern stehen Vorrat/Hub, Tageszeit und Kampf nur in der Leiste oben, nie in einem Spielerfeld (B-413/AC-01).
- **AC-02** Der Kompass steht je Feld oben, im obersten Feld unter der Leiste, ohne Überlagerung (B-413/AC-02).
- **AC-03** Unter der Bodenlinie liegt ein Fluss mit kopfüber gespiegeltem Hintergrund und strömenden Wellen; Beobachtung am Gerät (B-413/AC-03).
- **AC-04** Die Leiste weicht dem Home-Button und den Touch-Flächen aus (B-413/AC-04).
- **AC-05** „Spieler abmelden“ trifft den Öffner des Menüs bzw. bei zwei Tastatur-Spielern den gewählten: `removeSlot` nur für dessen Slot, beim letzten lokalen Spieler ohne Rückfrage `leave` (B-411/AC-01 bis B-411/AC-04).
- **AC-06** Am Gerät: Ein Spieler meldet sich ab, die anderen spielen weiter, Wiederbeitritt per A (B-411/AC-05).

## Offene Fragen

Entschieden mit Default (autonom, Claude), Prüfung durch 🧑: Goldtopf B-525, Hintergrund-Höhe B-526, Abmelden mit zwei Tastatur-Spielern B-527.

## Sessions

| Nr. | Datei | Typ | Agent | Status |
|---|---|---|---|---|
| BD4.1 | `BD4.1-hud-leiste-kompass.md` | Umsetzung | autonom | fertig |
| BD4.2 | `BD4.2-fluss-spiegelung.md` | Umsetzung | autonom | offen |
| BD4.3 | `BD4.3-austreten-menue.md` | Umsetzung | autonom | offen |
| BD4.4 | `BD4.4-workshop-abnahme.md` | Workshop | Mensch | offen |
| BD4.5 | `BD4.5-review.md` | Review | autonom | offen |

Entwurf. Vor dem Aktivieren jede Session als Datei nach `docs/vorlagen/session.md` schreiben, die Kriterien in Klammern werden ihr Feld `Kriterien`.

- BD4.1 HUD-Leiste oben und Kompass je Feld, Layout-Tests (AC-01, AC-02, AC-04).
- BD4.2 Fluss mit gespiegeltem Hintergrund unter der Bodenlinie (AC-03).
- BD4.3 „Austreten“ im Spielmenü (AC-05).
- BD4.4 Workshop (🧑): Abnahme am Gerät, HUD-Leiste, Fluss, Austreten (AC-03, AC-06).
- BD4.5 Review (Code-Sprint): alle Kriterien prüfen.

## Abnahme

–
