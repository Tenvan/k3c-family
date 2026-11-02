# B-679 · Der Chat des Asset-Studios kann jede Bearbeitung des Studios als Aktionskarte auslösen

- **Domäne:** DEV
- **Typ:** Idee
- **Prio:** mittel
- **Umgebung:** offline
- **Status:** erledigt
- **Sprint:** WZ36
- **Projekt:** WZG
- **Erstellt:** 2026-10-31
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** 2026-10-31 autonom, Claude (Auftrag team-lead)

## Ausgangslage

Der Chat (`tools/k3c-studio/backend/chat_*.go`, `frontend/src/chat/`) hat keine Werkzeuge und kennt vier Aufträge: frei
(ohne Kontext), `welt` (Grundgerüst, Antwort „Als Entwurf laden“ im Welten Editor), `zuordnung` (Vorschlagskarten der
Seite Assets) und `quellen` (Kandidaten in die Vorschlagsliste). Die meisten Bearbeitungen des Studios gehen nur über
Knöpfe der Seiten.

Inventar (Seite · Bearbeitung · Aufruf · Datei → Chat heute):

| Seite | Bearbeitung | Aufruf | Datei | Chat heute |
|---|---|---|---|---|
| Assets | Zuordnung eines Slots setzen, umsortieren, leeren | `AssetsSet` | `data/assets.json` | **Lücke L3** |
| Assets | Kandidat zuordnen (lädt bei Bedarf) | `AssetCandidateAssign` | `data/assets.json`, `public/**` | **Lücke L4** |
| Assets | Vorschlag übernehmen / verwerfen | `AssetSuggestionAdopt/Drop` | Register, Vorschlagsliste | Karten nur bei `zuordnung` |
| Assets | Recherche starten, Lizenz eines Treffers bestätigen | `AssetsResearch`, `AssetsLicenseOK` | `data/assets-recherche.json` | **Lücke L5** |
| Assets › Quellen | Kandidaten eintragen | `SourceSuggestAdd` | Vorschlagsliste | Auftrag `quellen` |
| Assets › Quellen | Laden, aus dem Vorrat übernehmen, aufräumen, Netzmodus | `SourceDownload`, `SourceAdopt`, `SourceCleanup`, `SourceSetNetwork` | Vorrat, `public/**` | **Lücke L6** |
| Welten Editor | Welt wählen, anlegen, kopieren, löschen | `PlanWorldSelect/Add/Copy/Remove` | `data/worlds/*.json` | **Lücke L7** |
| Welten Editor | Welt-Entwurf speichern (Name, Inseln, Tiefen, Skalierung, Planung) | `PlanWorldSave` | `data/worlds/*.json` | nur über „Als Entwurf laden“ |
| Welten Editor | Chunk-Planung und -Anzahl einer Tiefe | `PlanSave` | `data/worlds/*.json` | **Lücke L8** |
| Welten Editor | Testspielstand anlegen, löschen | `WorldTestSaveCreate/Delete` | `.studio-saves/` | **Lücke L9** |
| Gebäude-Planer | Plätze, Stufen, Kosten, Mauerlinien ändern | `HubSave` | `data/hub.json`, Regeltabelle | **Lücke L2** |
| alle | Ticket anlegen | `TicketCreate` | `docs/backlog/` | **Lücke L10** |
| Planer, Monarch, Gegner | Seitenkontext im Chat | – | – | **Lücke L1** (nur Assets und Welt) |
| Welten Editor (geplant) | Welt-Overrides (Vererbung) | – | – | **Lücke L11**, Nachfolge-Ticket B-683 nach B-680 |

Monarch und Gegner sind reine Ansichten (Slot-Register nur lesend), Truppen, Biome und Gegnerwerte haben im Studio
keine Bearbeitung: dort gibt es nichts, was der Chat auslösen müsste.

## Ziel

Jede Bearbeitung aus dem Inventar lässt sich im Chat anstoßen: Claude schlägt sie als Aktion vor, das Studio prüft sie
vorab und zeigt sie als Karte, Ralf führt sie per Knopf aus. Der Chat bekommt keine Werkzeuge und schreibt nie selbst.

## Beteiligte und Zielgruppen

🧑 Ralf bearbeitet Assets, Welten und den Hub im Studio; Claude (Chat) schlägt vor.

## Anforderungen

- Jede Nachricht nennt die offene Seite (`seite`); dazu geht ein Aktionskatalog mit Kontext an die CLI (Welten, aktive
  Welt, Slots; auf dem Gebäude-Planer die `data/hub.json`).
- Die Antwort nennt Aktionen in einem Block ```aktionen; das Backend prüft jede vor dem Anzeigen mit derselben Prüfung
  wie die Oberfläche (Trockenlauf) und merkt den Stand der Datei.
- „Ausführen“ ruft dieselbe Funktion wie der Knopf der Seite mit dem gemerkten Stand; geänderte Datei → Ablehnung,
  nichts überschrieben. Jede Karte läuft höchstens einmal.
- Karten erscheinen auf jeder Seite unter dem Chat, mit Grund, wenn gesperrt, und dem Ergebnis nach dem Ausführen.
- Datei ≤ 400 Zeilen, Funktion ≤ 60, Emoji-Logs, Radix nur über `@webkit`, Atomic Design.

## Nicht-Ziele

- „Im Spiel öffnen“ (öffnet ein Browserfenster) und Datei-Import per Upload bleiben Knöpfe der Seiten.
- Neue Bearbeitungen für Monarch, Gegner, Truppen oder Biome (gibt es im Studio nicht).
- Werkzeuge oder Schreibrechte für die CLI.
- Welt-Overrides (Vererbung: Basisdaten, je Welt Ressourcen, Kosten und Werte optional übersteuert): Der Chat muss sie
  bearbeiten können, sobald es sie gibt: Nachfolge-Ticket B-683 nach Umsetzung des Welt-Vererbungs-Tickets B-680.

## Regeln und Einschränkungen

Entscheidung 004 (Studio schreibt `data/**` und `public/`), Schreibregel mit Stand (Stale statt Überschreiben),
Downloads nur nach Bestätigung (B-601: die Karte nennt die Bytes, „Ausführen“ ist die Bestätigung), Lizenzregel Q70.
Chat-Aufrufe nur von Loopback.

## Beispiele

- Planer: „Taverne ab Hub-Stufe 3“ → Karte „Gebäude-Planer ändern“ ohne Befund → Ausführen → `data/hub.json` und
  Regeltabelle geschrieben, REG-Hinweis.
- Assets: „Nimm für sfx:coin die Datei audio/coin.ogg“ → Karte „Zuordnung sfx:coin“ → Ausführen → Register geändert.
- Welt: „Lege eine Welt Nordsee an“ → Karte „Welt anlegen: Nordsee“.

## Ausnahme- und Fehlerfälle

- Unbekannte Art, fehlendes Feld, Prüfbefund → Karte gesperrt mit Grund, kein Knopf.
- Datei seit dem Vorschlag geändert → Ausführen meldet den veralteten Stand, nichts geschrieben.
- Kein Block oder ungültiges JSON → keine Karten, Hinweis an der Antwort.

## Akzeptanzkriterien

- **AC-01** Dieses Ticket enthält das Inventar und die Lückenliste L1–L10.
- **AC-02** Mit `seite` geht der Aktionskatalog samt Kontext als Instruktion an die CLI, auf dem Planer mit der
  `data/hub.json` (Go-Test).
- **AC-03** Ein Block ```aktionen wird zu Karten; jede Art prüft vorab, ungültige sind gesperrt mit Grund (Go-Test).
- **AC-04** Ausführen schreibt über dieselbe Funktion wie die Seite, auf geändertem Stand nicht, und höchstens einmal
  (Go-Test für Planer, Zuordnung, Welt-Dateien).
- **AC-05** Der Katalog deckt L2–L10 ab; jede Art hat Prüfung und Ausführung (Go-Test über die Tabelle).
- **AC-06** Die Oberfläche schickt die Seite mit und zeigt Karten mit Ausführen und Verwerfen auf jeder Seite (Vitest
  gegen den Mock).
- **AC-07** Manuelle Abnahme durch 🧑 mit der echten CLI: je eine Aktion auf Assets, Welten Editor und Planer.

## Offene Fragen

keine (Default: Karten statt Sofort-Ausführung, wie bei den Vorschlagskarten).

## Notizen

2026-10-31 (Claude, Team WZG): Prüfung ergab: bereits umgesetzt (Commits 3025a8fd, 5e0ecf39). AC-02 `TestChatStromAktionen`, AC-03 `TestChatAktionenLesenUndSperren`, `TestChatAktionTicketVorab`, AC-04 `TestChatAktionGebaeude`/`Zuordnung`/`Welten`, AC-05 `TestChatAktionenKatalog` (20 Arten), AC-06 `ChatActions.test.tsx`; AC-01 Inventar im Ticket. AC-07 manuell, 🧑 steht aus.
