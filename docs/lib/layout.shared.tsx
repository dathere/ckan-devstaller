import type { BaseLayoutProps } from "fumadocs-ui/layouts/shared";
import { SailboatIcon } from "lucide-react";

/**
 * Shared layout configurations
 *
 * you can customise layouts individually from:
 * Home Layout: app/(home)/layout.tsx
 * Docs Layout: app/docs/layout.tsx
 */
export function baseOptions(): BaseLayoutProps {
  return {
    nav: {
      title: (
        <>
          <SailboatIcon />
          ckan-devstaller
        </>
      ),
    },
    // see https://fumadocs.dev/docs/ui/navigation/links
    links: [],
    githubUrl: "https://github.com/dathere/ckan-devstaller",
  };
}
