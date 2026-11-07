// RD1.2: Die bewegten Rigs sind im Renderer eingetragen, sobald render/ausgabe.ts geladen ist.
import { describe, expect, it } from 'vitest';
import '../render/ausgabe';
import { RIG_BILDER } from '../render/bild';

describe('Rig-Register', () => {
  it('figur ist eingetragen', () => {
    expect(RIG_BILDER.figur).toBeTypeOf('function');
  });
});
