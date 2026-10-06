# U5 · CLI · Debug-Overlay und Cheat-Dialog bedienbar

- **Status:** geplant
- **Domäne:** CLI
- **Prio:** hoch
- **Reife:** Entwurf
- **Einschiebbar:** nein
- **Tickets:** B-192, B-317, B-191
- **Start-Commit:** –
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

Die Dev-Aktionsliste schließt nicht mit Ö (B-192), der Cheat-Dialog zeigt keinen Fokus und ist nicht mit Pfeiltasten und Controller bedienbar (B-317), Overlay und Liste verdecken das HUD (B-191).

## Ziel

Overlay, Aktionsliste und Cheat-Dialog sind mit Maus, Tastatur und Controller bedienbar und verdecken das HUD nicht. Am Ende sichtbar: Ö schließt Overlay und Liste, HUD bleibt lesbar, Cheat-Dialog mit Fokus und Controller.

## Beteiligte und Zielgruppen

🧑 testet am PC; Agent baut in `src/scenes/`.

## Anforderungen

B-192 › Anforderungen; B-317 › Anforderungen; B-191 › Anforderungen.

## Nicht-Ziele

Neue Dev-Aktionen, Xbox-Zugang zum Overlay (B-195, PL1).

## Regeln und Einschränkungen

CLI; B nicht belegen, View + Menu reserviert.

## Beispiele

Ö drücken → Overlay und Liste zu; Pfeil runter im Dialog → nächster Button hervorgehoben.

## Ausnahme- und Fehlerfälle

Dialog offen und Ö → Dialog zuerst zu, Overlay bleibt.

## Akzeptanzkriterien

- **AC-01** Die Dev-Aktionsliste schließt sich mit Ö (B-192/AC-01, B-192/AC-02).
- **AC-02** Der Cheat-Dialog zeigt den Fokus und lässt sich mit Pfeiltasten, Leertaste und Controller bedienen (B-317/AC-01, B-317/AC-02, B-317/AC-03, B-317/AC-04).
- **AC-03** Debug-Overlay und Aktionsliste verdecken das HUD nicht (B-191/AC-01, B-191/AC-02).

## Offene Fragen

keine

## Sessions

Entwurf. Vor dem Aktivieren jede Session als Datei nach `docs/vorlagen/session.md` schreiben, die Kriterien in Klammern werden ihr Feld `Kriterien`.

- U5.1 Ö schließt Overlay und Liste, HUD frei (AC-01, AC-03).
- U5.2 Cheat-Dialog mit Fokus, Tastatur und Controller (AC-02).
- U5.3 Review (Code-Sprint): alle Kriterien prüfen.

## Abnahme

–
