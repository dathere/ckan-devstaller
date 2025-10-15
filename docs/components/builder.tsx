"use client";

import { CodeBlock, Pre } from "fumadocs-ui/components/codeblock";
import defaultMdxComponents from "fumadocs-ui/mdx";
import {
  BarChartBigIcon,
  BlocksIcon,
  SailboatIcon,
  TerminalSquareIcon,
} from "lucide-react";
import { useEffect, useState } from "react";
import PresetsBuilderSection from "./builder-sections/presets";
import CKANVersionBuilderSection from "./builder-sections/ckan-version";
import CKANExtensionsBuilderSection from "./builder-sections/ckan-extensions";
import FeaturesBuilderSection from "./builder-sections/features";

export type Config = {
  preset: string | undefined;
  ckanVersion: string;
  extensions: string[];
  features: string[];
};

export const selectedCardClasses =
  "bg-blue-100 dark:bg-blue-950 border-blue-300 dark:border-blue-900 border-2";

export default function Builder() {
  const { Card, Cards } = defaultMdxComponents;
  const [command, setCommand] = useState("./ckan-devstaller");
  const [config, setConfig] = useState<Config>({
    preset: "ckan-only",
    ckanVersion: "2.11.3",
    extensions: [],
    features: [],
  });

  // Update command string when user changes configuration
  useEffect(() => {
    const ckanVersionString = `--ckan-version ${config.ckanVersion}`;
    const extensionsString =
      config.extensions.length > 0
        ? ` \\\n--extensions ${config.extensions.join(" ")}`
        : undefined;
    const featuresString =
      config.features.length > 0
        ? ` \\\n--features ${config.features.join(" ")}`
        : undefined;
    setCommand(
      `./ckan-devstaller \\
${ckanVersionString}${extensionsString ? extensionsString : ""}${featuresString ? featuresString : ""}`,
    );
  }, [config]);

  return (
    <div className="md:grid md:grid-cols-3 md:gap-4">
      <div className="md:col-span-1 md:border-r-2 md:pr-4">
        <div className="sticky top-8">
          <h2>ckan-devstaller command</h2>
          <CodeBlock title="Installation command">
            <Pre className="text-wrap pl-4 max-w-[21rem]">{command}</Pre>
          </CodeBlock>
          <h2>Selected configuration</h2>
          <div>
            <strong>CKAN version</strong>: {config.ckanVersion}
            <br />
            <br />
            {config.extensions.length > 0 && (
              <>
                <strong>Extensions:</strong>
                <ul>
                  {config.extensions.map((extension) => (
                    <li key={extension}>{extension}</li>
                  ))}
                </ul>
              </>
            )}
            {config.features.length > 0 && (
              <>
                <strong>Features:</strong>
                <ul>
                  {config.features.map((feature) => (
                    <li key={feature}>{feature}</li>
                  ))}
                </ul>
              </>
            )}
          </div>
        </div>
      </div>
      <div className="md:col-span-2">
        <h2>Configuration options</h2>
        <PresetsBuilderSection config={config} setConfig={setConfig} />
        <CKANVersionBuilderSection config={config} setConfig={setConfig} />
        <CKANExtensionsBuilderSection config={config} setConfig={setConfig} />
        <FeaturesBuilderSection config={config} setConfig={setConfig} />
      </div>
    </div>
  );
}
