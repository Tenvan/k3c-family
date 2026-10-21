import { Tabs } from '@radix-ui/themes';
import type { ReactNode } from 'react';
import { Flex, Text } from '../atoms';

/** Reiter der Kopfzeile: Schlüssel in Anzeige-Reihenfolge und ihre Beschriftung. */
export interface HeaderTabs<P extends string> {
  pages: readonly P[];
  labels: Record<P, string>;
}

interface Props<P extends string> {
  title: string;
  /** Mit Reitern muss die Kopfzeile in einem Tabs.Root stehen (AppFrame tut das). */
  tabs?: HeaderTabs<P>;
  /** Rechts in der Kopfzeile: Badges, Schalter, Verweise. */
  right?: ReactNode;
}

/** Kopfzeile eines Wails-Programms (B-064 › Kopfzeile): Titel, Reiter, rechte Leiste. */
export function AppHeader<P extends string>({ title, tabs, right }: Props<P>) {
  return (
    <header className="wk-header">
      <Text weight="bold" className="wk-header-title">
        {title}
      </Text>
      {tabs ? (
        <Tabs.List className="wk-header-tabs">
          {tabs.pages.map((p) => (
            <Tabs.Trigger key={p} value={p}>
              {tabs.labels[p]}
            </Tabs.Trigger>
          ))}
        </Tabs.List>
      ) : (
        <span className="wk-header-tabs" />
      )}
      <Flex align="center" gap="3" className="wk-header-right">
        {right}
      </Flex>
    </header>
  );
}
