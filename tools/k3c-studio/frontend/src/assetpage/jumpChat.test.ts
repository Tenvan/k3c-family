import { describe, expect, it, vi } from 'vitest';
import { chatTask, endSourcesChat, startSourcesChat } from '../chat/chatBridge';
import { gotoAssets, takePendingSlot } from './jump';
import { searchPrompt } from './pageLogic';

// B-673/B-677: Sprung mit Slot zur Assets-Seite und der Chat-Auftrag „Claude suchen lassen“.
describe('Sprung zur Assets-Seite', () => {
  it('merkt sich den Slot des Sprungs einmalig und meldet ihn dem Rahmen', () => {
    const seen = vi.fn();
    vi.stubGlobal('window', { dispatchEvent: seen });
    gotoAssets('tavern');
    vi.unstubAllGlobals();
    expect(seen).toHaveBeenCalledOnce();
    expect(takePendingSlot()).toBe('tavern');
    expect(takePendingSlot()).toBe('');
  });
});

describe('Chat-Auftrag Quellen', () => {
  it('stellt die Auftragsart `quellen` ein und gibt sie frei, sobald ein Slot oder Chip gewählt wird', () => {
    expect(chatTask()).toBeUndefined();
    startSourcesChat(searchPrompt('sound'));
    expect(chatTask()).toBe('quellen');
    endSourcesChat();
    expect(chatTask()).toBeUndefined();
  });

  it('nennt die Art im Auftrag', () => {
    expect(searchPrompt('sound')).toContain('Töne');
    expect(searchPrompt('grafik')).toContain('Grafiken');
    expect(searchPrompt(undefined)).toContain('Töne und Grafiken');
  });
});
