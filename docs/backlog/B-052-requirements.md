# B-052 · Alle vorausgesetzten Installationen stehen in requirements.md

- **Domäne:** INF
- **Typ:** Idee
- **Prio:** mittel
- **Status:** eingeplant
- **Sprint:** SP01
- **Erstellt:** 2026-09-30
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** 2026-09-30 🧑 Chat-Anweisung von Ralf („zum Go-Gerüst gehört auch ein requirements.md im Root“), mit SP01 Revision 3

## Ausgangslage

Seit SP01.2 braucht das Repo neben Node.js auch Go und golangci-lint. Welche Werkzeuge und Versionen nötig sind,
steht verstreut in `.nvmrc`, `go.mod` und `.github/workflows/ci.yml`.

## Ziel

Alle vorausgesetzten Installationen stehen in `requirements.md`. Nutzen: Ein neuer Rechner oder Agent weiß ohne
Suchen, was zu installieren ist und wie man es prüft.

## Beteiligte und Zielgruppen

Entwickler und Agenten, die das Repo auf einem neuen Rechner einrichten.

## Anforderungen

- `requirements.md` im Repo-Root listet jedes Werkzeug, das nicht über `npm ci` kommt, mit Version, Zweck,
  Quelle der Version und Prüfbefehl.
- Die Versionen stimmen mit `.nvmrc`, `go.mod` und `ci.yml` überein.

## Nicht-Ziele

Installationsskripte; Werkzeuge künftiger Sprints (Docker ab SP03) vor ihrem Einsatz.

## Regeln und Einschränkungen

Prozess nur in `docs/arbeitsweise.md`; keine neue Abhängigkeit ohne Ticket und Zustimmung im Review; Komplexitäts-Budget.
Kommt ein Werkzeug dazu (z. B. Docker in SP03), ergänzt die einführende Session `requirements.md`.

## Beispiele

Neuer Windows-PC → `requirements.md` lesen, Node 22, Go 1.27, golangci-lint 2.14 installieren, `npm ci`,
`npm run check` und `npm run check:go` sind grün.

## Ausnahme- und Fehlerfälle

Lokal ist eine neuere Node-Version installiert → funktioniert meist; maßgeblich für die CI bleibt `.nvmrc`.

## Akzeptanzkriterien

- **AC-01** `requirements.md` im Root nennt Git, Node.js, Go und golangci-lint mit Version, Zweck, Quelle und Prüfbefehl.
- **AC-02** Die Versionen in `requirements.md` stimmen mit `.nvmrc`, `go.mod` und `ci.yml` überein.

## Offene Fragen

keine

## Notizen

Die Übereinstimmung der Versionen prüft kein Test; bei Bedarf als eigenes Ticket.
