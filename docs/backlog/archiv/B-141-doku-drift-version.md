# B-141 · Doku und CLAUDE.md stimmen mit dem Code überein, die Version ist sichtbar

- **Domäne:** INF
- **Typ:** Schuld
- **Prio:** mittel
- **Umgebung:** live
- **Status:** erledigt
- **Sprint:** F5
- **Projekt:** –
- **Erstellt:** 2026-10-02
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** 2026-10-03, Chat (Ralf), mit Sprint F5

## Ausgangslage

- `docs/rules/ist-abgleich.md`, `ist-gegner-bosse.md`, `ist-material-gebaeude.md` und `ist-monarch-buerger.md` beschreiben den Ist-Stand vor den Regelwerken R1 bis R4; sie sind überholt, werden aber von Regeln und Tickets noch verlinkt.
- `CLAUDE.md` nennt die Spielstruktur (Entscheidung 003) als „Umsetzung offen, Ist-Code: eine Stufe = eine Welt“; seit SP12 bis SP14 gibt es Inseln und Stufen im Code.
- Die Version steckt im Server (`cmd/k3c-server/main.go`, `-X main.version`) und nur im token-geschützten `GET /api/status` (`engine/net/status.go`); `GET /api/health` liefert nur `{"ok":true}`. Die Landingpage (`index.html`, Fußzeile) zeigt keine Version. Der Client hat keine Build-Version.
- `engine/net/static.go` setzt für `.html` bereits `Cache-Control: no-cache`, für alle anderen Dateien `immutable`; ein Test dafür ist nicht gefunden.

## Ziel

Die Doku zeigt den heutigen Stand, und auf der Landingpage steht, welche Version Server und Client haben. Nutzen: Pi-Image und gecachter Xbox-Client, die auseinanderlaufen, fallen sofort auf.

## Beteiligte und Zielgruppen

Entwickler, Agenten (lesen `CLAUDE.md`), Betreiber des Pi, Spieler an der Xbox (sehen die Version).

## Anforderungen

- Die vier `ist-*.md` werden nach `docs/rules/archiv/` verschoben; alle Verweise darauf (`docs/`, `CLAUDE.md`) sind angepasst, nichts verlinkt eine fehlende Datei.
- `CLAUDE.md`: der Satz zur Spielstruktur (003) beschreibt den Ist-Stand ohne „Umsetzung offen“, kurz und ohne Erfindungen.
- `GET /api/health` liefert zusätzlich `version`; `{"ok":true}` bleibt erhalten.
- Der Client kennt eine Build-Version (Vite-Define, aus `VERSION` oder `package.json`); die Fußzeile der Landingpage zeigt „Server <Version> · Client <Version>“ und hebt Abweichung hervor.
- Test: `index.html` und andere `.html`-Dateien werden mit `no-cache` ausgeliefert, JS und Assets mit `immutable`.

## Nicht-Ziele

Kacheln der Landingpage (B-079, gleicher Sprint), Release-Prozess und Tag-Konvention (B-170), Pi-Betrieb (B-142).

## Regeln und Einschränkungen

Domäne INF; Änderungen an `engine/net/` und `index.html` sind Domänen-Ausnahmen, die die Sprint-Spec F5 nennt (wie SP11). Seitenregeln aus `CLAUDE.md` bleiben (`installPageChrome()`, Eintrag in `src/landing/pages.ts`). Kein Math.random, keine neue Abhängigkeit.

## Beispiele

Pi läuft mit Image v0.3.0, die Xbox hat noch den alten Client im Cache → Fußzeile: „Server v0.3.0 · Client v0.2.0“, markiert.

## Ausnahme- und Fehlerfälle

Server nicht erreichbar → Fußzeile zeigt „Server –“ und die Client-Version, keine Fehlermeldung im Spiel. Entwicklungsbuild ohne Tag → Version `dev`.

## Akzeptanzkriterien

- **AC-01** `docs/rules/archiv/` enthält die vier `ist-*.md`, `docs/rules/` selbst keine; `grep -rn "ist-abgleich\|ist-gegner\|ist-material\|ist-monarch" docs CLAUDE.md --include=*.md` findet nur Verweise auf `archiv/` oder Dateien in `sprints/erledigt/` (Befehl, Ausgabe).
- **AC-02** Der Satz zur Spielstruktur in `CLAUDE.md` enthält weder „Umsetzung offen“ noch „Ist-Code: eine Stufe = eine Welt“ (Suche).
- **AC-03** Go-Test: `GET /api/health` antwortet mit `ok: true` und `version` (`task check:go`).
- **AC-04** Test (Vitest): Die Fußzeile der Landingpage zeigt eine Server- und eine Client-Version und markiert Abweichung (`task check`).
- **AC-05** Go-Test: `.html` mit `Cache-Control: no-cache`, `.js` mit `immutable` (`task check:go`).

## Offene Fragen

keine

## Notizen

Aus Plan Lücken 10 und 16. Der Plan nennt zusätzlich „Landing-Kacheln an Lobby anpassen“: das ist B-079.
