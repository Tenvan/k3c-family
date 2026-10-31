# B-687 · Der Studio-Chat setzt die Claude-Sitzung per Resume fort und fällt auf den Verlauf als Text zurück

- **Domäne:** DEV
- **Typ:** Idee
- **Prio:** niedrig
- **Umgebung:** offline
- **Status:** eingeplant
- **Sprint:** WZ36
- **Projekt:** WZG
- **Erstellt:** 2026-10-31
- **Spec:** freigegeben
- **Revision:** 2
- **Freigabe:** 2026-10-31 🧑 Ralf im Chat (Q1 Ja), Q2 geprüft

## Ausgangslage

Jeder Aufruf der Claude-CLI im Studio ist neu: `chatArgs` (`tools/k3c-studio/backend/chat_cli.go`) setzt `--no-session-persistence`, `chatPrompt` (`chat_http.go`) klebt die letzten 20 Nachrichten des Verlaufs als Text vor die neue Frage. `TestChatArgsOhneRechte` (`chat_cli_test.go`) verbietet `--resume` und `--continue` ausdrücklich. Das Vergessen in Welt-Aufträgen ist damit nicht behoben, es hängt an Instruktion und fehlendem Entwurf-Zustand (Ticket aus dieser Session, siehe Notizen).

## Ziel

Der Chat nutzt die Sitzung der CLI weiter (Resume), statt bei jeder Nachricht den Verlauf neu zu schicken. Das spart Zeit und Token bei langen Chats und nutzt das Prompt-Caching. Fehlt die Sitzung, läuft der Chat wie heute mit dem Verlauf als Text.

## Beteiligte und Zielgruppen

🧑 Ralf nutzt den Chat im Studio; Sessions der Domäne DEV setzen um.

## Anforderungen

- Je Chat eine Sitzungs-ID (im Verlauf des Studios gespeichert); Folgenachrichten rufen die CLI mit Resume auf, nur die neue Nachricht plus Seitenkontext gehen als Prompt.
- Der Seitenkontext (Welten, Hub, Slots, aktueller Entwurf) kommt bei jeder Nachricht frisch und ersetzt den vorherigen; er darf in der fortgesetzten Sitzung nicht veralten oder kollidieren.
- Fallback: Schlägt Resume fehl (Sitzung fehlt, CLI-Version kennt es nicht), wird der Verlauf als Text geschickt wie heute, ohne Fehler an der Oberfläche.
- Die Härtung bleibt: keine Werkzeuge, keine Rechte, `--safe-mode`; der Test `TestChatArgsOhneRechte` wird gezielt angepasst (nur `--resume` mit Sitzungs-ID erlaubt, `--continue` bleibt verboten).
- Datenschutz und Aufräumen: Sitzungsdateien der CLI werden beim Löschen eines Chats und beim Aufräumen des Verlaufs (`chatCleanup`) mit entfernt oder ihr Ablageort ist dokumentiert.
- Wahrheit bleibt der Verlauf des Studios und der Editor-Entwurf, nicht das Gedächtnis des Modells.

## Nicht-Ziele

Das Welt-Vergessen selbst (Entwurf im Prompt, Hafen-Regel): eigenes Ticket. Werkzeuge für den Chat.

## Regeln und Einschränkungen

`CLAUDE.md` (Logging mit Emoji, Datei ≤ 400 Zeilen, Funktion ≤ 60), B-514 (Prozessbaum), Chat-Härtung aus B-679.

## Beispiele

Zweite Nachricht in einem Chat → CLI-Aufruf mit Resume, Prompt enthält nur die neue Nachricht; Sitzungsdatei fehlt → Aufruf wie heute mit Verlauf als Text.

## Ausnahme- und Fehlerfälle

Resume scheitert → Fallback; CLI ohne Resume-Unterstützung → dauerhaft Fallback mit Logzeile; Chat gelöscht → Sitzungsdatei entfernt.

## Akzeptanzkriterien

- **AC-01** Go-Test (Fake-CLI): Folgenachricht ruft Resume mit der Sitzungs-ID auf und schickt keinen Verlauf.
- **AC-02** Go-Test: fehlende Sitzung fällt auf den Verlauf als Text zurück.
- **AC-03** Go-Test: Chat löschen entfernt die Sitzungsdatei oder ihren Verweis.
- **AC-04** `TestChatArgsOhneRechte` erlaubt nur `--resume` mit ID, keine weiteren Rechte.
- **AC-05** Abnahme durch 🧑 mit der echten CLI.

## Offene Fragen

keine.

- Q1 entschieden von 🧑 am 2026-10-31: Ja, das Studio darf Sitzungsdateien der CLI im Profil der CLI ablegen (`~/.claude/projects/<Arbeitsordner>/<id>.jsonl`); sie werden beim Löschen eines Chats und beim Aufräumen mit entfernt.
- Q2 geprüft am 2026-10-31 (CLI 2.1.304): Eine Probe mit `-p --output-format json --tools "" --permission-mode dontAsk --strict-mcp-config --safe-mode --session-id <id>` und danach `--resume <id>` liefert das zuvor genannte Codewort zurück, ohne Fehler. Resume ist mit der Härtung des Chats vereinbar; die Sitzungsdatei liegt unter dem Arbeitsordner des Chats.

## Notizen

Entstanden aus der Frage im Chat am 2026-10-31, ob Resume effektiver wäre. Empfehlung Claude: erst den Zustand sauber machen (aktueller Entwurf und Fehlerliste im Prompt, Hafen vom Studio ergänzt), danach Resume als Beschleuniger.

2026-10-31 (Claude, Team WZG): AC-01 bis AC-04 maschinell erfüllt (`chat_session_test.go`: `TestChatSitzungFortgesetzt`, `TestChatSitzungFehltVerlaufAlsText`, `TestChatSitzungOhneResumeDauerhaftText`, `TestChatSitzungGeloescht`; `chat_cli_test.go` `TestChatArgsOhneRechte`). AC-05 manuell mit der echten CLI, 🧑 steht aus. Ungeprüft: ob `--append-system-prompt-file` in der Sitzung gespeichert wird. Offen (Review LOW): Der Glob zum Löschen sucht in allen `~/.claude/projects/*`, sollte auf den Ordner des Arbeitsverzeichnisses beschränkt werden.
