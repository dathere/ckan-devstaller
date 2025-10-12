"use client";

import { CodeBlock, Pre } from "fumadocs-ui/components/codeblock";
import defaultMdxComponents from "fumadocs-ui/mdx";
import {
  BarChartBigIcon,
  BlocksIcon,
  SailboatIcon,
  TerminalSquareIcon,
} from "lucide-react";
import { useState } from "react";

type Config = {
  preset: string | undefined;
  ckanVersion: string;
  extensions: string[];
};

export default function Builder() {
  const { Card, Cards } = defaultMdxComponents;
  const [command, setCommand] = useState("./ckan-devstaller");
  const [config, setConfig] = useState<Config>({
    preset: undefined,
    ckanVersion: "2.11.3",
    extensions: [],
  });

  return (
    <div className="grid grid-cols-3 gap-4">
      <div className="col-span-1 border-r-2 pr-4">
        <div className="sticky top-8">
          <h2>ckan-devstaller command</h2>
          <CodeBlock title="Installation command">
            <Pre className="text-wrap pl-4 max-w-[21rem]">{command}</Pre>
          </CodeBlock>
          <h2>Selected configuration</h2>
          <div>
            <strong>CKAN version</strong>: 2.11.3
            <br />
            <br />
            <strong>Extensions:</strong>
            <ul>
              <li>DataStore</li>
              <li>ckanext-scheming</li>
              <li>DataPusher+</li>
            </ul>
            <strong>Extra features:</strong>
            <ul>
              <li>Enable SSH</li>
            </ul>
          </div>
        </div>
      </div>
      <div className="col-span-2">
        <h2>Configuration options</h2>
        <h3>Presets</h3>
        <Cards className="grid-cols-2">
          <Card
            className="bg-blue-100 dark:bg-blue-950 border-blue-300 dark:border-blue-900 border-2"
            icon={<SailboatIcon />}
            title="CKAN-only"
          >
            Installs CKAN with ckan-compose.
          </Card>
          <Card icon={<BlocksIcon />} title="CKAN and the DataStore extension">
            Installs CKAN and the DataStore extension.
          </Card>
          <Card icon={<BarChartBigIcon />} title="datHere Default">
            Installs CKAN, the DataStore extension, the ckanext-scheming
            extension, and the DataPusher+ extension.
          </Card>
        </Cards>
        <h3>CKAN version</h3>
        <Cards>
          <Card icon={<SailboatIcon />} title="2.11.3"></Card>
          <Card icon={<SailboatIcon />} title="2.10.8"></Card>
          <Card
            icon={<SailboatIcon />}
            title="Install a different version"
          ></Card>
          <Card
            icon={<SailboatIcon />}
            title="Clone from remote Git repository"
          ></Card>
        </Cards>
        <h3>CKAN extensions</h3>
        <Cards>
          <Card icon={<TerminalSquareIcon />} title="ckanext-scheming"></Card>
          <Card icon={<TerminalSquareIcon />} title="ckanext-gztr"></Card>
          <Card icon={<TerminalSquareIcon />} title="DataStore"></Card>
          <Card icon={<TerminalSquareIcon />} title="DataPusher+"></Card>
          <Card icon={<TerminalSquareIcon />} title="ckanext-spatial"></Card>
          <Card icon={<TerminalSquareIcon />} title="Custom extension"></Card>
        </Cards>
        <h3>Extra features</h3>
        <Cards>
          <Card icon={<TerminalSquareIcon />} title="Enable SSH">
            Installs openssh-server and net-tools.
          </Card>
          <Card icon={<TerminalSquareIcon />} title="Run a Bash script">
            Run a Bash script before or after any step during the installation.
          </Card>
        </Cards>
      </div>
    </div>
  );
}
