# S9 · Zusammenfassung

Regelbasiert aus der Sprint-README erzeugt (`plan_digest`, B-696); maßgeblich bleibt [README.md](README.md).

- **Titel:** Rückmeldung für Schlag und Skills, ein Hinweis je Spieler
- **Projekt:** BED

## Ziel

Spieler sehen je Weltposition höchstens ein Anzeige-Element und erkennen jeden Druck auf Schlag oder Skill. Am Ende sichtbar: Jeder Tastendruck auf Schlag oder Skill ist sichtbar, das Aktionen-Overlay zeigt je Spieler einen Hinweis.

## Ergebnis

- 2026-10-09 (S9.4, Review): AC-01 geprüft (S9.1), AC-02 geprüft (S9.2, S9.3), AC-03 geprüft (S9.1–S9.4, `task check`, `task check:go`), AC-04 geprüft (S9.1a; Golden nur `attackCooldown`). - 2026-10-11 (S9.5, 🧑 am PC): AC-01 erfüllt (nie zwei Texte, B-319/AC-04), AC-02 erfüllt (B-318/AC-03); AC-05 geprüft (S9.6, Test; PC: nie zwei Preisschilder, B-395/AC-02), AC-06 geprüft (S9.7, Test; PC: Monarch holt aus, B-396/AC-02). - Behoben: gehaltener Skill ohne Ziel stapelte „kein Ziel“ je Tick (S9.4). Neue Tickets: B-377, B-395 und B-396 (beide erledigt). B-318, B-319, B-321, B-395, B-396 archiviert. …

## Kriterien

| Kriterium | Status | Inhalt |
|---|---|---|
| AC-01 | geprüft | Das Aktionen-Overlay zeigt je Spieler nur einen Hinweis, 24 px, nie über einem Preisschild … |
| AC-02 | geprüft | Schlag und Skills zeigen auch ohne Ziel sichtbar, dass die Taste ankam: Der Client zeichne … |
| AC-03 | geprüft | `task check` und `task check:go` grün; `src/scenes` rechnet nichts (`noSim.test.ts`). |
| AC-04 | geprüft | Schlag ohne Treffer und Skill ohne Ziel erzeugen in der Simulation ein Ereignis, Golden än … |
| AC-05 | geprüft | Von mehreren Preisschildern in Reichweite zeigt jeder Spieler nur das nächste; nie liegen … |
| AC-06 | geprüft | Beim Schlag, auch ohne Treffer, holt der Monarch sichtbar aus (Angriffs-Animation), je Mon … |

## Tickets

| Ticket | Status | Kriterien | Titel |
|---|---|---|---|
| B-319 | erledigt | AC-01 geprüft | Das Aktionen-Overlay zeigt je Spieler nur einen Hinweis, 24 px, nie über einem P … |
| B-318 | erledigt | AC-02 geprüft | Schlag und Skills zeigen auch ohne Ziel sichtbar, dass die Taste ankam |
| B-321 | erledigt | AC-04 geprüft | Schlag ohne Treffer und Skill ohne Ziel erzeugen ein Ereignis |
| B-395 | erledigt | AC-05 geprüft | Nahe Bauplätze zeigen ihre Preisschilder nie übereinander |
| B-396 | erledigt | AC-06 geprüft | Der Monarch holt beim Schlag ohne Ziel sichtbar aus |
