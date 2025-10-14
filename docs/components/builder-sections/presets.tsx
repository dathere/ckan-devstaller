import { Config, selectedCardClasses } from "../builder";
import { BarChartBigIcon, SailboatIcon } from "lucide-react";
import defaultMdxComponents from "fumadocs-ui/mdx";

export default function PresetsBuilderSection({
  config,
  setConfig,
}: {
  config: Config;
  setConfig: any;
}) {
  const { Card, Cards } = defaultMdxComponents;

  return (
    <>
      <h3>Presets</h3>
      <Cards className="grid-cols-2">
        <Card
          className={
            config.preset === "ckan-only" &&
            config.extensions.length === 0 &&
            config.features.length === 0
              ? selectedCardClasses
              : "cursor-pointer"
          }
          icon={<SailboatIcon />}
          title="CKAN-only"
          onClick={() => {
            setConfig({
              ...config,
              preset: "ckan-only",
              extensions: [],
              features: [],
            });
          }}
        >
          Installs CKAN with ckan-compose. No CKAN extensions and extra features
          are installed.
        </Card>
        <Card
          className={
            config.preset === "dathere-default"
              ? selectedCardClasses
              : "cursor-pointer"
          }
          icon={<BarChartBigIcon />}
          title="datHere Default"
          onClick={() => {
            setConfig({
              ...config,
              preset: "dathere-default",
              ckanVersion: "2.11.3",
              extensions: ["ckanext-scheming", "DataStore", "DataPusher+"],
              features: ["enable-ssh"],
            });
          }}
        >
          datHere's default preset featuring the DataPusher+ extension.
        </Card>
      </Cards>
    </>
  );
}
