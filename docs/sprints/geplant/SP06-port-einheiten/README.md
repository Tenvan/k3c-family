# SP06 · SIM · Port II – Gegner, Wellen, Reisen, Kampagne

- **Status:** geplant
- **Domäne:** SIM
- **Reife:** Entwurf
- **Einschiebbar:** nein
- **Tickets:** B-043
- **Start-Commit:** –
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

Nach SP05 fehlen in Go Gegner, Wellen, Projektile (auch das Schießen der Bogenschützen), Burg-Fall, Reisen und die Kampagne. Welt, Tag/Nacht, eigene Truppen und Wirtschaft kommen aus SP05.

## Ziel

Die Go-Simulation ist vollständig. Am Ende sichtbar: alle Golden-Läufe inkl. Nacht und Stufenwechsel grün.

## Beteiligte und Zielgruppen

Entwickler oder Agent.

## Anforderungen

B-043 › Anforderungen. Sprint-eigen: Gegner, Wellen, Projektile, Burg-Fall (dazu `applyDamage` und `destroySite` aus `common.ts`), Reisen, Kampagne und Spielstand in Go; Spielstand-Format mit Version.

## Nicht-Ziele

Räume und Netz (SP07).

## Regeln und Einschränkungen

Wie SP05; das Spielstand-Format ist ein Vertrag mit Version.

## Beispiele

Golden-Lauf mit Nacht und Stufenwechsel → identische Snapshots.

## Ausnahme- und Fehlerfälle

Alter Spielstand → bleibt lesbar.

## Akzeptanzkriterien

- **AC-01** Gegner, Wellen und Kampf: die Golden-Läufe mit Nacht sind grün.
- **AC-02** Reisen und Kampagne: die Golden-Läufe mit Stufenwechsel sind grün (B-043/AC-02).
- **AC-03** Das Spielstand-Format hat eine Version, alte Stände bleiben lesbar (Test).

## Offene Fragen

keine

## Sessions

Entwurf. Vor dem Aktivieren jede Session als Datei nach `docs/vorlagen/session.md` schreiben, die Kriterien in Klammern werden ihr Feld `Kriterien`.

- SP06.1 Gegner, Wellen, Projektile, Burg-Fall (AC-01).
- SP06.2 Reisen, Kampagne und Spielstand (Format mit Version, alte Stände lesbar) (AC-02, AC-03).
- SP06.3 🔍 Review (alle).

## Abnahme

–
