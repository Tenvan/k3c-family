# LG1.3 · Xbox: WebGL 1 und Texturbudget 130 MiB

- **Status:** fertig
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** CLI
- **Lane:** –
- **Modell:** –
- **Umgebung:** offline
- **Branch:** lg1/3-xbox-webgl1-textur-budget
- **Abhängig von:** LG1.1
- **Tickets:** B-772
- **Kriterien:** AC-03

## Ziel

Der Client wählt auf der Xbox WebGL 1 und hält den Texturspeicher unter 130 MiB; das Client-Log meldet Version, Budget und Verbrauch (B-772).

## Kontext

`src/core/renderInfo.ts` (B-761) liefert Renderer, GPU, max. Texturgröße, Texturzahl und -speicher; `src/core/clientLog.ts` sendet an `/api/clientlog` (Rate-Limit). `src/main.ts` startet mit `Phaser.AUTO`. RX1.2 hat den Kontextverlust abgefangen, RX1.6 plant das Texturprofil (≤ 2048 px, Laden bei Bedarf); LG1.3 darf RX1.6 nicht doppeln und stimmt sich mit dessen Stand ab. Werte (130 MiB) stammen aus der Erinnerung von 🧑 und sind noch nicht am Log belegt. Spec: B-772.

## Erlaubte Dateien

- `src/main.ts`, `src/core/renderInfo.ts`, `src/core/clientLog.ts`, neue Datei für die Renderer-Wahl und Budgetprüfung samt `*.test.ts`
- `data/` (Budget-Wert, z. B. `data/render.json`), falls nötig die Typen unter `src/model/`

## Nicht-Ziele

Kein WebGL 2 auf der Xbox, keine Änderung an anderen Geräten, keine neue Grafik, kein Umbau der Texturgruppen (RX1.6).

## Schritte

1. Vor Beginn Log am Pi prüfen (`logs_query` mit `server`, wenn LG1.2 fertig; sonst `curl`): stimmen WebGL 1 und 130 MiB? Abweichung → 🧑 melden.
2. Geräteerkennung Xbox, WebGL-1-Kontext als bevorzugter Weg, Fallback-Kette erhalten.
3. Budget aus `data/` lesen, gegen den Texturspeicher aus `renderInfo.ts` prüfen, bei Überschreitung kleinere Auflösung oder Verzögerung statt Kontextverlust.
4. Eine Startzeile im Client-Log (Emoji-Präfix) mit Version, Budget, Verbrauch.
5. Tests für Wahl und Budget.

## Fertig, wenn

- [ ] AC-03: `B-772/AC-01` und `AC-02` grün: Renderer-Wahl und Budgetprüfung getestet, Budget kommt aus `data/`.

## Prüfen

```bash
task check:diff
```

Einmal vor dem Commit; während der Arbeit nur gezielte Tests (`docs/arbeitsweise.md` › Tempo im Lauf).
Review-Session statt dessen: `task check`, `task check:go` (nacheinander) und das Abnahme-Ticket des Sprints anlegen.
Sim-, Balance- und Gen-Läufe (`sim_test`, Balancing, `gen:paper` samt Golden, `check:race`) nur, wenn diese Session das
betroffene Feature entwickelt; sonst laufen sie im Abnahme-Ticket (`docs/arbeitsweise.md` › Sim, Balance und Gen).
Manuelle Prüfungen (Browser, Xbox, TV) nur, wenn diese Datei sie nennt und 🧑 sie für diesen Lauf freigegeben hat.

## Ergebnis

- AC-03 geprüft: `src/core/textureBudget.test.ts` und die neuen Fälle in `src/core/renderLog.test.ts` (29 Tests in den drei Dateien grün), `task typecheck` und `task lint` grün. B-772/AC-01 (Wahl der WebGL-Version, Budgetprüfung) und AC-02 (Budget aus `data/render.json`, 130 MiB, nur auf der Xbox) erfüllt.
- Befund zur WebGL-Version: Phaser 4.2.1 fordert den Kontext mit `getContext('webgl')` an (`node_modules/phaser/src/renderer/webgl/WebGLRenderer.js:709`), das ist per Definition ein WebGL-1-Kontext. Der Client läuft also schon mit WebGL 1; eine eigene „Wahl“ war nicht nötig. Neu ist die Startzeile mit `glVersion`, Budget und Ampel.
- Neu: `data/render.json` (`xbox.textureMiB` 130, `near` 0,85), `src/core/textureBudget.ts` (Xbox-Erkennung am User-Agent, Ampel ok/near/over), Budget in Startzeile, Verlaufszeile und Ladezeile des Render-Logs, Warnzeile bei Überschreitung (höchstens eine je Minute).
- Abweichung von B-772/Anforderungen: Es gibt **keine automatische Verkleinerung** der Texturen bei Überschreitung, nur Messung, Log und Warnung. Eine kleinere Auflösung braucht Texturvarianten und gehört zum Texturprofil RX1.6 (`≤ 2048 px`, Laden je Stufe, Domäne PAP). Nachgelagert: RX1.6 sollte das Budget aus `data/render.json` als Ziel nutzen.
- Das Budget (130 MiB) stammt aus der Erinnerung von 🧑 und ist an den Pi-Logs noch nicht geprüft (LG1.6, `B-772/AC-03`).
