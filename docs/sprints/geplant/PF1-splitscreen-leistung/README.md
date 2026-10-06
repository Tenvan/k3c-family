# PF1 · CLI · Split-Screen flüssig auf der Xbox

- **Status:** geplant
- **Domäne:** CLI
- **Prio:** hoch
- **Reife:** Entwurf
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

## Beispiele

Zwei Kameras, eine Nacht → FPS im Debug-Overlay ≥ angenommenes Ziel.

## Ausnahme- und Fehlerfälle

Ziel nicht erreichbar → Messwerte und Ticket, angenommener Wert bleibt.

## Akzeptanzkriterien

- **AC-01** Der Split-Screen läuft auf der Xbox flüssig (B-194/AC-01, B-194/AC-02).

## Offene Fragen

keine

## Sessions

Entwurf. Vor dem Aktivieren jede Session als Datei nach `docs/vorlagen/session.md` schreiben, die Kriterien in Klammern werden ihr Feld `Kriterien`.

- PF1.1 Profil und Optimierung der zweiten Kamera (AC-01).
- PF1.2 Review (Code-Sprint): alle Kriterien prüfen.
- PF1.3 Messung an der Xbox (Mensch) (AC-01).

## Abnahme

–
