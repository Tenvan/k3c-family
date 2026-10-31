import { useState } from 'react';
import { Button, Flex, Text } from '../atoms';
import { CheckField } from '../molecules';

export interface Facet {
  /** Schlüssel der Gruppe (z. B. `art`). */
  id: string;
  title: string;
  options: readonly { value: string; label: string; count?: number }[];
}

export interface FacetRailProps {
  facets: readonly Facet[];
  /** Gewählte Werte je Gruppe; mehrere Werte einer Gruppe gelten als „oder“, Gruppen untereinander als „und“. */
  selected: Readonly<Record<string, readonly string[]>>;
  onToggle: (facet: string, value: string, on: boolean) => void;
  onClear: () => void;
  /** Höchstens so viele Optionen je Gruppe, der Rest hinter „weitere“; gewählte bleiben immer sichtbar (Vorgabe 8). */
  limit?: number;
}

/** Eine Gruppe: lange Listen zeigen die ersten Optionen und die gewählten, der Rest klappt auf. */
function FacetGroup({ facet, chosen, limit, onToggle }: { facet: Facet; chosen: readonly string[]; limit: number; onToggle: FacetRailProps['onToggle'] }) {
  const [all, setAll] = useState(false);
  const hidden = all ? [] : facet.options.slice(limit).filter((o) => !chosen.includes(o.value));
  const shown = facet.options.filter((o) => !hidden.includes(o));
  return (
    <Flex direction="column" gap="1" role="group" aria-label={facet.title}>
      <Text size="1" color="gray" weight="bold">{facet.title}</Text>
      {shown.map((o) => (
        <CheckField key={o.value} size="1" label={o.count === undefined ? o.label : `${o.label} (${o.count})`}
          checked={chosen.includes(o.value)} onCheckedChange={(c) => onToggle(facet.id, o.value, c === true)} />
      ))}
      {hidden.length > 0 && <Button size="1" variant="ghost" color="gray" onClick={() => setAll(true)}>{hidden.length} weitere anzeigen</Button>}
      {all && facet.options.length > limit && <Button size="1" variant="ghost" color="gray" onClick={() => setAll(false)}>weniger anzeigen</Button>}
    </Flex>
  );
}

/** Facetten links (B-677): je Gruppe Markierfelder mit Zähler; „Zurücksetzen“ erscheint, sobald etwas gewählt ist. */
export function FacetRail({ facets, selected, onToggle, onClear, limit = 8 }: FacetRailProps) {
  const any = facets.some((f) => (selected[f.id] ?? []).length > 0);
  return (
    <Flex direction="column" gap="3" role="navigation" aria-label="Facetten" className="wk-facets">
      {facets.map((f) => <FacetGroup key={f.id} facet={f} chosen={selected[f.id] ?? []} limit={limit} onToggle={onToggle} />)}
      {any && <Button size="1" variant="soft" color="gray" onClick={onClear}>Facetten zurücksetzen</Button>}
    </Flex>
  );
}
