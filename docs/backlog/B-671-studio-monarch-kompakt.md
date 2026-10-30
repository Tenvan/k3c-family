# B-671 · Die Monarch-Seite hat eine Werkzeugleiste und eine große Bühne, die Skills liegen in einem einklappbaren Seitenpanel

- **Domäne:** DEV
- **Typ:** Idee
- **Prio:** mittel
- **Umgebung:** offline
- **Status:** eingeplant
- **Sprint:** –
- **Projekt:** WZG
- **Erstellt:** 2026-10-30
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** 2026-10-30 🧑 Chat („specs sind alle freigegeben“)

## Ausgangslage

Monarch-Seite des Studios (`tools/k3c-studio/frontend/src/monarch/MonarchPage.tsx`). Links steht dauerhaft die Skill-Tabelle (`SkillList`, 26 Zeilen, Spalte bis 26rem, `.wd-monarch`-Grid), rechts drei getrennte Leisten: Auswahl (Reittier, Aktion, Bewegung, Animation, Grafik Reittier), Wiedergabe (`PlaybackControls` mit Textknöpfen) und Zoom. Darunter die Bühne mit fester Höhe 28rem und die Spielerfarben als Liste mit eigenen Auswahllisten. Bewegung und Animation sind gesperrt, sobald eine Aktion gewählt ist; das sieht man nur am ausgegrauten Feld. Gesehen am 2026-10-30 im Browser-Pane (1600×1000, `?mock=1`).

Skills: Das Spiel hat je Skill ein Aussehen (`src/scenes/effects/skillLooks.ts`: Farbe, Radius, Dauer, Partikel, Bewegung `burst`/`rise`/`fall`/`wave`, gerechnet von `skillFrame`; ohne eigenes Aussehen der Ring der Linie, Effekt `skill` in `EFFECT_CONFIG`). Die Seite zeigt davon nur ein Farbfeld und das Wort der Bewegung in der Tabelle; einen Skill auf der Bühne abspielen geht nicht. Die Aktion „Skill ohne Ziel“ zeigt nur den Text `cast.noTarget`.

## Ziel

Die Figuren bekommen die meiste Fläche, die ganze Bedienung steht in einer Leiste, die Skills sind auf Wunsch da und lassen sich so abspielen, wie das Spiel sie zeigt.

## Beteiligte und Zielgruppen

- 🧑 Ralf prüft Monarch-Grafik, Reittiere und Animationen im Studio.
- Sessions der Domäne DEV setzen um.

## Anforderungen

- **Eine Werkzeugleiste:** links Reittier, Modus als Segment-Knöpfe „Stehen · Laufen · Sprinten · Aktion … · Skill …“ (Aktion bzw. Skill öffnet die jeweilige Auswahl, Bewegung und Animation entfallen dann sichtbar statt nur gesperrt), Animation; rechts Wiedergabe als Symbol-Knöpfe (Schleife, Bild zurück, Pause/Weiter, Bild vor), Tempo als Segment (½× 1× 2×), Zoom als Segment (− 100 % +, Klick auf den Wert = Spielmaßstab), Knopf „Skills 26“. Lücken als ein Chip mit Anzahl, Details im Tooltip.
- **Bühne füllt den Rest** der Fensterhöhe (wie `.wd-live` im Welten Editor), mindestens die heutige Höhe.
- **Spieler als schmale Zeile unter der Bühne:** je Spieler Kennfarbe, Kurzname der Grafik, Auswahl der Präferenz als kleines Dropdown.
- **Skills als Seitenpanel rechts**, standardmäßig zu, öffnet über den Knopf; Filter nach Linie als Segment-Knöpfe; der Zustand (offen/zu) wird per `localStorage` gemerkt.
- **Skill-Animation abspielen:** Modus „Skill …“ wählt einen Skill (Auswahl nach Linie gruppiert), alternativ Klick auf eine Zeile im Skills-Panel (▶ je Zeile). Alle vier Monarchen spielen im selben Takt den Effekt des Spiels: eigenes Aussehen aus `skillLook` (Farbe, Radius, Partikel, Bewegung über `skillFrame`), sonst der Ring der Linie. Dazu die Animation des Monarchen beim Wirken (`attack`, sofern das Spiel sie nutzt; sonst Lücke „Wirk-Animation fehlt“). Dauer aus dem Aussehen, Schleife, Einzelbild und Tempo gelten auch für den Effekt.
- **Passive Skills** zeigen, was das Spiel zeigt; ohne Darstellung die Lücke wie heute in der Tabelle, kein erfundener Ersatz.
- Effekt-Code wird aus dem Spiel importiert (`@game/scenes/effects/*`), nicht nachgebaut; ein Test sichert, dass Studio und Spiel dieselbe Funktion nutzen.
- `PlaybackControls` als kompakte Variante in `tools/webkit` oder `src/playback/`, damit Gegner-Seite (B-672) sie teilt.

## Nicht-Ziele

- Neue Animationen, Aktionen oder Reittiere.
- Änderungen am Slot-Register (bleibt nur lesend).
- Bearbeiten von Skills.

## Regeln und Einschränkungen

- Domäne DEV: `tools/k3c-studio/frontend/src/monarch/`, `src/playback/`, `styles/world.css`, Bausteine in `tools/webkit`.
- Radix nur über `@webkit`, Atomic Design; keine neue Abhängigkeit; Datei ≤ 400 Zeilen, Funktion ≤ 60 Zeilen.
- Zoom mit Alt + Mausrad bleibt.

## Beispiele

- Seite öffnen → eine Leiste, darunter die vier Monarchen groß, keine Skill-Tabelle.
- „Skills 26“ klicken → Panel rechts mit Filter „Alle · Tank · Zauberer · Heiler · Dieb“, Bühne wird schmaler.
- Modus „Aktion …“ → Auswahl der Aktion erscheint, Bewegung und Animation sind ausgeblendet.
- Modus „Skill …“, `fireball` → vier Monarchen, je ein Effekt in der Farbe und Bewegung von `fireball`, in Schleife; Tempo ½× verlangsamt auch den Effekt.
- ▶ bei `meteor` im Panel → Modus springt auf „Skill“, `meteor` spielt mit Bewegung „fällt“.
- Skill ohne eigenes Aussehen → Ring in der Farbe der Linie.

## Ausnahme- und Fehlerfälle

- Keine Slots für ein Reittier oder einen Spieler → keine Präferenz-Auswahl, wie heute.
- Grafiken nicht geladen → Meldung in der Leiste, wie heute.
- Schmales Fenster (< 900 px) → Skills-Panel überlagert die Bühne statt sie zu verkleinern.
- Skill fehlt in `data/monarch.json` oder hat keine Darstellung → Lücken-Chip, Bühne zeigt nur den Monarchen.
- Einzelbild bei einem Skill → der Effekt steht auf dem entsprechenden Fortschritt (`skillFrame(look, p)`), nicht auf 0.

## Akzeptanzkriterien

- **AC-01** Auswahl, Wiedergabe und Zoom stehen in einer Leiste; bei 1600×1000 bricht sie nicht um (Browser-Pane, Port 5196, `?mock=1`).
- **AC-02** Die Skill-Tabelle ist standardmäßig zu und öffnet über den Knopf; der Zustand bleibt nach Neuladen (Vitest).
- **AC-03** Mit Modus „Aktion“ oder „Skill“ sind Bewegung und Animation nicht sichtbar; ohne den Modus keine Aktions- bzw. Skill-Auswahl (Vitest).
- **AC-04** Die Bühne ist bei 1600×1000 mindestens so hoch wie heute (28rem) und breiter als heute bei geschlossenem Panel (Browser-Pane).
- **AC-05** Bestehende Tests von `MonarchPage`, `PlaybackControls` und `monarchZoom` sind grün oder ohne Verlust an geprüftem Verhalten angepasst.
- **AC-06** Jeder der 26 Skills lässt sich abspielen: mit eigenem Aussehen genau mit `skillLook(id)`, sonst mit dem Ring der Linie; Skills ohne Darstellung zeigen die Lücke (Vitest auf der Logik, Bühnen-Attrappe).
- **AC-07** Schleife, Tempo und Einzelbild wirken auf den Skill-Effekt; Einzelbild zeigt den Fortschritt aus `skillFrame` (Vitest).
- **AC-08** Browser-Pane: `fireball`, `meteor` und ein Skill ohne eigenes Aussehen sind auf der Bühne sichtbar (Screenshot in der Abnahme).

## Offene Fragen

- Skills-Panel rechts oder links? Default: rechts (Mock). Entscheidet 🧑.
- Skill-Effekt am Monarchen oder am Ziel (z. B. vor dem Monarchen in Reichweite)? Default: dort, wo das Spiel ihn zeichnet (Ereignis `skill` mit x-Stelle); prüft die umsetzende Session im `worldRenderer`. Entscheidet 🧑 bei Abweichung.

## Notizen

- Mock aus dem Chat vom 2026-10-30 (Widget „studio_planer_monarch_gegner_mock“, Reiter Monarch).
- Geschwister-Tickets: B-670 (Gebäude-Planer), B-672 (Gegner).
- Skill-Animationen auf Wunsch von 🧑 am 2026-10-30 ergänzt (Anforderungen, AC-06 bis AC-08).
- **Umgesetzt 2026-10-30:** `monarch/useMonarch.ts`, `MonarchToolbar.tsx`, `SkillPanel.tsx` (neu, ersetzt `SkillList.tsx`), `MonarchPage.tsx`, `actions.ts`; gemeinsam mit B-672 `playback/timeline.ts`, `effectView.ts`, kompakte `PlaybackControls`, `stage/figureLayer.ts` (aus `PreviewStage.ts` ausgelagert, 397 Zeilen), `styles/figures.css`.
- AC-01, AC-04 Browser-Pane 5196 `?mock=1` 1600×1000: eine Leiste (24 px), Bühne 1200×786 px. AC-02, AC-03, AC-05 bis AC-07 Vitest (`MonarchPage.test.tsx`, `actions.test.ts`, `timeline.test.ts`). AC-08 im Browser-Pane gesehen: `fireball` im Einzelbild als Ring an allen Monarchen; `check:diff` grün.
- Entschieden nach Default: Skills-Panel rechts; Effekt am Monarchen in 50 px Höhe wie `skillCasts.ts`. Lücke: Wirk-Animation des Monarchen (das Spiel spielt nur den Ton).
- Hinweis: Das Panel ist 480 px breit, bei offenem Panel rutscht der vierte Monarch aus dem Bild (Figuren werden nicht neu eingepasst).
- Offen: Abnahme durch 🧑 im echten Studio.
