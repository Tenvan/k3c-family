# DBG3 · PLAT · Dungeon-Master-Seite /dm

- **Status:** aktiv
- **Domäne:** PLAT
- **Reife:** bereit
- **Einschiebbar:** ja
- **Tickets:** B-232
- **Start-Commit:** 3a6446a
- **Spec:** freigegeben
- **Revision:** 2
- **Freigabe:** 2026-10-05, Chat, durch 🧑 (Zugang HTTP-API, Umfang plus Welle und Tageszeit; umfasst B-232 Revision 2)

## Ausgangslage

Der Cheat-Dialog (B-231, PR #119) bietet im Spiel Gold, Material, Zeitraffer und Pause. Dev-Aktionen laufen nur über den WebSocket eines Spiel-Geräts mit Monarch; HTTP kennt nur `/api/status*`, `/api/save*`, `/api/level`, `/api/report`. Eine Seite ohne Monarch kann heute keine Dev-Aktion auslösen. Details: B-232.

## Ziel

Der Spielleiter steuert und beobachtet laufende Räume vom Handy oder Tablet, ohne das Spielbild zu stören. Am Ende sichtbar: `http://<server>/dm` am Handy, Raum wählen, „Zeit 4×“ tippen, der Raum am TV läuft schneller.

## Beteiligte und Zielgruppen

🧑 als Spielleiter am Spieleabend und Tester; 🧑 nimmt am Handy ab.

## Anforderungen

`B-232 › Anforderungen`, erste Fassung ohne Passwortschutz. Zugang über eine HTTP-API `/api/dev` (kein Protokoll-Bump):
Raumliste und Diagnose per Abfrage im Sekundentakt, Aktionen per POST. Umfang: Gold, Material, Zeitraffer, Pause,
dazu **Welle auslösen** und **Tageszeit setzen** (Sprung zum nächsten Tag, zur Dämmerung oder zur Nacht).

## Nicht-Ziele

Spielen auf der Seite; Passwortschutz (eigenes Ticket, sobald die Seite steht); Grad wechseln (B-107), Raum-Neustart (B-080), Gegner einzeln setzen.

## Regeln und Einschränkungen

- Dev-Aktionen nur im Dev-Mode des Servers.
- Domäne PLAT (Seite `dm.html`). Der Server-Zugang (`engine/net`, `engine/room`) und zwei Dev-Hilfen in `engine/sim/dev.go`
  (keine Spielregel, nur Dev-Mode) bekommen eine eigene Session DBG3.1 vor der Seite (`docs/arbeitsweise.md` › Domänen);
  das WebSocket-Protokoll bleibt unverändert.
- Aufruf über `/dm`, nicht über die Landingpage-Kacheln. Die Seite läuft außerhalb der Shell; die Seiten-Regeln aus
  `CLAUDE.md` gelten für sie nicht, `tests/projectRules.test.ts` nimmt sie aus.
- Datei ≤ 400, Funktion ≤ 60 Zeilen, keine neue Abhängigkeit.

## Beispiele

`B-232 › Beispiele`. Dazu: Raum KRNZ am Tag → „Nacht“ → Phase springt auf `night`, die Nachtwelle startet; „Welle“ → Welle +1.

## Ausnahme- und Fehlerfälle

`B-232 › Ausnahme- und Fehlerfälle`.

## Akzeptanzkriterien

- **AC-01** `B-232/AC-01` – `/dm` liefert die Seite, auf 375 px Breite ohne waagerechtes Scrollen bedienbar.
- **AC-02** `B-232/AC-02` – Diagnosedaten eines gewählten Raums, live aktualisiert.
- **AC-03** `B-232/AC-03` – Gold, Material, Zeitraffer und Pause wirken über die Seite (Test).
- **AC-04** `B-232/AC-04` – 🧑 hat die Seite am Handy abgenommen.
- **AC-05** `B-232/AC-05` – Welle auslösen und Tageszeit setzen wirken über die Seite (Test).

## Offene Fragen

- Passwortschutz: Zeitpunkt und Art (🧑, nicht blockierend).

## Sessions

| Nr. | Datei | Typ | Agent | Status |
|---|---|---|---|---|
| DBG3.1 | `DBG3.1-dev-api.md` | Umsetzung | autonom | fertig |
| DBG3.2 | `DBG3.2-seite-dm.md` | Umsetzung | autonom | fertig |
| DBG3.3 | `DBG3.3-review.md` | Review | autonom | fertig |
| DBG3.4 | `DBG3.4-abnahme-handy.md` | Workshop | Mensch | offen |

## Abnahme

2026-10-05 · AC-01 bis AC-03, AC-05 laut DBG3.1/DBG3.2 geprüft; AC-04 angenommen, Validierung offen (DBG3.4, Handy).
Keine schweren Befunde (DBG3.3); `/api/dev` liest ohne Token, Passwortschutz bleibt Offene Frage. Keine neuen Tickets.
Version: v0.11.0 vorgeschlagen (Minor: neue Server-API `/api/dev` und Seite `/dm`).
