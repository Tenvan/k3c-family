# B-871 · check_run fragt die Task-Version einmal ab und merkt sie sich

- **Domäne:** DEV
- **Typ:** Schuld
- **Prio:** niedrig
- **Umgebung:** offline
- **Status:** erledigt
- **Sprint:** WZ56
- **Projekt:** WZG
- **Erstellt:** 2026-11-14
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** Klasse A, Claude, 2026-11-14
- **Duplikat von:** –
- **Ersetzt:** –

## Ausgangslage

WZ55.2: `check_run` startet `task --version` bei jedem Lauf mit Muster neu (B-868), das Ergebnis wird nicht gemerkt.

## Ziel

Die Task-Version wird einmal je Workbench-Lauf abgefragt und gemerkt.

## Beteiligte und Zielgruppen

Claude (Umsetzung, Klasse A).

## Anforderungen

- Ergebnis zwischenspeichern (auch den Fehlerfall).
- Test: zwei Läufe, ein Aufruf.

## Nicht-Ziele

Neue Prüfungen.

## Regeln und Einschränkungen

Klasse A (Schuld, DEV).

- **Bereich:** `tools/k3c-workbench/internal/mcpsrv/check_taskversion*.go`
- **Abhängig von:** WZ55
- **Braucht 🧑:** –
- **Größe:** S
- **Sperre:** exe

## Beispiele

Zwei `check_run task:test pattern=x` → ein `task --version`.

## Ausnahme- und Fehlerfälle

Version nicht lesbar → Warnung einmal gemerkt.

## Akzeptanzkriterien

- **AC-01** Test zählt einen Versionsaufruf bei zwei Läufen.

## Offene Fragen

keine

## Notizen

Befund WZ55.2 (2026-11-14).
