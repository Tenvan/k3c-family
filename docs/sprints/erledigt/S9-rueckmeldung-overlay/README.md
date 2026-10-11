# S9 · CLI, SIM · Rückmeldung für Schlag und Skills, ein Hinweis je Spieler

- **Status:** erledigt
- **Projekt:** BED
- **Domäne:** CLI, SIM
- **Reife:** bereit
- **Tickets:** B-319, B-318, B-321, B-395, B-396
- **Start-Commit:** edf3f1b1
- **Spec:** freigegeben
- **Revision:** 4
- **Freigabe:** 2026-10-11, 🧑 im Chat, Revision 4 (B-396 als S9.7)

## Ausgangslage

Die PC-Abnahme von S3 (S3.4) fand zwei Mängel: Das Aktionen-Overlay zeigt mehrere Hinweise übereinander und über Preisschildern (B-319), Schlag und Skills zeigen ohne Ziel keine Rückmeldung (B-318).

## Ziel

Spieler sehen je Weltposition höchstens ein Anzeige-Element und erkennen jeden Druck auf Schlag oder Skill. Am Ende sichtbar: Jeder Tastendruck auf Schlag oder Skill ist sichtbar, das Aktionen-Overlay zeigt je Spieler einen Hinweis.

## Beteiligte und Zielgruppen

Alle Spieler; 🧑 bei Abnahmen; Agent baut in `src/scenes/`.

## Anforderungen

B-319 › Anforderungen; B-318 › Anforderungen.

## Nicht-Ziele

Neue Skills, Skill-Menü-Umbau (S3).

## Regeln und Einschränkungen

CLI; nach S3. Schriftgrößen aus dem Katalog (`FONTS`), B nicht belegen.

- Beschluss 🧑 2026-10-06 (Chat): „Ein Element je Weltposition“ gilt je Bildschirmzelle; jeder Spieler sieht in seiner Zelle nur seinen eigenen Hinweis (B-319).
- Beschluss 🧑 2026-10-06 (Chat): Die Rückmeldung für Schlag und Skills ist ein Server-Ereignis (`strike` auch ohne Treffer, `castFailed`), der Client zeichnet nur. Das Ereignis erzeugt die SIM (`engine/sim/`), das Protokoll überträgt es; S9 bleibt CLI, das Protokoll bekommt eine eigene Session (S9.2, Grenzfall Protokoll) (B-318).
- Das Erzeugen in `engine/sim/` (`monarch.go` › `stepAttack`, `skills.go` › `castSkill`) gehört zur Domäne SIM und passt nicht in diesen CLI-Sprint; das übernimmt B-321 in SK1.3, Voraussetzung von S9.2.

## Beispiele

Zwei Ziele nah beieinander → nur der Hinweis des nächsten; Schlag ins Leere → kurzer Schwung sichtbar.

## Ausnahme- und Fehlerfälle

Hinweis und Preisschild an derselben Position → Preisschild gewinnt, Hinweis entfällt.

## Akzeptanzkriterien

- **AC-01** Das Aktionen-Overlay zeigt je Spieler nur einen Hinweis, 24 px, nie über einem Preisschild (B-319/AC-01, B-319/AC-02, B-319/AC-03, B-319/AC-04).
- **AC-02** Schlag und Skills zeigen auch ohne Ziel sichtbar, dass die Taste ankam: Der Client zeichnet die Server-Ereignisse `strike` (Monarch, mit und ohne Treffer) und `castFailed` („kein Ziel“, keine Abklingzeit) je Monarch getrennt (B-318/AC-01, B-318/AC-02, B-318/AC-03).
- **AC-03** `task check` und `task check:go` grün; `src/scenes` rechnet nichts (`noSim.test.ts`).
- **AC-04** Schlag ohne Treffer und Skill ohne Ziel erzeugen in der Simulation ein Ereignis, Golden ändern sich nur um diese Ereignisse (B-321/AC-01, B-321/AC-02, B-321/AC-03).
- **AC-05** Von mehreren Preisschildern in Reichweite zeigt jeder Spieler nur das nächste; nie liegen zwei Preisschilder übereinander (B-395/AC-01, B-395/AC-02).
- **AC-06** Beim Schlag, auch ohne Treffer, holt der Monarch sichtbar aus (Angriffs-Animation), je Monarch getrennt, nicht beim Schlag eines Gegners (B-396/AC-01, B-396/AC-02).

## Offene Fragen

- Nicht mehr blockierend: Das SIM-Ticket ist B-321 (eingeplant in SK1.3, Felder `strike` mit `hit`, `castFailed` mit `from`, `slot`, `x`); S9.2 und S9.3 starten nach SK1.3.

## Sessions

| Nr. | Datei | Typ | Agent | Status |
|---|---|---|---|---|
| S9.1 | `S9.1-ein-hinweis.md` | Umsetzung | autonom | fertig |
| S9.1a | `S9.1a-ereignis-ohne-ziel.md` | Umsetzung | autonom | fertig |
| S9.2 | `S9.2-protokoll-rueckmeldung.md` | Umsetzung | autonom | fertig |
| S9.3 | `S9.3-rueckmeldung-zeichnen.md` | Umsetzung | autonom | fertig |
| S9.4 | `S9.4-review.md` | Review | autonom | fertig |
| S9.5 | `S9.5-pc-abnahme.md` | Workshop | Mensch | fertig |
| S9.6 | `S9.6-nur-naechstes-preisschild.md` | Umsetzung | autonom | fertig |
| S9.7 | `S9.7-monarch-holt-aus.md` | Umsetzung | autonom | fertig |

## Abnahme

- 2026-10-09 (S9.4, Review): AC-01 geprüft (S9.1), AC-02 geprüft (S9.2, S9.3), AC-03 geprüft (S9.1–S9.4, `task check`, `task check:go`), AC-04 geprüft (S9.1a; Golden nur `attackCooldown`).
- 2026-10-11 (S9.5, 🧑 am PC): AC-01 erfüllt (nie zwei Texte, B-319/AC-04), AC-02 erfüllt (B-318/AC-03); AC-05 geprüft (S9.6, Test; PC: nie zwei Preisschilder, B-395/AC-02), AC-06 geprüft (S9.7, Test; PC: Monarch holt aus, B-396/AC-02).
- Behoben: gehaltener Skill ohne Ziel stapelte „kein Ziel“ je Tick (S9.4). Neue Tickets: B-377, B-395 und B-396 (beide erledigt). B-318, B-319, B-321, B-395, B-396 archiviert.
- Reviews: unabhängige Agenten über die Diffs von S9.6 und S9.7, keine schweren Befunde.
- Version: v0.17.0 vorgeschlagen (Minor: neue Rückmeldung im Spiel; nach den offenen Vorschlägen v0.15.0 U5 und v0.16.0 PL1); nicht gesetzt (wartet auf Bestätigung 🧑).
