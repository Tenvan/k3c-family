# DL1 · SRV · Delta überträgt verschwundene Felder

- **Status:** erledigt
- **Domäne:** SRV
- **Prio:** hoch
- **Reife:** bereit
- **Einschiebbar:** ja
- **Tickets:** B-297
- **Start-Commit:** ada3483
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** 2026-10-05, Chat (Ralf: „B-296: Option (a), erst SRV-Session fürs Delta“), Revision 1, aus dem Auftrag abgeleitet; umfasst B-297

## Ausgangslage

Das Delta kann ein Feld, das aus dem Zustand verschwindet, nicht entfernen (`merchant`, `storms`, künftig `drops`), und
der Delta-Test verliert mit der Verlust-Kaskade aus W4.3a seinen Burgschaden. W4.3a ist deshalb blockiert (B-296 auf
`sprint/w4`); 🧑 hat entschieden, zuerst diese SRV-Arbeit zu machen.

## Ziel

Nach diesem Sprint entfernt der Client jedes Feld, das der Server nicht mehr schickt, und W4.3a kann weiterlaufen.
Am Ende sichtbar: `task check:go` grün, auch mit dem Stand von `wip/w4.3a-krieger`.

## Beteiligte und Zielgruppen

Entwickler SIM (W4.3a), SRV und CLI; Spieler sehen keinen Geister-Händler. 🧑 merged.

## Anforderungen

B-297 › Anforderungen.

## Nicht-Ziele

Verlust-Kaskade (W4.3a), Anzeige von Händler und Ausrüstung (W6), neue Nachrichten.

## Regeln und Einschränkungen

Protokoll-Änderung: eine Session für Protokoll und beide Enden (`docs/arbeitsweise.md` › Domänen). Einschiebbar, damit
LT1 (nur noch Hardware offen) die Bahn nicht sperrt. Datei ≤ 400 Zeilen, Funktion ≤ 60.

## Beispiele

B-297 › Beispiele.

## Ausnahme- und Fehlerfälle

B-297 › Ausnahme- und Fehlerfälle.

## Akzeptanzkriterien

- **AC-01** Server nennt verschwundene Felder in `unset`, `apply` ergibt genau den neuen Zustand (B-297/AC-01).
- **AC-02** Client entfernt die Felder aus `unset` (B-297/AC-02).
- **AC-03** Delta-Test prüft `castle` weiter, auch mit dem Stand von W4.3a (B-297/AC-03).
- **AC-04** Protokoll beschrieben, `task check` und `task check:go` grün (B-297/AC-04).

## Offene Fragen

keine

## Sessions

| Nr. | Datei | Typ | Agent | Status |
|---|---|---|---|---|
| DL1.1 | `DL1.1-unset.md` | Umsetzung | autonom | fertig |
| DL1.2 | `DL1.2-review.md` | Review | autonom | fertig |

Bei `Reife: Entwurf` genügen Stichpunkte mit den Kriterien in Klammern statt Session-Dateien. Vor dem Aktivieren
wird jede Session als eigene Datei nach `docs/vorlagen/session.md` geschrieben.

## Abnahme

2026-10-05, Review DL1.2 (leicht, nur schwere Befunde): keine schweren Befunde. tsc, oxlint (0 Fehler), Vitest (1337), `go test`, golangci-lint (0 issues) grün.
`null` bleibt Wert, `events` nie in `unset`, Server und Client entfernen dieselben Felder; AC-01 bis AC-04 mit Tests und Protokolltext belegt (`delta_test.go`, `clientDelta.test.ts`, `docs/protocol.md`).
AC-03: Der Test trifft die Burg selbst alle 600 Ticks, weil sie mit W4.3a in keinem Golden-Lauf fällt; `castle` läuft weiter über `deltaOf` und `apply`. B-297 archiviert, W4.3a ist entsperrt.
Version: v0.12.1 vorgeschlagen (Fehlerkorrektur am Protokoll, Patch nach dem vorgeschlagenen v0.12.0 aus S6; aktuell v0.11.0)
