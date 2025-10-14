import defaultMdxComponents from "fumadocs-ui/mdx";
import { SailboatIcon } from "lucide-react";
import { selectedCardClasses } from "../builder";

export default function CKANVersionBuilderSection({ config, setConfig }: any) {
  const { Card, Cards } = defaultMdxComponents;

  return (
    <>
      <h3>CKAN version</h3>
      <Cards>
        <Card
          icon={<SailboatIcon />}
          title="2.11.3"
          className={
            config.ckanVersion === "2.11.3"
              ? selectedCardClasses
              : "cursor-pointer"
          }
          onClick={() => {
            setConfig({ ...config, ckanVersion: "2.11.3" });
          }}
        ></Card>
        <Card
          icon={<SailboatIcon />}
          title="2.10.8"
          className={
            config.ckanVersion === "2.10.8"
              ? selectedCardClasses
              : "cursor-pointer"
          }
          onClick={() => {
            setConfig({ ...config, ckanVersion: "2.10.8" });
          }}
        ></Card>
      </Cards>
    </>
  );
}
