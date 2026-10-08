# U5 · CLI · Debug-Overlay und Cheat-Dialog bedienbar

- **Status:** geplant
- **Projekt:** BED
- **Domäne:** CLI
- **Prio:** hoch
- **Reife:** bereit
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

CLI; B nicht belegen, View + Menu reserviert. `src/scenes` rechnet nichts (`noSim.test.ts`), Datei ≤ 400, Funktion ≤ 60 Zeilen.

Beschluss 🧑 2026-10-06 (Chat): Bei Touch liegen Debug-Overlay und Aktionsliste rechts neben dem linken Lauf-Feld, unten mittig; das HUD oben bleibt frei (B-191).

Beschluss 🧑 2026-10-06 (Chat): Die Ursache, warum der Controller auf der Xbox nicht wirkt, klärt die Umsetzung über einen Bericht von `gamepad-test.html`; die Vermutung (Edge nutzt das D-Pad für Spatial Navigation zwischen HTML-Buttons) ist ungeprüft (B-317).

## Beispiele

Ö drücken → Overlay und Liste zu; Pfeil runter im Dialog → nächster Button hervorgehoben.

## Ausnahme- und Fehlerfälle

Dialog offen und Ö → Dialog zuerst zu, Overlay bleibt.

## Akzeptanzkriterien

- **AC-01** Die Dev-Aktionsliste schließt sich mit Ö (B-192/AC-01, B-192/AC-02).
- **AC-02** Der Cheat-Dialog zeigt den Fokus und lässt sich mit Pfeiltasten, Leertaste und Controller bedienen (B-317/AC-01, B-317/AC-02, B-317/AC-03, B-317/AC-04).
- **AC-03** Debug-Overlay und Aktionsliste verdecken das HUD nicht (B-191/AC-01, B-191/AC-02).
- **AC-04** Die Ursache, warum der Controller den Cheat-Dialog auf der Xbox nicht bedient, ist mit einem Bericht von `gamepad-test.html` (`reports/*.json`) belegt und behoben oder als Ticket festgehalten.
- **AC-05** `task check` ist grün.

## Offene Fragen

- Nicht blockierend: B-192 beschreibt die alte Aktionsliste `DevActionPanel` mit `.k3c-dev{display:flex}`; seit B-231 (Commit `5c03798d`) ist sie durch den modalen Cheat-Dialog (`CheatDialog`, Ä) ersetzt, der `.k3c-cheat[hidden]{display:none}` schon setzt. Ob B-192 damit erledigt ist oder Ö den Cheat-Dialog mit schließen soll, entscheidet 🧑; bis dahin gilt „Ausnahme- und Fehlerfälle“ (Ö schließt zuerst den Dialog, Overlay bleibt).
- Nicht blockierend: Ob der modale Cheat-Dialog (80 % der Fläche, Raum angehalten) unter AC-03 fällt oder nur die Info-Zeilen; bis dahin verschiebt U5.1 die Info-Zeilen und lässt die Größe des Dialogs unverändert.

## Sessions

| Nr. | Datei | Typ | Agent | Status |
|---|---|---|---|---|
| U5.1 | `U5.1-oe-hud-frei.md` | Umsetzung | autonom | offen |
| U5.2 | `U5.2-cheat-dialog-fokus.md` | Umsetzung | autonom | offen |
| U5.3 | `U5.3-review.md` | Review | autonom | offen |
| U5.4 | `U5.4-abnahme-pc-handy-xbox.md` | Umsetzung | Mensch | offen |

## Abnahme

–
