# B-174 · Je Domäne darf ein Sprint aktiv sein, Sessions werden per Branch beansprucht

- **Domäne:** INF
- **Typ:** Idee
- **Prio:** hoch
- **Status:** erledigt
- **Sprint:** F0
- **Erstellt:** 2026-10-03
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** 2026-10-03, Chat (Ralf), mit Sprint F0

## Ausgangslage

`docs/arbeitsweise.md` › Sprint-Lebenslauf erlaubt höchstens **einen** aktiven Sprint (plus einschiebbare), `tests/planning.test.ts` prüft das (Test „höchstens ein aktiver Sprint“). 🧑 will autonome Sessions auf einem zweiten Account parallel zu den eigenen Workshops und Abnahmen laufen lassen. Jeder Sprint ändert nur Dateien seiner Domäne (`docs/arbeitsweise.md` › Domänen), Sprints verschiedener Domänen kollidieren deshalb nicht in den Dateien. Aktuell ist SP11 (SRV) aktiv und wartet auf 🧑 (Pi).

## Ziel

Mehrere Sprints sind gleichzeitig aktiv, höchstens einer je Domäne; zwei Agenten (oder Accounts) greifen nicht dieselbe Session an. Nutzen: Autonome Arbeit in mehreren Domänen läuft parallel, ohne dass ein wartender Mensch-Sprint (SP11) alles blockiert.

## Beteiligte und Zielgruppen

🧑 und Agenten (auch auf einem zweiten Account); Domäne INF (`docs/arbeitsweise.md`, `tests/planning.test.ts`).

## Anforderungen

- Regel in `docs/arbeitsweise.md` (Sprint-Lebenslauf, Abschnitt „Autonomer Ablauf“): Höchstens ein aktiver Sprint **je Domäne**; einschiebbare Sprints (`Einschiebbar: ja`) zählen wie bisher nicht mit. Abhängigkeiten zwischen Sprints stehen im Fahrplan und gelten weiter (ein Sprint startet erst, wenn seine Voraussetzungen erledigt sind).
- Beanspruchen einer Session: Vor dem Start prüft der Agent mit `git ls-remote --heads origin <Branch>`, ob der Branch aus dem Feld `Branch` schon existiert; wenn ja, ist die Session vergeben, der Agent nimmt die nächste. Er legt den Branch an und schiebt ihn vor der Arbeit (`git push -u origin <Branch>`).
- Gemeinsame Planungsdateien (`docs/backlog/README.md`, `docs/sprints/README.md`, `docs/roadmap.md`) ändert eine Session erst am Ende und führt vor dem PR `git fetch` und `git rebase origin/main` aus.
- `tests/planning.test.ts` prüft „höchstens ein aktiver Sprint je Domäne“ statt „höchstens ein aktiver Sprint“.

## Nicht-Ziele

Automatisches Verteilen von Sessions auf Agenten, Sperrdateien, Änderungen an der Sprint-Vorlage.

## Regeln und Einschränkungen

Domäne INF; Datei ≤ 400 Zeilen, Funktion ≤ 60. Die Regel ändert den Prozess und braucht die Freigabe von 🧑. Das Ändern von `docs/arbeitsweise.md` gehört laut Domänentabelle zu INF.

## Beispiele

SP11 (SRV) wartet auf den Pi, F1 (REG), F2 (INF) und F3 (SIM) laufen gleichzeitig → Test grün. Zwei Sprints der Domäne SIM aktiv → Test rot.

## Ausnahme- und Fehlerfälle

Ein Branch existiert, die Session ist aber liegengeblieben → 🧑 löscht den Branch oder gibt die Session ausdrücklich frei. Zwei Sprints derselben Domäne wären nötig → der zweite wartet im Fahrplan.

## Akzeptanzkriterien

- **AC-01** `docs/arbeitsweise.md` nennt die Regel „je Domäne ein aktiver Sprint“, das Beanspruchen per Branch und das Rebase vor dem PR (Sichtprüfung; kein Widerspruch zu den übrigen Abschnitten).
- **AC-02** Test: `tests/planning.test.ts` besteht mit zwei aktiven Sprints verschiedener Domänen und scheitert mit zwei nicht einschiebbaren aktiven Sprints derselben Domäne (Beleg über einen kurzen Probelauf mit temporären Ordnern oder Dateien, die Ergebnisse stehen im Session-Ergebnis).
- **AC-03** `task check` ist grün.

## Offene Fragen

keine

## Notizen

Aus der Planung der zwei Spuren (Plan § 11, Entscheidung 2026-10-03).
