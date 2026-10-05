/**
 * „Gesehen“-Merkung der Hinweise der ersten Nacht (S6.3, B-148) je Gerät im `localStorage`, Muster wie `settings.ts`.
 * Gesperrter oder kaputter Speicher: die Merkung lebt nur bis zum Neuladen, die Hinweise erscheinen bei jedem Start.
 */

type SeenStorage = Pick<Storage, 'getItem' | 'setItem'>;

const KEY = 'k3c-guide-seen';

export class GuideSeen {
  private seen: Set<string> | undefined;

  /** `storage` fehlt → `localStorage`, erst beim ersten Zugriff gelesen, weil schon der Zugriff werfen kann. */
  constructor(private readonly storage?: SeenStorage) {}

  /** Gesehene Hinweise (lebendige Menge, ändert sich mit `mark` und `reset`) */
  get ids(): ReadonlySet<string> {
    return (this.seen ??= this.load());
  }

  mark(id: string): void {
    (this.ids as Set<string>).add(id);
    this.save();
  }

  /** Optionen: Hinweise wieder einschalten zeigt alle erneut */
  reset(): void {
    (this.ids as Set<string>).clear();
    this.save();
  }

  private load(): Set<string> {
    try {
      const raw: unknown = JSON.parse((this.storage ?? globalThis.localStorage).getItem(KEY) ?? '[]');
      return new Set(Array.isArray(raw) ? raw.filter((id): id is string => typeof id === 'string') : []);
    } catch {
      return new Set();
    }
  }

  /** Wirft nie; ein gesperrter oder voller Speicher behält die Merkung nur im Speicher. */
  private save(): void {
    try {
      (this.storage ?? globalThis.localStorage).setItem(KEY, JSON.stringify([...this.ids]));
    } catch {
      // gesperrt: nur bis zum Neuladen gemerkt
    }
  }
}

/** Eine Merkung je Seite: Optionen setzen sie zurück, das HUD liest und ergänzt sie. */
export const guideSeen = new GuideSeen();
