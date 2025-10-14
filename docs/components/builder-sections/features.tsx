import defaultMdxComponents from "fumadocs-ui/mdx";
import { SailboatIcon, TerminalSquareIcon } from "lucide-react";
import { Config, selectedCardClasses } from "../builder";

const getFeatureClassName = (config: Config, featureName: string) => {
  return config.features.includes(featureName) ? selectedCardClasses : "";
};

const updateFeatures = (
  config: Config,
  setConfig: any,
  featureName: string,
) => {
  if (config.features.includes(featureName))
    setConfig({
      ...config,
      features: config.features.filter((feature) => feature !== featureName),
    });
  else setConfig({ ...config, features: [...config.features, featureName] });
};

export default function FeaturesBuilderSection({
  config,
  setConfig,
}: {
  config: Config;
  setConfig: any;
}) {
  const { Card, Cards } = defaultMdxComponents;

  return (
    <>
      <h3>Features</h3>
      <Cards>
        <Card
          className={getFeatureClassName(config, "enable-ssh")}
          icon={<TerminalSquareIcon />}
          title="Enable SSH"
          onClick={() => {
            updateFeatures(config, setConfig, "enable-ssh");
          }}
        >
          Installs the openssh-server package.
        </Card>
      </Cards>
    </>
  );
}
