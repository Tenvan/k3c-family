# B-868 · Die Mindestversion von Task wird geprüft, auf die check_run beim Quoten von CLI_ARGS baut

- **Domäne:** DEV
- **Typ:** Schuld
- **Prio:** niedrig
- **Umgebung:** offline
- **Status:** erledigt
- **Sprint:** WZ55
- **Projekt:** WZG
- **Erstellt:** 2026-11-14
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** Klasse A, Claude, 2026-11-14
- **Duplikat von:** –
- **Ersetzt:** –

## Ausgangslage

WZ54.2: `check_run task:test` verlässt sich darauf, dass Task `CLI_ARGS` quotet; belegt für Task 3.54, `requirements.md` nennt 3.53, niemand prüft eine Mindestversion.

## Ziel

Die Workbench (bzw. `task install`) prüft die Task-Version und meldet zu alte Versionen; `requirements.md` nennt die geprüfte Mindestversion.

## Beteiligte und Zielgruppen

Claude (Umsetzung, Klasse A); 🧑 beim Einrichten.

## Anforderungen

- Mindestversion an einer Stelle, Prüfung vor `check_run`-Zielen mit Mustern.
- `requirements.md` nachgezogen.

## Nicht-Ziele

Task aktualisieren.

## Regeln und Einschränkungen

Klasse A (Schuld, DEV).

- **Bereich:** `tools/k3c-workbench/internal/mcpsrv/check_*.go`, `requirements.md`
- **Abhängig von:** WZ54
- **Braucht 🧑:** –
- **Größe:** S
- **Sperre:** exe

## Beispiele

Task 3.40 → `check_run task:test pattern=x` lehnt mit Hinweis auf die Mindestversion ab.

## Ausnahme- und Fehlerfälle

Version nicht lesbar → Warnung, kein Abbruch.

## Akzeptanzkriterien

- **AC-01** Test: zu alte Version wird abgelehnt, passende läuft; `requirements.md` nennt die Version.

## Offene Fragen

keine

## Notizen

Befund WZ54.2 (2026-11-14).
