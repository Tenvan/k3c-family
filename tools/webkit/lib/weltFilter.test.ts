import { describe, expect, it } from 'vitest';
import { formatWeltHash, parseWeltHash, WELT_VORGABE, zuordnung, type WeltSichtLike } from './weltFilter';

const sicht: WeltSichtLike = {
  welt: 'sengoku',
  basis: 'standard',
  eigene: { 'gegner/ninja': 'sengoku', 'hintergruende/weltbild-standard': 'standard' },
  aus: ['gegner/wolf', 'hintergruende/weltbild-nil'],
};

describe('Welt-Filter (B-1169)', () => {
  it('gesamt zeigt eigene und geerbte mit Herkunft, exklusiv nur die eigenen', () => {
    expect(zuordnung(sicht, 'gesamt', 'gegner/ninja')).toEqual({ sichtbar: true, herkunft: 'sengoku', exklusiv: true, geerbt: false });
    expect(zuordnung(sicht, 'gesamt', 'hintergruende/weltbild-standard')).toMatchObject({ sichtbar: true, herkunft: 'standard', geerbt: true });
    expect(zuordnung(sicht, 'gesamt', 'figuren/monarch')).toMatchObject({ sichtbar: true, herkunft: 'standard', exklusiv: false, geerbt: true });
    expect(zuordnung(sicht, 'exklusiv', 'gegner/ninja').sichtbar).toBe(true);
    expect(zuordnung(sicht, 'exklusiv', 'figuren/monarch').sichtbar).toBe(false);
    expect(zuordnung(sicht, 'exklusiv', 'hintergruende/weltbild-standard').sichtbar).toBe(false);
  });

  it('ersetzte und fremde Assets fehlen in beiden Modi', () => {
    for (const modus of ['gesamt', 'exklusiv'] as const) {
      expect(zuordnung(sicht, modus, 'gegner/wolf').sichtbar).toBe(false);
      expect(zuordnung(sicht, modus, 'hintergruende/weltbild-nil').sichtbar).toBe(false);
    }
  });

  it('ohne Welt ist alles sichtbar', () => {
    expect(zuordnung(null, 'exklusiv', 'gegner/wolf').sichtbar).toBe(true);
  });

  it('Adresse: Rundlauf, Vorgaben fehlen, fremde Parameter bleiben', () => {
    const s = { ...WELT_VORGABE, welt: 'sengoku', modus: 'exklusiv' as const, gruppe: 'gegner', geaendert: true };
    const hash = formatWeltHash(s, '#tab=katalog');
    expect(hash).toBe('#tab=katalog&welt=sengoku&modus=exklusiv&gruppe=gegner&geaendert=1');
    expect(parseWeltHash(hash)).toEqual(s);
    expect(formatWeltHash(WELT_VORGABE, '#welt=nil&modus=exklusiv')).toBe('');
    expect(formatWeltHash({ ...WELT_VORGABE, welt: 'nil' })).toBe('#welt=nil');
  });

  it('ungültiger Modus fällt auf gesamt', () => {
    expect(parseWeltHash('#welt=nil&modus=quatsch')).toMatchObject({ welt: 'nil', modus: 'gesamt' });
  });
});
