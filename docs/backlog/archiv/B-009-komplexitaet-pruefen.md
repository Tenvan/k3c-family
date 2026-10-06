# B-009 · Komplexitäts-Budget wird automatisch geprüft

- **Domäne:** INF
- **Typ:** Idee
- **Prio:** hoch
- **Umgebung:** offline
- **Status:** erledigt
- **Sprint:** SP01
- **Erstellt:** 2026-09-29
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** 2026-09-30 🧑 Chat-Freigabe durch Ralf (Revision 1)

## Ausgangslage

Das Komplexitäts-Budget steht nur in `docs/arbeitsweise.md`; kein Werkzeug prüft es.

## Ziel

Komplexitäts-Budget wird automatisch geprüft. Nutzen: Niedrige Komplexität ist Pflicht in jeder Session und soll nicht vom Gedächtnis des Reviews abhängen.

## Beteiligte und Zielgruppen

Entwickler und Cloud-Agenten, die Sessions autonom abarbeiten; Review-Session.

## Anforderungen

- Oxlint (TypeScript) und `golangci-lint` (Go) prüfen Datei- und Funktionslänge, Verschachtelung und zyklomatische Komplexität.
- Regel-Tests prüfen Schichtgrenzen und die Ratsche für Bestandscode.

## Nicht-Ziele

Bestandscode verkleinern (B-018, B-034); Stil-Regeln.

## Regeln und Einschränkungen

Prozess nur in `docs/arbeitsweise.md`; keine neue Abhängigkeit ohne Ticket und Zustimmung im Review; Komplexitäts-Budget. Grenzen aus `docs/arbeitsweise.md` › Komplexitäts-Budget.

## Beispiele

Neue Funktion mit 61 Zeilen → `npm run lint` scheitert.

## Ausnahme- und Fehlerfälle

Bestandsdatei über dem Ziel → Eintrag in der Ausnahmeliste mit heutigem Wert, der nur sinken darf.

## Akzeptanzkriterien

- **AC-01** `npm run check` scheitert bei Verstößen gegen das Budget, lokal und in der CI.
- **AC-02** Die Go-Prüfungen scheitern bei Verstößen, in der CI.

## Offene Fragen

keine

## Notizen

TS 7 hat keine JS-API, deshalb Oxlint statt `typescript-eslint`.
Umgesetzt in SP01, lokal geprüft im Review SP01.4; der CI-Teil von AC-01 und AC-02 ist verschoben nach B-053.
