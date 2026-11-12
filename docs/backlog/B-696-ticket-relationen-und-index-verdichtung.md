# B-696 · Tickets verweisen auf Duplikate und Ersatz, der Index und die Archive bleiben klein

- **Domäne:** DEV
- **Typ:** Idee
- **Prio:** mittel
- **Umgebung:** offline
- **Status:** eingeplant
- **Sprint:** WZ47
- **Projekt:** WZG
- **Erstellt:** 2026-11-05
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** autonom, Claude, 2026-11-12

## Ausgangslage

Der Planer von k3c-workbench (`tools/k3c-workbench/internal/planning/`, MCP-Tools `plan_*`) pflegt Tickets als Markdown mit Kopf-Feldern.
`Status: verworfen` und `erledigt` gibt es, aber ohne Verweis: Es ist nicht erkennbar, wodurch ein Ticket ersetzt wurde oder wessen Duplikat es ist.
Wer ein neues Ticket anlegt, wird nicht auf ähnliche offene oder archivierte Tickets hingewiesen (`plan_create` → `createTicket` in `create.go` prüft nur Nummer, Felder und Projekt).

Zugleich wächst der Kontext, den jede Session lädt:

- `docs/backlog/README.md` hat etwa 108 KB. Der Abschnitt „## Archiv“ führt alle 548 erledigten und verworfenen Tickets als Zeilen, obwohl die Regel „nur auf Nachfrage lesen“ gilt (`syncIndex` in `tables.go` trägt beim Archivieren die Zeile in diesen Abschnitt).
- `docs/backlog/archiv/` hat 2,9 MB, `docs/sprints/erledigt/` 7,9 MB. Zu erledigten Sprints gibt es keine Kurzfassung.
- Im Abschnitt „## Offen“ steht heute eine erledigte Zeile (B-677); beim Umbau mitprüfen.

Anlass: Vergleich mit dem Open-Source-Projekt Beads (gastownhall/beads). Es ersetzt den Planer nicht und wird **nicht** eingeführt; nur drei Ideen werden im vorhandenen Planer nachgebaut: `supersedes`, `duplicates` und „memory decay“ (Verdichtung alter Einträge).

Stand der Technik im Code (geprüft):

- `edit.go`: `Set` → `setFields` prüft jedes Feld mit `checkValue` gegen die Vorlage (`templateFields`), danach `follow` → `followTicket` verschiebt bei `erledigt`/`verworfen` nach `backlog/archiv/` und zieht über `syncIndex` die Index-Zeile nach. Ein Feld, das in der Datei fehlt, ist heute ein Fehler („das Feld … fehlt in der Datei“).
- `create.go`: `createTicket` füllt die Vorlage über `ticketkit.Draft` (gemeinsamer Ticket-Kern `tools/assetkit/ticket`, B-595) und trägt die Zeile mit `ticketkit.AddToIndex` ein.
- `list.go`: `archived()` liest `backlog/archiv/B-*.md` selbst, nicht den Index. Ein verlagerter Archiv-Index berührt das nicht.
- `tests/planningDocs.ts` › `checkTemplate` verlangt, dass **jedes** Ticket exakt die Feldnamen der Vorlage in gleicher Reihenfolge hat. Neue Pflicht-Felder in `docs/vorlagen/ticket.md` machen damit alle ~640 Altdateien rot.
- `tests/planning.test.ts` › „Index listet jedes Ticket genau einmal“ liest nur `backlog/README.md` und erwartet dort alle Tickets samt Archiv.

## Ziel

Drei aufeinander aufbauende Teile; Reihenfolge A, dann B, dann C.

**Teil A · Relationen (zuerst).** Ein Ticket kann sagen, wessen Duplikat es ist und was es ersetzt. Agenten und 🧑 sehen beim Lesen eines Tickets sofort die Verweise in beide Richtungen, und beim Anlegen kommt ein Hinweis auf ähnliche Tickets.

**Teil B · Index-Verdichtung.** `docs/backlog/README.md` enthält nur offene und eingeplante Tickets und bleibt dadurch klein. Die Zeilen erledigter und verworfener Tickets liegen in `docs/backlog/archiv/README.md`.

**Teil C · Sprint-Zusammenfassungen (optional, zuletzt).** Zu jedem erledigten Sprint gibt es eine kurze `ZUSAMMENFASSUNG.md`; eine Session kann sie statt der vollen Sprint-Ordner lesen. Nichts wird gelöscht.

## Beteiligte und Zielgruppen

- 🧑 (Ralf): entscheidet die offenen Fragen, gibt die Spec frei und nimmt ab.
- Agenten-Sessions: lesen Tickets und Index, legen Tickets an, verwerfen Duplikate.
- Entwicklung: Workbench-Planungscode, Ticket-Kern `assetkit/ticket`, Tests in `tests/` und `internal/planning`.

## Anforderungen

**Teil A**

- Zwei neue optionale Kopf-Felder in `docs/vorlagen/ticket.md`: `Duplikat von:` (genau eine ID oder `–`) und `Ersetzt:` (Liste von IDs, kommagetrennt, oder `–`). Standard `–`.
- Gespeichert wird nur **eine** Richtung (im neueren Ticket). Die Gegenrichtung („ersetzt durch“, „Duplikate“) berechnet `plan_get` beim Lesen aus allen Tickets und gibt sie als Zeilen unter dem Kopf aus; die Datei bleibt unverändert.
- `plan_set` (und `plan_create` mit `fields`) validiert beide Felder: jede ID existiert (offen oder Archiv), kein Selbstverweis, kein Zyklus (A ersetzt B, B ersetzt A, auch über mehrere Schritte), Format `B-\d{3}`.
- `Duplikat von: B-xxx` setzt automatisch `Status: verworfen`; das Ticket wandert wie bisher ins Archiv (`followTicket`). Ein Duplikat eines Tickets, das selbst verworfen oder ersetzt ist, zeigt auf die Wurzel der Kette (nur Hinweis, keine Pflicht).
- `plan_create` prüft vor dem Anlegen den Titel gegen den Index (offen und archiviert) per Wortüberlappung (Kleinschreibung, Stoppwörter und Wörter unter 4 Zeichen weg, Jaccard oder Anteil gemeinsamer Wörter, fester Schwellwert) und gibt bis zu fünf Treffer als Warnung in den Hinweisen zurück. Ohne KI, deterministisch. Die Warnung **blockiert nie**.
- Abwärtskompatibel: Altdateien ohne die neuen Felder bleiben gültig; fehlendes Feld gilt als `–`; keine Migration der ~640 Dateien. `plan_set` darf das Feld in einem Altticket neu einfügen (heute lehnt `setFields` fehlende Felder ab).
- Die neuen Felder erscheinen auch in `plan_list` nur dort, wo sie gesetzt sind (z. B. Zusatz „→ B-xxx“ bei Duplikaten), nicht als neue Spalte im Index.

**Teil B**

- `docs/backlog/README.md` führt nur Tickets mit Status `offen` oder `eingeplant`. Der Abschnitt „## Archiv“ wird durch einen Verweis auf `archiv/README.md` ersetzt.
- `docs/backlog/archiv/README.md` enthält die Zeilen aller erledigten und verworfenen Tickets in derselben Tabellenform; Links ohne `archiv/`-Präfix.
- `followTicket`/`syncIndex`, `createTicket` und der Ticket-Kern (`ticketkit.AddToIndex`, `IndexRow`) schreiben beim Status-Wechsel in die richtige Datei. Studio und Workbench nutzen denselben Kern (Ticket-Button, B-595) und dürfen nicht abweichen.
- Die Regel „die Archiv-Datei bleibt beim Verschieben unverändert“ gilt weiter. Es werden keine Ticket-Dateien geändert oder gelöscht.
- Die Tests aus `tests/planning.test.ts` (Index vollständig, Status gleich, Pfad gleich) prüfen beide Index-Dateien zusammen.
- `docs/arbeitsweise.md` und `CLAUDE.md` (Zeile „Setzt eine Session ein Ticket auf erledigt …“) nennen die neue Ablage.

**Teil C (optional)**

- Neues Tool `plan_digest` mit den Parametern `älter_als` (Datum oder Anzahl Sprints) und `dry` (nur anzeigen, nichts schreiben; Standard `true`).
- Je erledigtem Sprint entsteht `docs/sprints/erledigt/<Sprint>/ZUSAMMENFASSUNG.md`: Ziel (gekürzt), Ergebnis (gekürzt), Tickets mit Status und AC-Status. Regelbasiert aus den Abschnitten „Ziel“ und „Ergebnis“ der Sprint-README, feste maximale Länge. Eine KI-Fassung ist ein getrennter, geprüfter späterer Schritt (eigenes Ticket).
- Ein erneuter Lauf verändert vorhandene Zusammenfassungen nicht, außer mit ausdrücklichem Parameter.
- Die Originale bleiben; `ZUSAMMENFASSUNG.md` ist eine zusätzliche Datei und gilt nicht als Sprint-Dokument nach Vorlage (Tests und `Load` müssen sie überspringen).

## Nicht-Ziele

- Beads oder Dolt einführen; keine zweite Planungsablage.
- KI-Zusammenfassungen im ersten Wurf (Teil C bleibt regelbasiert).
- Dateien löschen (Tickets, Sprints, Sessions); verdichtet wird nur durch zusätzliche Dateien und durch Verlagern von Index-Zeilen.
- Änderung der Session-, Sprint- oder Rang-Logik (`next.go`, `plan_next`, Projekte).
- Rückwirkendes Befüllen der neuen Felder für Altdateien (kann später ein eigenes Ticket sein, z. B. echte Duplikate unter den 548 Archivierten verknüpfen).
- Automatisches Zusammenlegen oder Verwerfen aufgrund der Ähnlichkeitswarnung.

## Regeln und Einschränkungen

- Planung nur über k3c-workbench (`CLAUDE.md`, B-210); Vorlagen sind Pflicht, `tests/planning.test.ts` prüft sie.
- Go-Code in `tools/k3c-workbench` und `tools/assetkit`: Datei ≤ 400 Zeilen, Funktion ≤ 60 Zeilen, Namen englisch (Komplexitäts-Budget laut `docs/arbeitsweise.md`); Logging mit Emoji.
- Domäne DEV: Review vor dem Commit nur bei Risiko (ab etwa 150 Code-Zeilen oder Pfad-/Eingabe-Code); `task check:diff` einmal vor dem Commit.
- Schreibzugriffe laufen über den `changeSet` (atomar, CRLF der Vorlage bleibt erhalten); Workbench teilt den Ticket-Kern (`assetkit/ticket`, eigenes Go-Modul). Das Asset-Studio ist entfallen (Entscheidung 005, WB1).
- Kein globaler Zustand, deterministisch, auf Windows lauffähig.

- **Bereich:** `tools/k3c-workbench/internal/planning/**`, `tools/k3c-workbench/internal/mcpsrv/tools_planning*.go`, `tools/k3c-workbench/app_ticket*.go`, `tools/assetkit/ticket/**`, `tests/planning.test.ts`, `tests/planningDocs.ts`, `docs/vorlagen/ticket.md`, `docs/backlog/archiv/README.md`, `docs/arbeitsweise.md`, `CLAUDE.md`
- **Abhängig von:** B-783
- **Braucht 🧑:** –
- **Größe:** L
- **Sperre:** exe

Defaults zu den Offenen Fragen (Claude, 2026-11-12, Meldung im Nachgang): 1 drei Sessions A → B → C in Sprint WZ47; 2 keine nachträgliche Verknüpfung; 3 Stoppwörter weg, Mindestlänge 4, Schwelle 0,4, höchstens fünf Treffer; 4 Felder optional am Ende des Kopfs; 5 nicht Teil des Tickets. Größe L ist in WZ47 in drei Sessions je Teil zerlegt.

## Beispiele

- B-700 (neu) „Ersetzt: B-210“ → `plan_get B-210` zeigt zusätzlich „ersetzt durch B-700“; B-210 selbst ist unverändert.
- `plan_set B-701 {"Duplikat von": "B-650"}` → B-701 hat Status `verworfen`, liegt in `backlog/archiv/`, Index-Zeile wandert in `archiv/README.md`; `plan_get B-650` zeigt „Duplikate: B-701“.
- `plan_create` mit dem Titel „Balancing-Seite zeigt eine Liste mit Varianten“ → Hinweis „ähnlich: B-xxx (offen), B-yyy (archiv)“, das Ticket wird trotzdem angelegt.
- Größe: `docs/backlog/README.md` nach Teil B unter 30 KB statt etwa 108 KB (Ziel: nur Kopf + offene und eingeplante Zeilen, aktuell unter 100 Zeilen).
- Teil C: `plan_digest {älter_als: "SP10", dry: true}` listet die Sprints, für die eine Zusammenfassung entstünde; ohne `dry` werden die Dateien geschrieben.

## Ausnahme- und Fehlerfälle

- Zielticket unbekannt oder Tippfehler → Fehler mit Liste ähnlicher IDs, nichts geändert.
- Selbstverweis (`Ersetzt: B-696` in B-696) oder Zyklus → Fehler, nichts geändert.
- `Duplikat von` bei einem Ticket, das schon `erledigt` ist → Fehler (ein erledigtes Ticket ist kein Duplikat; 🧑 entscheidet).
- Mehrere Werte bei `Duplikat von` → Fehler (genau eine ID); bei `Ersetzt` sind mehrere erlaubt.
- Wert mit `|` oder Zeilenumbruch → Fehler wie bei jedem Feld (`plainValue`).
- Ähnlichkeitsprüfung findet nichts, bricht ab oder der Index ist unlesbar → `plan_create` legt das Ticket trotzdem an, die Warnung entfällt mit Hinweis.
- Altticket ohne das Feld → `plan_get` und `plan_list` behandeln es als `–`; `plan_set` fügt das Feld an der Stelle laut Vorlage ein.
- `archiv/README.md` fehlt beim ersten Status-Wechsel nach dem Umbau → wird aus der Vorlage des Index angelegt, kein Fehler.
- Teil C: Sprint ohne Ziel oder Ergebnis → Zusammenfassung mit Platzhalter „–“, Hinweis im Ergebnis; schreibgeschützte oder fehlende Dateien → Lauf bricht ab, nichts halb geschrieben.

## Akzeptanzkriterien

Teil A

- **AC-01** `docs/vorlagen/ticket.md` enthält `Duplikat von:` und `Ersetzt:` mit Standard `–`; `tests/planningDocs.ts` › `checkTemplate` akzeptiert Tickets mit und ohne diese Felder (Altdateien unverändert grün); `task test -- planning` ist grün.
- **AC-02** Go-Test in `internal/planning`: `Set` mit `Ersetzt` oder `Duplikat von` lehnt unbekannte ID, Selbstverweis, Zyklus (direkt und über zwei Schritte) und mehrere Werte bei `Duplikat von` ab und ändert dann keine Datei.
- **AC-03** Go-Test: `Set` mit `Duplikat von: B-xxx` setzt `Status: verworfen`, verschiebt die Datei nach `backlog/archiv/` und zieht den Index nach.
- **AC-04** Go-Test: `Get` zeigt die Gegenrichtung („ersetzt durch“, „Duplikate“) berechnet; die Datei des Gegenstücks ist danach byte-gleich zu vorher.
- **AC-05** Go-Test: `Set` auf einem Altticket ohne die Felder fügt sie an der Vorlagenposition ein; Zeilenenden (CRLF) bleiben.
- **AC-06** Go-Test: `Create` mit einem Titel, der einem offenen und einem archivierten Ticket ähnelt, legt das Ticket an und nennt beide in den Hinweisen; ein unähnlicher Titel erzeugt keine Warnung; ein Fehler der Prüfung verhindert das Anlegen nie.

Teil B

- **AC-07** Nach der Umstellung enthält `docs/backlog/README.md` nur Zeilen mit Status `offen` oder `eingeplant` und ist kleiner als 30 KB (`wc -c`); `docs/backlog/archiv/README.md` enthält alle Zeilen erledigter und verworfener Tickets (Anzahl gleich der Anzahl `archiv/B-*.md`).
- **AC-08** `tests/planning.test.ts` prüft Index-Zeilen aus beiden Dateien: jedes Ticket genau einmal, Status und Pfad stimmen; `task test -- planning` ist grün.
- **AC-09** Go-Test: Status-Wechsel `offen → erledigt` und `erledigt → offen` verschiebt Datei und Zeile zwischen den beiden Index-Dateien; der Ticket-Text bleibt byte-gleich (außer den Kopf-Feldern, die der Wechsel ändert). Dasselbe über Studio/Workbench-Ticket-Button (`app_ticket_test.go` in beiden Modulen) grün.
- **AC-10** `docs/arbeitsweise.md` und `CLAUDE.md` nennen die neue Ablage; `git diff --stat` des Umbaus zeigt keine gelöschten Ticket-Dateien.

Teil C (nur wenn freigegeben)

- **AC-11** `plan_digest` mit `dry: true` schreibt nichts (Dateibaum vorher und nachher gleich) und listet die betroffenen Sprints.
- **AC-12** Ohne `dry` entsteht je erledigtem Sprint eine `ZUSAMMENFASSUNG.md` mit Ziel, Ergebnis und Ticket-Tabelle samt AC-Status, höchstens 60 Zeilen; Originale unverändert; ein zweiter Lauf ändert nichts.
- **AC-13** `Load`, `plan_list` und die Vitest-Tests überspringen `ZUSAMMENFASSUNG.md` (kein Fehlschlag „Datei ohne Session/Vorlage“).

## Offene Fragen

1. **Aufteilung (Empfehlung Claude):** Drei Sessions in **einem** Sprint, Reihenfolge A → B → C, weil alle am selben Code hängen (`edit.go`, `tables.go`, `assetkit/ticket`, `tests/planning*.ts`) und B den Index-Test aus A mitzieht. A und B jeweils eigener Commit, damit B einzeln zurückgenommen werden kann. Teil C als Ticket abtrennen, falls 🧑 ihn nicht sofort will, weil er unabhängig ist und weniger dringend. Entscheidet 🧑.
2. **Altlasten verknüpfen:** Sollen nach Teil A echte Duplikate unter den 548 Archivierten nachträglich verknüpft werden (eigenes Ticket, z. B. mit der Ähnlichkeitsliste als Vorschlag)? Entscheidet 🧑.
3. **Schwellwert und Treffer:** Wortüberlappung hat Fehltreffer (kurze Titel, gleiche Fachwörter wie „Studio“, „Balancing“). Vorschlag: Stoppwörter weg, Mindestlänge 4, Schwelle etwa 0,4, höchstens fünf Treffer; per Test an echten Titelpaaren kalibrieren. Entscheidet 🧑 nach Sicht der Trefferliste.
4. **Vorlage ändern ohne Migration:** Die Prüfung `checkTemplate` verlangt heute exakt die Feldliste der Vorlage. Empfehlung: die neuen Felder als „optional, ans Ende des Kopfs“ führen und `checkTemplate` so ändern, dass fehlende optionale Felder erlaubt sind. Alternative: Relationen als Abschnitt „## Verknüpfungen“ statt Kopf-Feldern (gleiches Problem bei Überschriften-Prüfung). Entscheidet 🧑.
5. **Archiv-Index für Sprints:** Soll der Fahrplan `docs/sprints/README.md` später ebenfalls erledigte Sprints auslagern? Hier nicht Teil des Tickets.

## Notizen

- Verwandte Tickets (alle erledigt, Archiv): B-210 (Planung über MCP-Tools), B-357 (plan-Tools für Projekte), B-387 (eindeutige Ticket-Nummern), B-518 (`plan_sections`), B-595 (Ticket-Button und gemeinsamer Ticket-Kern). Ein offenes Ticket zum Thema (Duplikat/Ersatz-Verweise, Index-Verdichtung) gibt es nach `docs/backlog/README.md` nicht (Stichwortsuche am 2026-11-05).
- Risiken: (1) `tests/planning.test.ts` und `checkTemplate` setzen heute Index-Format und Feldliste voraus, siehe Ausgangslage; die Anpassung gehört in Teil A und B. (2) Titel-Ähnlichkeit hat Fehltreffer und übersieht Umformulierungen; deshalb nur Hinweis. (3) Zwei Index-Dateien können auseinanderlaufen, wenn jemand Zeilen von Hand verschiebt; der Test aus AC-08 fängt das. (4) Studio und Workbench teilen sich den Ticket-Kern per `replace`; ein Format-Wechsel muss beide Module testen. (5) Beim Anlegen von Hand (ohne Workbench) fehlen Warnung und Gegenrichtung; nur ein Fall für den Notbetrieb.
- Anlage dieses Tickets: Die Workbench-MCP-Tools waren in der anlegenden Session nicht verbunden (ECONNREFUSED); das Ticket wurde deshalb von Hand nach Vorlage angelegt und im Index ergänzt (Ausnahme laut `CLAUDE.md`).
- Beads: https://github.com/gastownhall/beads (nur als Ideengeber).
