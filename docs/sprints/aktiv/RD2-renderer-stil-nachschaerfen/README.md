# RD2 · PAP · Renderer und Stil nach der Sichtprüfung: Schatten, Materialien, Sichtbarkeit, Rig-Proportionen

- **Status:** aktiv
- **Projekt:** PAP
- **Domäne:** PAP
- **Reife:** bereit
- **Tickets:** B-712, B-713, B-714, B-715
- **Start-Commit:** 8cd4c5be
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** Ralf, 2026-11-07, Chat (Revision 1)

## Ausgangslage

PS1 hat gezeigt: Papier-Look schwächer als im Prototyp, fehlende Materialien, keine Sichtbarkeit je Keyframe, Rigs ohne Proportionen je Figur, renderKey ohne Reittier.

## Ziel

Renderer und Stil liefern alles, was die Asset-Sprints MG9–MG11 brauchen.

## Beteiligte und Zielgruppen

Ralf (🧑) prüft die Sichtprüfung und gibt frei; Claude setzt um; Nutzer sind die Asset-Sprints MG9 bis MG11 und das Spiel.

## Anforderungen

B-712, B-713, B-714, B-715.

## Nicht-Ziele

Assets außer den in B-712 genannten Material-Umstellungen.

## Regeln und Einschränkungen

Welle 4, erster Sprint: vor MG9, MG10, MG11. Lanes `stil` und `renderer`.

## Beispiele

Monarch idle 4 neben dem Prototyp: gleich starke Schatten und Struktur. Pferd-Spec ändern: Monarch wird neu gerendert. Teil mit `sichtbar: false` fehlt im Frame. Elefant ohne `galopp` ist gültig.

## Ausnahme- und Fehlerfälle

Pferd ohne `galopp` ist ungültig. `sichtbar` fehlt im Keyframe: Teil bleibt wie im vorigen Keyframe. Ersatz-Material nach Umstellung noch benutzt: Test schlägt fehl.

## Akzeptanzkriterien

- **AC-01** B-712/AC-01 bis AC-04
- **AC-02** B-713/AC-01, AC-02
- **AC-03** B-714/AC-01 bis AC-03
- **AC-04** B-715/AC-01 bis AC-03

## Offene Fragen

keine

## Sessions

| Nr. | Datei | Typ | Agent | Status |
|---|---|---|---|---|
| RD2.1 | `RD2.1-stil-schatten.md` | Umsetzung | autonom | fertig |
| RD2.2 | `RD2.2-stil-materialien.md` | Umsetzung | autonom | fertig |
| RD2.3 | `RD2.3-renderer-sichtbar-renderkey.md` | Umsetzung | autonom | fertig |
| RD2.4 | `RD2.4-renderer-rig-proportionen.md` | Umsetzung | autonom | fertig |
| RD2.5 | `RD2.5-review.md` | Review | autonom | offen |

Reife Entwurf:

- RD2.1 Stil: Schatten wie Prototyp (B-712)
- RD2.2 Stil: neue Materialien und Umstellung (B-712)
- RD2.3 Renderer: renderKey, Keyframe `sichtbar` (B-713, B-714)
- RD2.4 Renderer: Rig-Proportionen (B-715)
- RD2.5 Review

## Abnahme

Wird von der Review-Session (Doku-Sprint: letzte Session) ausgefüllt, höchstens fünf Zeilen: Datum, Kriterien
(Verweis auf Session-Ergebnisse), behobene Befunde, neue Tickets. Bis dahin `–`.
