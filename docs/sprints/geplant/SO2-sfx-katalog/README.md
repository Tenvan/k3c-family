# SO2 · CLI · SFX-Katalog und Einbau

- **Status:** geplant
- **Domäne:** CLI
- **Reife:** Entwurf
- **Einschiebbar:** ja
- **Tickets:** B-167
- **Start-Commit:** –
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

Nach SO1 steht der Audio-Kern, es gibt aber keine Effekt-Sounds und keinen Katalog. Details in B-167.

## Ziel

Jedes wichtige Ereignis hat einen Sound mit Quelle und Lizenz, und die Sounds sind eingebaut. Am Ende sichtbar: `docs/assets/sounds.md`, Dateien mit Credits unter `public/audio/`, hörbare Effekte am TV.

## Beteiligte und Zielgruppen

🧑 wählt Quellen und Stil (Q15); der Agent katalogisiert und baut ein; Spieler am TV.

## Anforderungen

B-167 › Anforderungen.

## Nicht-Ziele

Audio-Kern (SO1), Musik (SO4), Hörprobenseite (SO3), eigene Kompositionen.

## Regeln und Einschränkungen

CC0 oder CC-BY mit Credits (B-165-Test); `src/scenes` rechnet nichts; 2 Spieler; Voraussetzung: SO1, F3, F4.

## Beispiele

Münze aufheben → Münz-Sound; Datei fehlt → Ereignis bleibt stumm.

## Ausnahme- und Fehlerfälle

Sound lädt nicht → stumm und Log-Eintrag, kein Absturz.

## Akzeptanzkriterien

- **AC-01** `docs/assets/sounds.md` enthält jedes Ereignis der Liste mit Sound, Quelle, Lizenz oder als Lücke mit Ticket (B-167/AC-01).
- **AC-02** Jede Sound-Datei unter `public/audio/` hat einen Credit-Eintrag (B-167/AC-02).
- **AC-03** Münze aufheben, Schlag, Gegner-Tod, Bauen fertig und Nacht naht lösen am TV je ihren Sound aus (B-167/AC-03).
- **AC-04** Das Mapping Ereignis → Sound ändert keinen Spielzustand (B-167/AC-04).
- **AC-05** Eine fehlende Sound-Datei lässt das Spiel ohne Fehlermeldung weiterlaufen (B-167/AC-05).
- **AC-06** `task check` ist grün.

## Offene Fragen

- Quellen, Klangstil und „wichtige“ Ereignisse: Entscheidet 🧑 (`docs/fragenkatalog.md` Q15); blockiert die Freigabe.

## Sessions

Entwurf. Vor dem Aktivieren jede Session als Datei nach `docs/vorlagen/session.md` schreiben, die Kriterien in Klammern werden ihr Feld `Kriterien`.

- SO2.1 🧑 Workshop (Agent: Mensch): Quellen, Stil und Ereignisliste bestätigen; Agent legt Katalog an (AC-01).
- SO2.2 Sounds beschaffen, nach `public/audio/` legen, Credits (AC-02).
- SO2.3 Einbau über den Audio-Kern, Rückfall bei fehlender Datei (AC-03, AC-04, AC-05).
- SO2.4 Review (AC-06).

## Abnahme

–
