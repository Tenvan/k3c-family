import { describe, expect, it } from 'vitest';
import { CROSSFADE_SECONDS } from '@game/audio/crossfade';
import { FAR_GAIN, playPlan, playStatus } from './playback';

describe('Abspiel-Zuordnung je Slot (B-459/AC-06)', () => {
  it('Atlas-Ton: SFX, fern gedämpft', () => {
    expect(playPlan({ bus: 'sfx' }, { datei: 'audio/atlas.ogg', sprite: 'coin' })).toEqual({ kind: 'sprite', name: 'coin', bus: 'sfx', gain: 1 });
    const far = playPlan({ bus: 'sfx' }, { datei: 'audio/atlas.ogg', sprite: 'coin' }, true);
    expect(far).toEqual({ kind: 'sprite', name: 'coin', bus: 'sfx', gain: FAR_GAIN });
    expect(FAR_GAIN).toBeGreaterThan(0); // hörbar gedämpft, nicht stumm
    expect(FAR_GAIN).toBeLessThan(1);
  });
  it('Musik blendet über (music:tag → music:nacht)', () => {
    expect(playPlan({ bus: 'music' }, { datei: 'audio/musik/nacht.ogg' })).toEqual({ kind: 'file', url: 'audio/musik/nacht.ogg', opts: { fade: CROSSFADE_SECONDS } });
  });
  it('Umgebungsgeräusch als Schleife, sonst SFX einmal', () => {
    expect(playPlan({ bus: 'ambient' }, { datei: 'audio/wald.ogg' })).toEqual({ kind: 'file', url: 'audio/wald.ogg', opts: { bus: 'ambient', loop: true } });
    expect(playPlan({ bus: 'sfx' }, { datei: 'audio/hit.ogg' })).toEqual({ kind: 'file', url: 'audio/hit.ogg', opts: { bus: 'sfx', loop: false } });
    expect(playPlan({}, { datei: 'audio/hit.ogg' }).kind).toBe('file');
  });
  it('Statuszeile nennt Datei und Ton oder die fehlende Datei', () => {
    expect(playStatus('Münze', { datei: 'audio/atlas.ogg', sprite: 'coin' }, true)).toBe('Münze · audio/atlas.ogg#coin');
    expect(playStatus('Tag', { datei: 'audio/x.ogg' }, false)).toContain('audio/x.ogg');
    expect(playStatus('Münze', { datei: 'audio/atlas.ogg', sprite: 'coin' }, false)).toContain('„coin“ nicht im Audio-Atlas');
  });
});
