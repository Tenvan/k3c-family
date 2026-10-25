# NT1 · SRV · Stabile Tests, Warteschlange und Snapshot-Budget

- **Status:** erledigt
- **Projekt:** LST
- **Domäne:** SRV
- **Reife:** bereit
- **Tickets:** B-274, B-284, B-286, B-280, B-190, B-263
- **Start-Commit:** 819c53d3
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** autonom, Claude, 2026-10-25, Revision 1 (Ziel von Ralf: Specs genehmigt)

## Ausgangslage

Mehrere Go-Tests scheitern gelegentlich (B-274, B-284, B-286); die Warteschlange einer Verbindung kann volllaufen (B-280); verworfene Ereignisse kommen unzuverlässig an (B-190); der Snapshot mit 39 Plätzen je Stufe ist nicht gemessen (B-263).

## Ziel

Die Prüfungen sind reproduzierbar grün und Netz-Nachrichten bleiben auch mit vielen Plätzen im Budget. Am Ende sichtbar: `task check:go` ohne Wiederholung grün, Warteschlange und Snapshot im Budget.

## Beteiligte und Zielgruppen

Agenten und CI; 🧑 für Release-Läufe.

## Anforderungen

B-274 › Anforderungen; B-284 › Anforderungen; B-286 › Anforderungen; B-280 › Anforderungen; B-190 › Anforderungen; B-263 › Anforderungen.

## Nicht-Ziele

Neue Netz-Features, Protokoll-Umbau.

## Regeln und Einschränkungen

SRV; Tests nie mit `t.Skip` stilllegen, Ursache beheben.

## Beispiele

`task check:all` dreimal hintereinander unter Windows → jedes Mal grün.

## Ausnahme- und Fehlerfälle

Ursache liegt außerhalb SRV → Ticket, Test bleibt aktiv.

## Akzeptanzkriterien

- **AC-01** TestRestore läuft unter Windows auch in task check:all stabil grün (B-274/AC-01, B-274/AC-02).
- **AC-02** TestGleicherSeedGleicheEingaben scheitert nicht, wenn task check:go parallel läuft (B-284/AC-01).
- **AC-03** TestTickReiheJeRaum schlägt im Gesamtlauf gelegentlich fehl (B-286/AC-01, B-286/AC-02).
- **AC-04** Die Warteschlange einer Verbindung läuft nicht voll, wenn andere Nachrichten zwischen Zuständen stehen (B-280/AC-01).
- **AC-05** Der Client erfährt zuverlässig, wie viele Ereignisse verworfen wurden (B-190/AC-01).
- **AC-06** Der Welt-Snapshot bleibt mit 39 Plätzen je Stufe im Budget (B-263/AC-01).

## Offene Fragen

keine

## Sessions

| Nr. | Datei | Typ | Agent | Status |
|---|---|---|---|---|
| NT1.1 | `NT1.1-flakige-tests.md` | Umsetzung | autonom | fertig |
| NT1.2 | `NT1.2-warteschlange-budget.md` | Umsetzung | autonom | fertig |
| NT1.3 | `NT1.3-review.md` | Review | autonom | fertig |

Entwurf. Vor dem Aktivieren jede Session als Datei nach `docs/vorlagen/session.md` schreiben, die Kriterien in Klammern werden ihr Feld `Kriterien`.

- NT1.1 Flakige Tests: TestRestore, TestGleicherSeedGleicheEingaben, TestTickReiheJeRaum (AC-01, AC-02, AC-03).
- NT1.2 Warteschlange, verworfene Ereignisse, Snapshot-Budget (AC-04, AC-05, AC-06).
- NT1.3 Review (Code-Sprint): alle Kriterien prüfen.

## Abnahme

2026-10-25, Claude (NT1.3): `task check` und `task check:go` grün, ohne Wiederholung; `check:race` lokal übersprungen (kein C-Compiler), die CI prüft es.
AC-01 bis AC-03 erfüllt durch NT1.1 (Ursachen behoben, Nachweis mit -count=20 bzw. 200 unter Last), AC-04 bis AC-06 durch NT1.2 (Warteschlange, `eventsDropped`, Messwert im Protokoll).
Review (Opus): APPROVE, der LOW-Befund (wiederholte Warnung) ist behoben.
Keine neuen Tickets.
