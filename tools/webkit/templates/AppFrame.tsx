import { Tabs, Theme } from '@radix-ui/themes';
import type { ReactNode } from 'react';
import { AppHeader, type HeaderTabs } from '../organisms';

interface Props<P extends string> {
  title: string;
  appearance: 'light' | 'dark';
  /** Reiter: die Seiten stehen dann als Tabs.Content in children. */
  tabs?: HeaderTabs<P> & { value: P; onChange: (page: P) => void };
  right?: ReactNode;
  /** Neben der Seite (rechts), z. B. der Chat des Studios. */
  aside?: ReactNode;
  /** Über allem (z. B. Hinweise unten rechts). */
  overlay?: ReactNode;
  /** true: Fenster fest, nur die Seite scrollt; sonst scrollt das Dokument und die Kopfzeile klebt oben. */
  fixed?: boolean;
  children: ReactNode;
}

/** Rahmen jedes Wails-Programms: Theme, Kopfzeile, Seite (Template im Atomic Design). */
export function AppFrame<P extends string>({ title, appearance, tabs, right, aside, overlay, fixed, children }: Props<P>) {
  const body = (
    <div className="wk-body">
      <main className="wk-page">{children}</main>
      {aside}
    </div>
  );
  const shell = `wk-shell${fixed ? ' wk-shell-fixed' : ''}`;
  const pick = (v: string) => {
    const next = tabs?.pages.find((p) => p === v);
    if (next) tabs?.onChange(next);
  };
  return (
    <Theme appearance={appearance}>
      {tabs ? (
        <Tabs.Root value={tabs.value} onValueChange={pick} className={shell}>
          <AppHeader title={title} tabs={tabs} right={right} />
          {body}
        </Tabs.Root>
      ) : (
        <div className={shell}>
          <AppHeader title={title} right={right} />
          {body}
        </div>
      )}
      {overlay}
    </Theme>
  );
}
