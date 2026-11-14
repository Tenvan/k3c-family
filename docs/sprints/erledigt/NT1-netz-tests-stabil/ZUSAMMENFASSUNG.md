# NT1 · Zusammenfassung

Regelbasiert aus der Sprint-README erzeugt (`plan_digest`, B-696); maßgeblich bleibt [README.md](README.md).

- **Titel:** Stabile Tests, Warteschlange und Snapshot-Budget
- **Projekt:** LST

## Ziel

Die Prüfungen sind reproduzierbar grün und Netz-Nachrichten bleiben auch mit vielen Plätzen im Budget. Am Ende sichtbar: `task check:go` ohne Wiederholung grün, Warteschlange und Snapshot im Budget.

## Ergebnis

2026-10-25, Claude (NT1.3): `task check` und `task check:go` grün, ohne Wiederholung; `check:race` lokal übersprungen (kein C-Compiler), die CI prüft es. AC-01 bis AC-03 erfüllt durch NT1.1 (Ursachen behoben, Nachweis mit -count=20 bzw. 200 unter Last), AC-04 bis AC-06 durch NT1.2 (Warteschlange, `eventsDropped`, Messwert im Protokoll). Review (Opus): APPROVE, der LOW-Befund (wiederholte Warnung) ist behoben. Keine neuen Tickets.

## Kriterien

| Kriterium | Status | Inhalt |
|---|---|---|
| AC-01 | erfüllt | TestRestore läuft unter Windows auch in task check:all stabil grün (B-274/AC-01, B-274/AC- … |
| AC-02 | erfüllt | TestGleicherSeedGleicheEingaben scheitert nicht, wenn task check:go parallel läuft (B-284/ … |
| AC-03 | erfüllt | TestTickReiheJeRaum schlägt im Gesamtlauf gelegentlich fehl (B-286/AC-01, B-286/AC-02). |
| AC-04 | geprüft | Die Warteschlange einer Verbindung läuft nicht voll, wenn andere Nachrichten zwischen Zust … |
| AC-05 | geprüft | Der Client erfährt zuverlässig, wie viele Ereignisse verworfen wurden (B-190/AC-01). |
| AC-06 | geprüft | Der Welt-Snapshot bleibt mit 39 Plätzen je Stufe im Budget (B-263/AC-01). |

## Tickets

| Ticket | Status | Kriterien | Titel |
|---|---|---|---|
| B-274 | erledigt | AC-01 erfüllt | TestRestore läuft unter Windows auch in task check:all stabil grün |
| B-284 | erledigt | AC-02 erfüllt | TestGleicherSeedGleicheEingaben scheitert nicht, wenn task check:go parallel läu … |
| B-286 | erledigt | AC-03 erfüllt | TestTickReiheJeRaum schlägt im Gesamtlauf gelegentlich fehl |
| B-280 | erledigt | AC-04 geprüft | Die Warteschlange einer Verbindung läuft nicht voll, wenn andere Nachrichten zwi … |
| B-190 | erledigt | AC-05 geprüft | Der Client erfährt zuverlässig, wie viele Ereignisse verworfen wurden |
| B-263 | erledigt | AC-06 geprüft | Der Welt-Snapshot bleibt mit 39 Plätzen je Stufe im Budget |
