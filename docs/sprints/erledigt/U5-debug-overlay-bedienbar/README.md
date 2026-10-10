# U5 · CLI · Debug-Overlay und Cheat-Dialog bedienbar

- **Status:** erledigt
- **Projekt:** BED
- **Domäne:** CLI
- **Reife:** bereit
- **Tickets:** B-192, B-317, B-191
- **Start-Commit:** 07e3c329
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** 2026-10-09, 🧑 im Chat (Revision 1, Übergangsregeln zu den offenen Fragen)

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

Beschluss 🧑 2026-10-06 (Chat): Bei Touch liegen Debug-Overlay und Aktionsliste rechts neben dem linken Lauf-Feld, unten mittig; das HUD oben bleibt frei (B-191). **Ersetzt** durch den Beschluss vom 2026-10-09.

Beschluss 🧑 2026-10-09 (Chat, U5.1): Die Info-Zeilen liegen auch bei Touch links unten über der Skill-Zeile. Unten mittig liegen die Touch-Tasten über dem Canvas, darüber Beitritts-Hinweis und Reise-Text; das Lauf-Feld ist der ganze Bildschirm (geteilt an der Mitte), der Phaser-Text fängt keine Touches ab.

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
| U5.1 | `U5.1-oe-hud-frei.md` | Umsetzung | autonom | fertig |
| U5.2 | `U5.2-cheat-dialog-fokus.md` | Umsetzung | autonom | fertig |
| U5.3 | `U5.3-review.md` | Review | autonom | fertig |
| U5.4 | `U5.4-abnahme-pc-handy-xbox.md` | Umsetzung | Mensch | fertig |

## Abnahme

Review 2026-10-09 (U5.3, eigener Review-Agent): `task check` grün; ein schwerer Befund behoben: Bei offenem Cheat-Dialog lief das Skill-Menü weiter, Leertaste/A hätte Skills lernen oder zurücksetzen können (`GameScene`: `route` ohne Eingaben, Menüs schließen).
- AC-01, AC-03 geprüft (U5.1); AC-02 geprüft für Tests und Browser-Pane (U5.2); AC-05 geprüft. B-192 erledigt.
- Verschoben nach U5.4: B-191/AC-02, B-317/AC-03, B-317/AC-04 und AC-04 (Xbox-Prüfung; Befund und Prüfauftrag im Ergebnis von U5.2).
- Abnahme U5.4 (🧑, 2026-10-10, PC und Touch in Chrome): AC-02 bestanden (Cheat-Dialog mit Fokus, Pfeiltasten, Leertaste, Klick). AC-03 mit Befunden: Ö schließt bei offenem Dialog das Overlay mit (B-387); Touch-Tasten und Start-Button verdecken bei ca. 2000 px Breite HUD-Zeilen (B-388). AC-04 (Xbox) verschoben, niedrigste Priorität.
- Neue Tickets: B-387, B-388, B-389 (Befunde aus der Abnahme, freigegeben 2026-10-10; B-389: Beitritts-Hinweis für Spieler 2 fehlt).
- Version: v0.15.0 vorgeschlagen (neue Bedienung des Cheat-Dialogs mit Tastatur und Klick); nicht gesetzt (wartet auf Bestätigung 🧑).
