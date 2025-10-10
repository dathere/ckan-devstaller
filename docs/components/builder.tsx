import { CodeBlock, Pre } from "fumadocs-ui/components/codeblock";
import defaultMdxComponents from "fumadocs-ui/mdx";
import {
  BarChartBigIcon,
  BlocksIcon,
  SailboatIcon,
  TerminalSquareIcon,
} from "lucide-react";

export default function Builder() {
  const { Card, Cards } = defaultMdxComponents;
  return (
    <div className="grid grid-cols-3 gap-4">
      <div className="col-span-1 border-r-2 pr-4">
        <div className="fixed">
          <h2>ckan-devstaller command</h2>
          <CodeBlock title="Installation command">
            <Pre className="text-wrap pl-4 max-w-[21rem]">
              ./ckan-devstaller
            </Pre>
          </CodeBlock>
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
          <Card
            icon={<BarChartBigIcon />}
            title="CKAN, DataStore, ckanext-scheming, and DataPusher+ extensions"
          >
            Installs CKAN, the DataStore extension, the ckanext-scheming
            extension, and the DataPusher+ extension.
          </Card>
        </Cards>
        <h3>CKAN version</h3>
        <Cards>
          <Card icon={<SailboatIcon />} title="2.11.3"></Card>
          <Card icon={<SailboatIcon />} title="2.10.8"></Card>
          <Card icon={<SailboatIcon />} title="Other"></Card>
        </Cards>
        <h3>SSH capability</h3>
        <Cards>
          <Card icon={<TerminalSquareIcon />} title="Enable SSH">
            Installs openssh-server and net-tools.
          </Card>
        </Cards>
        <h3>CKAN extensions</h3>
        <Cards>
          <Card icon={<TerminalSquareIcon />} title="ckanext-scheming"></Card>
          <Card icon={<TerminalSquareIcon />} title="ckanext-gztr"></Card>
          <Card icon={<TerminalSquareIcon />} title="DataStore"></Card>
          <Card icon={<TerminalSquareIcon />} title="DataPusher+"></Card>
          <Card icon={<TerminalSquareIcon />} title="ckanext-spatial"></Card>
        </Cards>
      </div>
    </div>
  );
}
