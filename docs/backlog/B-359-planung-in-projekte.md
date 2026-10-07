# B-359 · Die offene Planung ist in Projekte umgezogen, erledigte und zusammengelegte Sprints sind abgeschlossen

- **Domäne:** INF
- **Typ:** Schuld
- **Prio:** hoch
- **Umgebung:** offline
- **Status:** eingeplant
- **Sprint:** PJ3
- **Erstellt:** 2026-10-07
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

Stand 2026-10-07: 11 aktive und 41 geplante Sprints ohne Projekt. Aktiv sind u. a. DBG3, LP1, LT1, MON2, RL1, SO1 und SO3, in denen nur noch Sessions mit `Agent: Mensch` offen sind (DBG3.4, LP1.4, LT1.3, MON2.4, RL1.2, SO1.5, SO3.3). Mehrere geplante Sprints überlappen (RG1/RG3, BAL5/BAL6, SO5/SO4, M10/DBG4). Tickets hängen an erledigten Sprints (B-090 U1, B-092 U3, B-208 W5, B-327 und B-328 K1) oder sind eigentlich Themen-Ziele (B-011 Sound, B-099 Balancing-Tester). `docs/plan-weiterentwicklung.md` § 11 (Bahnen, Wellen) ordnet die Arbeit parallel zur Planung.

## Ziel

Jeder offene Sprint und jedes offene Ticket gehört zu einem Projekt, `aktiv/` enthält nur Sprints mit offener autonomer Arbeit, und die Reihenfolge der Arbeit steht nur noch an einer Stelle (Projekt-Rang).

## Beteiligte und Zielgruppen

🧑 (Rangfolge und Zuschnitt entschieden 2026-10-07), Agenten beim Umzug über die `plan_*`-Tools.

## Anforderungen

Projekte anlegen und Sprints in dieser Reihenfolge zuordnen (Stand 2026-10-07, offene Sprints dieser Liste):

| Rang | Projekt | Sprints | Tickets ohne Sprint |
|---|---|---|---|
| 1 | LST Leistung & Stabilität | PM1 → PF1 → NT1 → ST1 | – |
| 2 | GRA Grafik | GR7 → M10 (mit DBG4) → neuer Sprint Figuren und Pipeline | B-329, B-331 |
| 3 | SND Sound (Ziel B-011) | SO2 → SO4 (mit SO5) | – |
| 4 | BED Bedienung & HUD | U5 → S8 → S9 → U6 → PL1 → LB1 | B-339, B-333, B-214 |
| 5 | WRT Wirtschaft & Spielstand | W6 → W7 → SV1 → W8 | B-342 |
| 6 | SKL Monarch & Skills | SK1 → RM1 (nur B-285) | – |
| 7 | KMP Kampf, Bosse, Events | K2 → K4 → K5 → K3 → LV1 | B-327, B-328, B-343, B-324 |
| 8 | WZ Werkzeuge & Testläufe | M11 → TR3 → PL2 | B-352, B-354, B-341 |
| 9 | REL Release & Betrieb | CI1 → RP1 → BT1 → PG1 → PB1 | – |
| ruht | BAL Balancing & Spieleabende (Ziel B-099) | RG1 (mit RG3) → BAL6 (mit BAL5) → RG2 → P1 → BAL4 → BR1 → BR2 | – |
| ohne Rang | ABN Abnahmen am Gerät | HW1 | – |

- Das Projekt `PRZ Arbeitsweise` mit den Sprints dieses Umbaus (PJ1 → PJ2 → PJ3) wird als erledigt geführt.
- **Schließen:** DBG3, LP1, LT1, MON2, RL1, SO1, SO3. Ihre offenen Mensch-Sessions werden `verworfen` und als Sessions in HW1 neu angelegt; die Abnahme des alten Sprints vermerkt das Kriterium als „angenommen, Validierung in HW1.x“.
- **Zusammenlegen:** RG3 in RG1, BAL5 in BAL6, SO5 in SO4, DBG4 in M10 (Tickets und Kriterien übernehmen, den leeren Sprint löschen).
- **RM1 teilen:** B-214 (Pause) geht ohne Sprint an BED, RM1 behält B-285.
- **Tickets bereinigen:** B-090, B-092, B-208 gegen den erledigten Sprint prüfen und archivieren oder einem Projekt geben; B-327, B-328 an KMP; B-011 und B-099 als `Ziel-Tickets` von SND bzw. BAL.
- `docs/sprints/README.md` nach Projekten gliedern; Abschnitt „Offen am Gerät“ verweist auf ABN.
- `docs/plan-weiterentwicklung.md` § 11 (Bahnen, Wellen, Spuren) durch einen Verweis auf `docs/projekte/README.md` ersetzen; die Zeile „Sprints“ in `CLAUDE.md` nennt Projekte und Rang.
- Strenge Prüfung in `tests/planning.test.ts` einschalten: jeder geplante und aktive Sprint und jedes offene Ticket hat ein Projekt.

## Nicht-Ziele

Inhalt der Sprints ändern, Sprints bereit machen oder freigeben, offene Reviews (K2.4, M11.2, TR3.2) durchführen, bestehende Sprints zu Domänen-übergreifenden umbauen (B-356 gilt erst beim nächsten Bereitmachen).

## Regeln und Einschränkungen

- Nur Planungsdateien (`docs/projekte/`, `docs/sprints/`, `docs/backlog/`, `docs/plan-weiterentwicklung.md`, `CLAUDE.md`) und die eine Zeile in `tests/planning.test.ts`, die die strenge Prüfung einschaltet.
- Alles über die `plan_*`-Tools (B-357); Handarbeit nur als Rückfall.
- Hängt von B-355, B-356, B-357 und B-338 ab.
- Zusammengelegte Sprints behalten Revision und Freigabe nicht: Spec zurück auf `Entwurf`, Revision + 1.

## Beispiele

- SO1: SO1.5 → `verworfen`, HW1 bekommt eine Session „Audio am TV (aus SO1.5)“; SO1 → `erledigt`, Ordner nach `erledigt/`.
- RG3 in RG1: B-346 bekommt `Sprint: RG1`, RG1 übernimmt das Kriterium, RG3 wird gelöscht.

## Ausnahme- und Fehlerfälle

- Ein Sprint der Liste wurde inzwischen abgeschlossen → nur ins Projekt eintragen, nichts schließen.
- Ein neuer Sprint oder ein neues Ticket ist seit 2026-10-07 dazugekommen → nach Thema zuordnen und im Ergebnis nennen; passt kein Projekt → Frage-Ticket, Sprint bleibt vorerst ohne Projekt (strenge Prüfung dann erst nach Klärung).

## Akzeptanzkriterien

- **AC-01** Die Projekte der Tabelle existieren mit Rang, Status und Sprints in der genannten Reihenfolge (`plan_list kind=projekt`).
- **AC-02** DBG3, LP1, LT1, MON2, RL1, SO1 und SO3 liegen in `docs/sprints/erledigt/`, ihre Gerät-Sessions stehen als Sessions in HW1.
- **AC-03** RG3, BAL5, SO5 und DBG4 sind in RG1, BAL6, SO4 bzw. M10 aufgegangen; B-214 hängt an BED ohne Sprint.
- **AC-04** B-090, B-092, B-208, B-327, B-328 sind archiviert oder einem Projekt zugeordnet; B-011 und B-099 sind Ziel-Tickets.
- **AC-05** `docs/sprints/README.md` ist nach Projekten gegliedert, § 11 in `docs/plan-weiterentwicklung.md` verweist auf `docs/projekte/README.md`, `CLAUDE.md` nennt Projekte.
- **AC-06** Die strenge Prüfung ist an und `task test -- planning` ist grün.

## Offene Fragen

keine (Zuschnitt und Rang von 🧑 am 2026-10-07 entschieden)

## Notizen

Links, Messwerte, verworfene Ansätze. Darf leer bleiben (`–`).
