# ALT · INF · Vorgeschichte vor der Sprint-Einteilung

- **Status:** erledigt
- **Domäne:** INF
- **Reife:** Entwurf
- **Einschiebbar:** nein
- **Tickets:** –
- **Start-Commit:** –
- **Spec:** rückwirkend
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

Vor SP00 gab es nur `docs/sessions.md` mit Phasen und die Sessions S1.1–S2.4.

## Ziel

Machbarkeitsnachweis im Browser: Vertical Slice „Ein Tag, eine Nacht“, Speichern, Truhen, Tiefen, Online-Modus.

## Beteiligte und Zielgruppen

Entwickler; die Familie als künftige Spieler.

## Anforderungen

nicht relevant – rückwirkend festgehaltener Stand, keine Anforderungen.

## Nicht-Ziele

Alles nach `df6e1de`.

## Regeln und Einschränkungen

nicht relevant – vor der heutigen Arbeitsweise entstanden.

## Beispiele

nicht relevant – rückwirkend.

## Ausnahme- und Fehlerfälle

nicht relevant – rückwirkend.

## Akzeptanzkriterien

- **AC-01** Im Browser sind ein Tag und eine Nacht zu zweit spielbar, mit Speichern, Truhen, Tiefen und Online-Modus (Stand `df6e1de`).

## Offene Fragen

keine

## Sessions

Die früheren Sessions S1.1–S1.11, S2.1, S2.2 und S2.4 (Details: `git log -- docs/sessions.md`). Wichtige Abweichungen vom alten Plan:

- **Eine Taste für alles (K2C):** A / Leertaste halten = im Takt Münzen an das nächste Ziel. Ohne Ziel fällt die Münze. X ist noch frei.
- Gold pro Spieler, Baumaterial gemeinsam im Hub. Bauplätze fest im Hub (`hub.json`).
- Start mit 1 Bauer + 2 Bogenschützen, sonst ist Nacht 1 nicht zu schaffen (Test „Balancing“).
- Spielstand: `src/world/sim/campaign.ts`, `server/saves.mjs`, Autosave bei Tagesanbruch und Stufenwechsel.
- Online-Modus (`?online=RAUM`), Touch-Steuerung, Figuren- und Reittier-Sprites kamen außerhalb des alten Plans dazu.
- Infrastruktur: CI (`ci.yml`), GitHub Pages (`deploy-pages.yml`), Release per Tag `v*` (`release.yml`).

## Abnahme

Kein Review nach heutiger Arbeitsweise. Der Stand ist die Ausgangsbasis für SP00 (`df6e1de`).
