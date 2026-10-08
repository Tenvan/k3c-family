# PJ3.3 · Fahrplan nach Projekten, § 11 und CLAUDE.md, strenge Prüfung

- **Status:** offen
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** INF
- **Umgebung:** live
- **Branch:** pj3/3-fahrplan-pruefung
- **Abhängig von:** PJ3.2
- **Tickets:** B-359
- **Kriterien:** AC-05, AC-06

## Ziel

Die Reihenfolge der Arbeit steht nur noch im Projekt-Rang: Der Fahrplan ordnet die Sprints nach Projekten, § 11 und `CLAUDE.md` verweisen auf `docs/projekte/README.md`, und `tests/planning.test.ts` verlangt für jeden geplanten und aktiven Sprint und jedes offene Ticket ein Projekt.

## Kontext

- Spec: B-359 › Anforderungen (Fahrplan, § 11, `CLAUDE.md`, strenge Prüfung). Stand nach PJ3.2: alle offenen Sprints und Tickets haben ein Projekt (sonst nennt das Frage-Ticket aus PJ3.1 die Ausnahmen).
- **Fahrplan** `docs/sprints/README.md`: k3c-dev pflegt ihn selbst (`syncRoadmap` in `tools/k3c-dev/internal/planning/tables.go`): Es sucht die Tabellen über die Überschriften „Aktiv“, „Geplant“, „Erledigt“ (`roadmapMarker`) und füllt Spalten nach Kopf (`Sprint`, `Domäne`, `Prio`, `Reife`, `Ordner`; andere Spalten behalten den alten Wert). Diese Überschriften und Tabellen bleiben deshalb. Gliederung nach Projekten = Spalte `Prio` durch `Projekt` ersetzen, Zeilen nach Projekt-Rang und Sprint-Tabelle des Projekts sortieren, Kopftext (Zeilen 6–8: Domänen-Regel, Prio, § 11) durch einen Verweis auf `../projekte/README.md` ersetzen. „Offen am Gerät“ verweist auf das Projekt ABN (HW1). Dass das Tool neue Zeilen mit `Projekt: –` anlegt, wird ein Ticket (SRV), keine Änderung hier.
- **§ 11** in `docs/plan-weiterentwicklung.md` (Zwei Spuren, Bahnen, Wellen) wird durch einen kurzen Verweis auf `docs/projekte/README.md` ersetzt. Ausnahmen: § 11.6 (angenommene Werte, verwiesen aus `docs/arbeitsweise.md` › Hardware entkoppelt) und § 11.5 (Golden-Bahn, verwiesen aus › Golden aktualisieren) bleiben als Abschnitte mit gleicher Nummer stehen, damit die Verweise gültig bleiben; `grep -n "§ 11" docs/ CLAUDE.md` vorher und nachher.
- **`CLAUDE.md`**, Zeile „Sprints“: nächste Session nach Projekt-Rang (`docs/projekte/README.md`), nicht mehr „höchste Prio“.
- **Strenge Prüfung:** `tests/planning.test.ts` hat die Übergangsregel „je Domäne ein aktiver Sprint (nur Sprints ohne Projekt)“ (Zeilen 87 ff.). Einschalten = ein Test, der für jeden Sprint in `aktiv/` und `geplant/` und jedes Ticket mit Status `offen`/`eingeplant` ein Projekt verlangt (`none()` aus `tests/planningDocs.ts`); die Übergangsregel entfällt dann (wird ohne Sprints ohne Projekt leer). Datei ≤ 400 Zeilen.
- Im Checkout der Repo-Wurzel arbeiten (B-275).

## Erlaubte Dateien

- `docs/sprints/README.md`, `docs/sprints/geplant/PJ3-planung-umziehen/` bzw. `aktiv/` (Status, Ergebnis)
- `docs/plan-weiterentwicklung.md` (§ 11)
- `CLAUDE.md` (Zeile „Sprints“)
- `tests/planning.test.ts` (strenge Prüfung, Übergangsregel)
- `docs/backlog/` (neue Tickets)

## Nicht-Ziele

k3c-dev ändern (Fahrplan-Spalte `Projekt` im Tool → Ticket SRV), Felder `Prio`/`Einschiebbar` aus Vorlagen und Sprints entfernen (B-361), `docs/arbeitsweise.md` umschreiben (nur falls ein Verweis auf § 11 bricht).

## Schritte

1. Fahrplan umbauen (Kontext › Fahrplan); danach mit einem `plan_set` ohne Wirkung (z. B. `Reife` auf den gleichen Wert) prüfen, dass das Tool den Fahrplan weiter findet.
2. § 11 ersetzen, Verweise prüfen.
3. `CLAUDE.md` Zeile „Sprints“ anpassen.
4. Strenge Prüfung in `tests/planning.test.ts` einbauen, Übergangsregel entfernen.
5. Ticket SRV für die Fahrplan-Spalte `Projekt` in k3c-dev anlegen.
6. `check_run task:check` grün, committen.

## Fertig, wenn

- [ ] AC-05: Fahrplan mit Spalte `Projekt` nach Rang sortiert und Verweis auf `docs/projekte/README.md`; § 11 verweist auf `docs/projekte/README.md` (11.5/11.6 bleiben); `CLAUDE.md` nennt Projekte und Rang.
- [ ] AC-06: Die strenge Prüfung ist im Test, `task test -- planning` ist grün; ein Probe-Sprint mit `Projekt: –` macht ihn rot (Probe danach verworfen).
- [ ] `task check` grün.

## Prüfen

```bash
task check
```

`check_run task:check` über k3c-dev. Keine manuellen Prüfungen.

## Ergebnis

Wird am Ende der Session ausgefüllt: Nachweis je Kriterium (`AC-01 geprüft: task check grün`,
`AC-02 verschoben: Grund, B-0NN`), wer manuell geprüft hat, Abweichungen vom Plan, neue Tickets. Bis dahin `–`.
