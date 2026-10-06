# SK1 · SIM · Skill-Baum mit Tank und Zauberer, Respec-Regeln abfragbar

- **Status:** geplant
- **Domäne:** SIM
- **Prio:** hoch
- **Reife:** bereit
- **Einschiebbar:** nein
- **Tickets:** B-007, B-270, B-321
- **Start-Commit:** –
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

S1 (`docs/sprints/erledigt/S1-monarch-schlag-skills/`) liefert Fund-Pool, Verteilung, Tier-Gating, Respec und die aktiven Skills von Tank und Zauberer in `engine/sim/` samt Spielstand; S3 macht sie im Client bedienbar (Abnahme am Gerät S3.4 offen). Es fehlen ein Nachweis mit zwei Linien gleichzeitig (B-007) und eine Prüfung von Respec und Lernen ohne Seiteneffekt (B-270): `Respec` und `LearnSkill` in `engine/sim/monarch.go` ändern den Spieler sofort.

## Ziel

Tank und Zauberer lassen sich über den Baum lernen und umskillen, der Server fragt die Regeln seiteneffektfrei ab. Am Ende sichtbar: Skill-Baum spielbar, Respec und Lernen ohne Seiteneffekt prüfbar.

## Beteiligte und Zielgruppen

Spielende; Agent baut in `engine/sim/`.

## Anforderungen

B-007 › Anforderungen; B-270 › Anforderungen.

## Nicht-Ziele

Anzeige im Client (S3), Heiler und Dieb über B-007 hinaus.

## Regeln und Einschränkungen

SIM; Werte aus `data/`, keine Zahlen im Code. Golden-Daten bleiben unverändert.

Beschluss 🧑 2026-10-06 (Chat): Skill-Tasten (B-026) und Presets (B-017) sind erledigt; Grundlage `docs/rules/monarch.md`, S1 und S3. SK1 plant nur, was nach S1 fehlt.

Beschluss 🧑 2026-10-06 (Chat): Respec ohne gelernte Skills ist erlaubt, ohne Wirkung (wie heute).

## Beispiele

Respec am Tag an der Burg → Punkte frei, Abfrage ändert nichts.

## Ausnahme- und Fehlerfälle

Lernen ohne Punkte → abgelehnt 🚫, Zustand unverändert.

## Akzeptanzkriterien

- **AC-01** Skill-Baum mit Tank und Zauberer ist spielbar: ein Sim-Test zeigt Tank und Zauberer zweier Spieler gleichzeitig, für Client (S3) und Spielstand (S1) steht der Nachweis im Ergebnis (B-007/AC-01, B-007/AC-02, B-007/AC-03).
- **AC-02** Die Sim prüft Respec und Lernen ohne Seiteneffekt (B-270/AC-01).
- **AC-03** `task check:go` ist grün, Golden-Daten unverändert außer den begründeten neuen Ereignissen aus SK1.3 (B-321/AC-03).
- **AC-04** Schlag ohne Treffer und Skill ohne Ziel erzeugen ein Ereignis (B-321/AC-01, B-321/AC-02); Voraussetzung für S9.2.

## Offene Fragen

keine

## Sessions

| Nr. | Datei | Typ | Agent | Status |
|---|---|---|---|---|
| SK1.1 | `SK1.1-pruefungen-ohne-seiteneffekt.md` | Umsetzung | autonom | offen |
| SK1.2 | `SK1.2-skill-baum-nachweis.md` | Umsetzung | autonom | offen |
| SK1.3 | `SK1.3-ereignis-ohne-ziel.md` | Umsetzung | autonom | offen |
| SK1.4 | `SK1.4-review.md` | Review | autonom | offen |

## Abnahme

–
