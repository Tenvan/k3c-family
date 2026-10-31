# B-678 · Übernehmen aus dem Vorrat ist gegen parallele Aufrufe gesichert, und Vorschauen spielen nie zwei Töne zugleich

- **Domäne:** DEV
- **Typ:** Schuld
- **Prio:** niedrig
- **Umgebung:** offline
- **Status:** eingeplant
- **Sprint:** WZ35
- **Projekt:** WZG
- **Erstellt:** 2026-10-30
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

Befunde des Reviews zu WZ33 (B-677), die nicht in WZ33 behoben wurden:

- `SourceAdopt` (`tools/k3c-studio/backend/app_sources.go`) und `VorratAdopt` (`tools/assetkit/vorrat_adopt.go`) laufen ohne Stand und ohne Sperre über den ganzen Ablauf. Zwei parallele Aufrufe für denselben Eintrag legen zwei Kopien an (`name`, `name-2`) und hängen beide an den Slot. Die Seite Assets ruft sie nur nacheinander auf (`useDownloads`), andere Aufrufer gibt es heute nicht.
- Die Abspielleiste (`assetpage/transport.ts`) stoppt bei Musik nur Effekte und Umgebung und überblendet die Musik. Eine Vorschau-Datei (`playFile`, Vorrat oder Netz) mit Bus „music“ könnte zeitgleich zu einer laufenden Musik klingen (ungeprüft).
- Treffer-Downloads laufen mit dem Client des Studios, der Weiterleitungen nur zu den Vorrat-Hosts erlaubt. Ein Wikimedia-Download mit Weiterleitung scheitert deshalb möglicherweise (ungeprüft).

## Ziel

Das Übernehmen ist atomar je Eintrag, die Abspielleiste lässt nie zwei Töne zugleich klingen, und die Download-Hosts sind geklärt.

## Beteiligte und Zielgruppen

- 🧑 Ralf arbeitet im Studio; Sessions der Domäne DEV setzen um.

## Anforderungen

- `VorratAdopt` und `VorratAdoptIf` reihen parallele Aufrufe je Eintrag ein (Sperre oder Index-Prüfung), sodass höchstens eine Kopie entsteht.
- `playFile` und `playOn` stoppen vor jedem Start alles außer einer Musik, die überblendet; eine zweite Musik-Vorschau ersetzt die laufende.
- Prüfen, ob Wikimedia-Downloads an Weiterleitungen scheitern; wenn ja, die Host-Liste für Weiterleitungen begründet erweitern.

## Nicht-Ziele

- Neue Quellen; Änderungen an den Lizenzregeln (Q70).

## Regeln und Einschränkungen

- Downloads bleiben nur über https von erlaubten Hosts; Dateien sind untrusted (B-601).

## Beispiele

- Zwei gleichzeitige `SourceAdopt` für `q05` → eine Datei unter `public/`, eine Zuordnung.
- ▶ auf eine Vorrat-Musik, während `music:nacht` läuft → die laufende Musik wird überblendet oder gestoppt, es klingt ein Ton.

## Ausnahme- und Fehlerfälle

- Der zweite Aufruf meldet „schon übernommen“ statt einen Fehler zu werfen.

## Akzeptanzkriterien

- **AC-01** Go-Test: zwei parallele `VorratAdopt` für denselben Eintrag ergeben eine Datei und eine Zuordnung.
- **AC-02** Vitest: zwei aufeinanderfolgende Starts auf der Abspielleiste lassen höchstens einen Ton aktiv.
- **AC-03** Wikimedia-Treffer-Download mit Weiterleitung ist geprüft (Test mit `httptest`); das Ergebnis steht in den Notizen.

## Offene Fragen

keine

## Notizen

Quelle: Review WZ33.7 vom 2026-10-30, Befunde 3, 6 und 10.

2026-10-31 (Claude, Team WZG): AC-01 erfüllt: `adoptMu` in `tools/assetkit/vorrat_adopt.go`, Test `TestVorratAdoptParallelEineKopie` (ohne Sperre rot). AC-02 erfüllt: `transport.test.ts` (Audio-Spion, echter Audio-Kern ungeprüft). AC-03 geklärt: Treffer-Downloads laufen über `http.DefaultClient`, `VorratRedirect` gilt nur für Vorrat-Downloads, Wikimedia steht nicht in `VorratHosts`; keine Host-Liste zu erweitern (`TestDownloadHitFolgtWeiterleitung`). Nebenbefund: siehe Ticket zur Weiterleitung auf beliebige Hosts.
