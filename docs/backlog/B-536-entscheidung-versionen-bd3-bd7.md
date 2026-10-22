# B-536 · Entscheidung: Versionen v0.21.0 (BD3) und v0.21.1 (BD7) bestätigen

- **Domäne:** REG
- **Typ:** Frage
- **Prio:** hoch
- **Umgebung:** offline
- **Status:** eingeplant
- **Sprint:** KR1
- **Projekt:** BED
- **Erstellt:** 2026-10-22
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

BD3 (Raum-Pause, Protokoll v8) und BD7 (Pause-Client, Skill-Reset, Schutz wartender Monarchen) sind gemergt. Ihre Abnahmen schlagen Versionen vor (`docs/sprints/erledigt/BD3-raum-pause/README.md`: v0.21.0 Minor; `.../BD7-pause-client-skill-schutz/README.md`: v0.21.1 Patch). Letzter Tag ist v0.20.0. Tags setzt nur 🧑 (Release, Fast-Forward auf `main`).

## Ziel

🧑 bestätigt oder ändert die beiden Versionsnummern (BD6/AC-03).

## Beteiligte und Zielgruppen

🧑 entscheidet und taggt. Claude hat vorbereitet.

## Anforderungen

nicht relevant (Frage).

## Nicht-Ziele

Release-Durchführung selbst.

## Regeln und Einschränkungen

`main` ist geschützt, nur Releases durch 🧑.

## Beispiele

nicht relevant (Frage).

## Ausnahme- und Fehlerfälle

nicht relevant (Frage).

## Akzeptanzkriterien

- **AC-01** 🧑 hat unter Notizen die Version je Sprint bestätigt, geändert oder verworfen.

## Offene Fragen

Siehe Notizen.

## Notizen

**Frage:** Welche Versionen bekommen BD3 und BD7?

- **Default (Claude, autonom):** BD3 → v0.21.0 (neue Server-Funktion, Protokoll v8), BD7 → v0.21.1 (Patch). Alternative A: beide zusammen als ein Release v0.21.0. Alternative B: keine eigenen Tags, Versionen erst mit dem nächsten Release.
- **Bis zur Entscheidung:** kein Tag gesetzt, nichts weiter zu tun.
- **Wenn 🧑 anders entscheidet:** nur der Vermerk in den beiden Sprint-READMEs (Abnahme › Version) ändert sich; kein Code betroffen.
