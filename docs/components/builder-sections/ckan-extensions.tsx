import defaultMdxComponents from "fumadocs-ui/mdx";
import { SailboatIcon, TerminalSquareIcon } from "lucide-react";
import { Config, selectedCardClasses } from "../builder";
import { toast } from "sonner";

const getExtensionClassName = (config: Config, extensionName: string) => {
  return config.extensions.includes(extensionName) ? selectedCardClasses : "";
};

const updateExtensions = (
  config: Config,
  setConfig: any,
  extensions: string[] | string,
  mode?: "add" | "remove",
) => {
  const extensionsArray = Array.isArray(extensions) ? extensions : [extensions];
  if (mode === "add") {
    setConfig({
      ...config,
      extensions: [...new Set([...config.extensions, ...extensionsArray])],
    });
    return;
  }
  for (const extensionName of extensionsArray) {
    if (config.extensions.includes(extensionName))
      setConfig({
        ...config,
        extensions: config.extensions.filter(
          (extension) => extension !== extensionName,
        ),
      });
    else if (!config.extensions.includes(extensionName))
      setConfig({
        ...config,
        extensions: [...config.extensions, extensionName],
      });
  }
};

export default function CKANExtensionsBuilderSection({
  config,
  setConfig,
}: {
  config: Config;
  setConfig: any;
}) {
  const { Card, Cards } = defaultMdxComponents;

  return (
    <>
      <h3>CKAN extensions</h3>
      <Cards>
        <Card
          className={getExtensionClassName(config, "ckanext-scheming")}
          icon={<TerminalSquareIcon />}
          title="ckanext-scheming"
          onClick={() => {
            if (
              config.extensions.includes("DataPusher+") &&
              config.extensions.includes("ckanext-scheming")
            ) {
              toast.error(
                "You cannot remove the ckanext-scheming extension because the DataPusher+ extension depends on it.",
              );
              return;
            }
            updateExtensions(config, setConfig, "ckanext-scheming");
          }}
        ></Card>
        <Card
          className={getExtensionClassName(config, "DataStore")}
          icon={<TerminalSquareIcon />}
          title="DataStore"
          onClick={() => {
            if (
              config.extensions.includes("DataPusher+") &&
              config.extensions.includes("DataStore")
            ) {
              toast.error(
                "You cannot remove the DataStore extension because the DataPusher+ extension depends on it.",
              );
              return;
            }
            updateExtensions(config, setConfig, "DataStore");
          }}
        ></Card>
        <Card
          className={getExtensionClassName(config, "DataPusher+")}
          icon={<TerminalSquareIcon />}
          title="DataPusher+"
          onClick={() => {
            if (config.extensions.includes("DataPusher+")) {
              updateExtensions(config, setConfig, "DataPusher+");
            } else {
              updateExtensions(
                config,
                setConfig,
                ["DataPusher+", "ckanext-scheming", "DataStore"],
                "add",
              );
            }
          }}
        ></Card>
      </Cards>
    </>
  );
}
