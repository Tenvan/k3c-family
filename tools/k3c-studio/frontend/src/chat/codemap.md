# tools/k3c-studio/frontend/src/chat/

## Responsibility
Chat rechts im Asset-Studio (B-488, WZ19.2): assistant-ui mit der Claude Code CLI als Modell, auf allen Seiten, ein- und ausblendbar.

## Design
- `ChatPanel.tsx`: `ChatPanel` hält Sitzung (`key`, Chat-Kennung, geladene Nachrichten), Liste gespeicherter Chats und Wahl; bleibt auch ausgeblendet eingehängt; `ChatToggle` für die Kopfzeile.
- `ChatControls.tsx`: Kopf mit „Neuer Chat“, Auswahl gespeicherter Chats, „Chat löschen“ und den Auswahlen Anbieter/Modell/Denkstufe, alle als `SelectField` und `Button` aus `@webkit/atoms` (leerer Wert = Neuer Chat bzw. Standard). `useChatChoices.ts`: Werte aus `backend.chatOptions()`, Wahl gemerkt (`chat.provider|model|effort`).
- `ChatThread.tsx`: Verlauf, Hinweiszeile, Eingabe (`useLocalRuntime` mit `initialMessages`); ein anderer Chat hängt ihn mit neuem `key` neu ein. `chatConvert.ts`: gespeicherte Nachrichten → Nachrichten der Oberfläche, Sortierung, Beschriftung.
- `ChatSplitter.tsx`: App-eigene Trennlinie Seite | Chat (Zeiger, Pfeiltasten, Doppelklick), Breite in px gemerkt (`chat.width`, 280 px … 60 % des Fensters).
- `chatAdapter.ts`: `createChatAdapter(stream, hooks)` macht aus dem Antwort-Strom eine fortlaufende Antwort; `done` setzt den Ausgang (complete, cancelled, error), der Chat des Servers bleibt erhalten, die Wahl geht je Nachricht mit; bei Fehler entfällt der doppelte Text der CLI (`answerText`, B-515).
- `chatBridge.ts`: Zustand außerhalb der Oberfläche: `useChatOpen`/`setChatOpen` (gemerkt unter `k3c-studio.chat.state`), `registerDraftLoader`/`loadDraft` für „Als Entwurf laden“, `chatTask()` macht Nachrichten zu Welt-Aufträgen (`task: 'welt'`), solange der Welten Editor einen Lader angemeldet hat (B-516; `ChatThread` zeigt dann den Hinweis `.chat-task`).
- Tests: `chatAdapter.test.ts`, `chatConvert.test.ts`, `ChatPanel.test.tsx`, `ChatSession.test.tsx`, `ChatSplitter.test.tsx` (happy-dom gegen den Mock, Hilfen in `chatTestKit.tsx`; Auswahlen per Wert über `tools/webkit/testing`: `choose`, `value`, `values`, `labels`).

## Flow
Eingabe → assistant-ui → Adapter → `backend.chatStream` (mit Chat, Text, Anbieter, Modell, Denkstufe) → `POST /api/studio/chat` (Go `backend/chat_http.go`) → Stücke zurück in die Antwort.

## Integration
- Backend: `tools/k3c-studio/backend/chat_*.go` (RPC `ChatList`, `ChatGet`, `ChatDelete`, `ChatOptions`); Einbindung in `../App.tsx`; Stile `../styles/app.css` (`.chat*`).
