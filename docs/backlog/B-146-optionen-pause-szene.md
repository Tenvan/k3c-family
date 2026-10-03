# B-146 · Der Client hat eine Optionen- und Pause-Szene mit getrennter Lautstärke und Barrierefreiheit

- **Domäne:** CLI
- **Typ:** Idee
- **Prio:** hoch
- **Status:** eingeplant
- **Sprint:** S5
- **Erstellt:** 2026-10-02
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** 2026-10-03, Chat (Ralf), mit Sprint S5

## Ausgangslage

`PlayerInput` kennt die Aktion `pause` (Menu-Taste, Esc) in `src/input/playerInput.ts`, aber es gibt keine Szene dafür; Menu zusammen mit View ist die reservierte Home-Kombi (`CLAUDE.md`). Es gibt weder Lautstärke-Regler noch Schalter für Screenshake, Flackern oder Farbschwäche-Symbole; Einstellungen je Gerät fehlen (`src/core/saveStore.ts` kennt nur Spielstände).

## Ziel

Eine Optionen-/Pause-Szene ist mit Controller, Tastatur und Touch bedienbar. Sie stellt Lautstärke Musik und SFX getrennt ein, schaltet Screenshake und Flash ab und aktiviert Farbschwäche-Symbole. Nutzen: Kinder und empfindliche Spieler können das Spiel anpassen (Plan Lücken 2 und 12).

## Beteiligte und Zielgruppen

Spieler am TV, Handy und PC; 🧑 testet am Gerät und entscheidet die Pause-Semantik (B-135).

## Anforderungen

- Neue Szene in `src/scenes/` (Pause/Optionen), geöffnet über die Aktion `pause`; View + Menu gemeinsam bleibt „zurück zur Landingpage“ und wird nicht umbelegt, B bleibt unbelegt.
- Regler Lautstärke Musik und SFX getrennt, Schalter Screenshake aus, Flash/Flackern aus, Farbschwäche-Symbole an; Werte je Gerät im `localStorage` (mit try/catch, das Spiel läuft auch ohne Speicher), Standard: alles an, Lautstärke 100 %.
- Was Pause bewirkt (nur lokal oder Raum hält an) richtet sich nach der Regel aus B-135; die Szene setzt sie um und enthält keine Spiel-Logik.
- Die Einstellungen liegen als reine Funktionen/Datenstruktur vor (Standardwerte, Lesen, Schreiben, Begrenzen), damit Audio (B-011) und Juice (B-164) sie lesen können.
- Bedienung mit 2+ lokalen Spielern: Zuordnung, welches Gerät das Menü steuert, laut Pause-Regel.

## Nicht-Ziele

Audio-Mixer selbst (B-011), Juice-Effekte (B-164), Controller-Glyphen (B-149), Speichern des Spielstands (B-147).

## Regeln und Einschränkungen

`CLAUDE.md` (Client rechnet nichts, Home-Button oben, ca. 70 px frei, B nie belegen, View + Menu reserviert); Eingabe nur über `PlayerInput`. Datei ≤ 400 Zeilen, Funktion ≤ 60, Schichtgrenzen aus `docs/arbeitsweise.md`; `src/input/` gehört zu PLAT, jede Änderung dort braucht die Freigabe der Spec.

## Beispiele

Spieler öffnet mit Menu die Pause, stellt Musik auf 0 % und SFX auf 50 %, schaltet Screenshake aus → nach dem Neuladen gelten die Werte noch.

## Ausnahme- und Fehlerfälle

`localStorage` gesperrt oder gelöscht → Standardwerte, kein Absturz. Gespeicherter Wert außerhalb 0–100 % → wird auf den Bereich begrenzt.

## Akzeptanzkriterien

- **AC-01** Test: Die Einstellungen liefern Standardwerte, begrenzen Werte auf 0–100 % und fallen bei kaputtem oder fehlendem `localStorage` auf die Standardwerte zurück.
- **AC-02** Die Szene öffnet und schließt sich per `pause` mit Controller, Tastatur und Touch; Lautstärke Musik und SFX, Screenshake aus, Flash aus und Farbschwäche-Symbole sind getrennt einstellbar (Beobachtung am Gerät).
- **AC-03** Die Einstellungen bleiben nach Neuladen der Seite erhalten.
- **AC-04** View + Menu führt weiterhin zur Landingpage, B ist in der Szene nicht belegt (Beobachtung am Gerät, `tests/projectRules.test.ts` grün).
- **AC-05** 🧑 hat die Szene am TV und am Handy abgenommen.

## Offene Fragen

Pause-Semantik im gemeinsamen Raum (lokal oder raumweit, wer darf): 🧑, `docs/fragenkatalog.md Q01`, Regel in B-135.

## Notizen

Aus Plan Phase 1 (S5) und Lücken 2 und 12.
