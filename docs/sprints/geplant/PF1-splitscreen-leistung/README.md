# PF1 · CLI · Split-Screen flüssig auf der Xbox

- **Status:** geplant
- **Projekt:** –
- **Domäne:** CLI
- **Prio:** hoch
- **Reife:** bereit
- **Einschiebbar:** nein
- **Tickets:** B-194
- **Start-Commit:** –
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

Der Split-Screen ruckelt auf der Xbox (B-194); N2 hat Zeitleiste und Vorhersage gebracht.

## Ziel

Zwei Spieler im Split-Screen spielen auf der Xbox flüssig. Am Ende sichtbar: Zwei Spieler im Split-Screen ohne sichtbares Ruckeln auf der Xbox.

## Beteiligte und Zielgruppen

🧑 misst an der Xbox; Agent profiliert im Browser-Pane.

## Anforderungen

B-194 › Anforderungen.

## Nicht-Ziele

Netz-Latenz (N2), Atlas (GR4).

## Regeln und Einschränkungen

CLI; Hardware entkoppelt: Messung an der Xbox ist eine Mensch-Session.

Beschluss 🧑 2026-10-06 (Chat): Die Rückfrage aus B-194 (Szenario der Testseite, ruckelt auch `game.html`?) bleibt unbeantwortet und blockiert nicht; die erste Session misst. Die Messung an der Xbox ist eine Mensch-Session nach dem Review (Hardware entkoppelt, `docs/plan-weiterentwicklung.md` § 11.6).

## Beispiele

Zwei Kameras, eine Nacht → FPS im Debug-Overlay ≥ angenommenes Ziel.

## Ausnahme- und Fehlerfälle

Ziel nicht erreichbar → Messwerte und Ticket, angenommener Wert bleibt.

## Akzeptanzkriterien

- **AC-01** Der Split-Screen läuft auf der Xbox flüssig (B-194/AC-01, B-194/AC-02).
- **AC-02** Im Browser-Pane sind FPS und Frame-Zeit mit einer und mit zwei Kameras (Tag und Nacht) sowie der Snapshot-Abstand gemessen; Messwerte und vermutete Ursache stehen in B-194 › Notizen.
- **AC-03** Die gemessene Hauptlast aus AC-02 ist behoben oder gesenkt; Frame-Zeit im Browser-Pane mit zwei Kameras vorher und nachher steht im Ergebnis von PF1.2.
- **AC-04** `task check` ist grün; keine geänderte Datei über 400 Zeilen, keine Funktion über 60.

## Offene Fragen

Nicht blockierend: Welche Szenario-Einstellung lief am 2026-10-03 auf der Testseite, und ruckelt auch `game.html` mit zwei Controllern? Klärt 🧑 spätestens in PF1.4 am Gerät.

## Sessions

| Nr. | Datei | Typ | Agent | Status |
|---|---|---|---|---|
| PF1.1 | `PF1.1-profil.md` | Umsetzung | autonom | offen |
| PF1.2 | `PF1.2-korrektur.md` | Umsetzung | autonom | offen |
| PF1.3 | `PF1.3-review.md` | Review | autonom | offen |
| PF1.4 | `PF1.4-xbox-messung.md` | Umsetzung | Mensch | offen |

## Abnahme

–
