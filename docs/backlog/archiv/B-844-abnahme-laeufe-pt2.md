# B-844 · Abnahme-Läufe PT2

- **Domäne:** PAP
- **Typ:** Schuld
- **Prio:** mittel
- **Umgebung:** live
- **Status:** erledigt
- **Sprint:** PT2
- **Projekt:** PAP
- **Erstellt:** 2026-11-13
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** Regel B-742 (Abnahme-Ticket)

## Ausgangslage

Sprint PT2 (Paper-CLI und Stil) ist abgeschlossen. Gen und Grafik-Golden liefen am Feature in PT2.2 (B-735, neue Materialien, Golden aller Gruppen neu); dieses Ticket wiederholt die Läufe nach Regel B-742 auf dem Stand nach dem Merge.

## Ziel

Die Abnahme-Läufe des Sprints PT2 sind grün.

## Beteiligte und Zielgruppen

Claude arbeitet autonom ab (nachts); 🧑 liest das Ergebnis.

## Anforderungen

Läufe laut `docs/arbeitsweise.md` › Sim, Balance und Gen, nur über `check_run`.

## Nicht-Ziele

Nachbessern im Ticket; Rot wird ein neues Ticket (Typ Problem) im Projekt PAP.

## Regeln und Einschränkungen

Sim und Balancing gesperrt (Beschluss 🧑 2026-11-11).

- **Bereich:** `public/grafik/**`, `tools/paper/golden/**`, `tests/paperAssets.test.ts`
- **Abhängig von:** PT2
- **Braucht 🧑:** –
- **Größe:** S
- **Sperre:** golden

## Beispiele

nicht relevant: reine Prüfläufe.

## Ausnahme- und Fehlerfälle

Lauf rot → neues Ticket Typ Problem im Projekt PAP, dieses Ticket mit Verweis erledigt.

## Akzeptanzkriterien

- **AC-01** Sim: verschoben (Sperre 2026-11-11); PT2 berührt die Simulation nicht.
- **AC-02** Balance: verschoben (Sperre 2026-11-11); PT2 berührt kein Balancing.
- **AC-03** Gen: `check_run task:test` (ruft `gen:paper`, Grafik-Golden `tests/paperAssets.test.ts`, `tools/paper/golden` samt `ersatz.test.ts` und `public.test.ts`, Sprite-Tests) grün; gewollte Golden-Änderung nur nach „Golden aktualisieren“.
- **AC-04** Race: nicht relevant, PT2 ändert keinen Go-Code.

## Offene Fragen

keine

## Notizen

Umnummeriert 2026-11-13 von B-838 (Kollision mit WZ44 in Welle 5b).

2026-11-14 (Claude, auf `sprint/welle6`): AC-01/AC-02 verschoben (Sperre). AC-03 `check_run task:test` grün (4 min 4 s, samt `gen:paper`, `tools/paper/golden`, `tests/paperAssets.test.ts`), Golden ohne Inhaltsänderung. AC-04 nicht relevant.
