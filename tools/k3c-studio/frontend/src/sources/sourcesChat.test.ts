import { describe, expect, it } from 'vitest';
import { chatTask, endSourcesChat, startSourcesChat } from '../chat/chatBridge';
import { searchPrompt } from './sourcesFilter';

// B-673: „Claude suchen lassen“ öffnet den Chat mit vorbereitetem Auftrag; die Nachrichten sind Quellen-Aufträge, bis die Seite verlassen wird.
describe('Chat-Auftrag Quellen', () => {
  it('stellt die Auftragsart `quellen` ein und gibt sie beim Verlassen der Seite frei', () => {
    expect(chatTask()).toBeUndefined();
    startSourcesChat(searchPrompt('freesound', 'skill-effekt:heiler', 'sound'));
    expect(chatTask()).toBe('quellen');
    endSourcesChat();
    expect(chatTask()).toBeUndefined();
  });
});
