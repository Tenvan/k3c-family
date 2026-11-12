# B-741 · HUD ohne Tastenhinweise, Tastenbelegung im Menü

- **Domäne:** CLI
- **Typ:** Idee
- **Prio:** mittel
- **Umgebung:** offline
- **Status:** verworfen
- **Sprint:** –
- **Projekt:** BED
- **Erstellt:** 2026-11-08
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

Das HUD nennt Tasten direkt: Steuerzeile unten rechts (`hud.hint.*`, `HudScene`/`controlsHint.ts`), Aktionen-Overlay („A halten: Mauer bauen“, `actionHints.ts`) und Führungs-Hinweise („{key} halten: …“, `guide.*`). Das Menü hat keine Übersicht der Belegung.

## Ziel

Das HUD zeigt keine Tastennamen mehr; die Belegung steht im Optionen-Menü unter „Tastenbelegung“.

## Beteiligte und Zielgruppen

Spieler an Tastatur, Controller und Touch; Entwickler (🧑).

## Anforderungen

- Steuerzeile im HUD entfällt.
- Aktionen-Overlay und Führungs-Hinweise zeigen nur die Aktion ohne Taste/Glyph.
- Optionen-Eintrag „Tastenbelegung“ öffnet eine Seite, die alle Belegungen je Gerät listet; Quelle sind `slotBindings.ts`, `keyboardLayouts.ts`, Touch-Beschriftung (keine zweite Hartkodierung).
- Mindestschrift Q03, 2 Spieler (Tastatur 1 und 2) sichtbar.

## Nicht-Ziele

Beitritts-Hinweise (`hud.join.*`) bleiben (kein Menü vor dem Beitritt); kein Umbelegen der Tasten.

## Regeln und Einschränkungen

B und View+Menu bleiben reserviert; Texte nur in `texts.*.ts` (de/en); `optionsLogic` ohne Phaser testbar.

## Beispiele

Optionen › Tastenbelegung → Zeilen „Schlag: X · E · Num 0 · ⚔“ usw.

## Ausnahme- und Fehlerfälle

nicht relevant: reine Anzeige.

## Akzeptanzkriterien

- **AC-01** Im HUD erscheint kein Text mit Tastennamen außer Beitritts-Hinweisen.
- **AC-02** Optionen enthalten „Tastenbelegung“; die Seite listet Bestätigen, Sprinten, Schlag, Skills 1–4, Skill-Menü, Pause, Vollbild für Pad, Tastatur 1, Tastatur 2, Touch.
- **AC-03** Ein Test leitet die Zeilen aus `slotBindings`/`KEYBOARD_1`/`KEYBOARD_2` ab.
- **AC-04** Abnahme am Gerät durch 🧑 (Anzeige).

## Offene Fragen

keine

## Notizen

Auftrag aus der Schaltzentrale; zusammen mit B-740 auf einem Branch.

Verworfen 2026-11-12 (Entscheidung Ralf): abgelöst durch B-823 (nur noch natives Spiel am PC, Entscheidung 009). Umgesetzt im Browser-Client mit #518 (0d4f6563c), die Abnahme entfällt.
