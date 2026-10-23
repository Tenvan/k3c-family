# KR3.1 · Workshop: kritische Fragen zu GRA entscheiden (🧑)

- **Status:** offen
- **Typ:** Workshop
- **Agent:** Mensch
- **Domäne:** REG
- **Umgebung:** live
- **Branch:** kr3/1-fragen-gra
- **Abhängig von:** –
- **Tickets:** B-546, B-547, B-548, B-549, B-550, B-551, B-552, B-553
- **Kriterien:** AC-01, AC-02

## Ziel

Ralf hat G1–G8 (B-546 bis B-553) entschieden; bestätigte sind erledigt, geänderte haben Folgearbeit im Projekt GRA.

## Kontext

Tabelle in `docs/sprints/aktiv/KR3-fragen-gra/README.md` › Anforderungen, Details je Ticket unter Notizen. Dieselbe Durchsicht erfüllt GR10.1 (`docs/sprints/aktiv/GR10-entscheidungen-grafik/`); beide zusammen abschließen. G1, G2, G7 mit den Sessions GR7.3, GR7.4, GR9.5 (TV, Asset-Studio) ansehen, G4 auf `pipeline.html`.

## Erlaubte Dateien

- `docs/backlog/B-546*` … `B-553*`, `B-193*`, `B-251*`, `B-329*`, `B-331*`, `B-505*` (über `plan_*`)
- neue Tickets/Sprints im Projekt GRA (über `plan_create`)

## Nicht-Ziele

Umsetzung der Folgearbeit; die Geräte-Abnahmen selbst.

## Schritte

1. Ralf vermerkt je Ticket unter Notizen „bestätigt“ oder die Alternative.
2. Claude setzt bestätigte auf `erledigt`, plant geänderte als Ticket/Sprint im Projekt GRA.
3. GR10.1 und KR3.1 auf `fertig`.

## Fertig, wenn

- [ ] AC-01: Jedes Ticket trägt Ralfs Entscheidung.
- [ ] AC-02: Bestätigte erledigt, geänderte mit Folgearbeit.

## Prüfen

```bash
task test -- planning
```

## Ergebnis

Teilergebnis (Workshop 2026-10-23 mit Ralf, Claude moderiert und protokolliert); die Session bleibt `offen`, weil B-552 (G7) erst nach GR9.5 am TV entschieden wird.

- **AC-01 teilweise:** Sieben von acht Tickets tragen Ralfs Entscheidung; B-552 trägt „nach GR9.5“.
- **AC-02 teilweise:** Bestätigt und `erledigt`: B-548 (A), B-549 (A), B-550 (A), B-551 (A), B-553 (A, Freigabe in GR8, GR9, GR10 nachgetragen). Geändert und `erledigt` mit Verweis: B-546 (B, je Gegner ein Kandidat, siehe Ticket → Einbau in GR7.3 Schritt 4) und B-547 (B, ganzzahlig skalieren → GR7.4 Schritt 3). Offen: B-552 (`eingeplant`).
- **Neue Tickets:** keine; die Folgearbeit läuft in den vorhandenen Sessions GR7.3 und GR7.4.
- **Hinweis:** Reihenfolge GR7.3 vor GR7.4. B-331/AC-01 und AC-04 sind abgeschlossen (Notiz in B-331).
