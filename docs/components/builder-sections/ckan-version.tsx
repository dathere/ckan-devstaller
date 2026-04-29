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
          title="2.11.5"
          className={
            config.ckanVersion === "2.11.5"
              ? selectedCardClasses
              : "cursor-pointer"
          }
          onClick={() => {
            setConfig({ ...config, ckanVersion: "2.11.5" });
          }}
        ></Card>
        <Card
          icon={<SailboatIcon />}
          title="2.10.10"
          className={
            config.ckanVersion === "2.10.10"
              ? selectedCardClasses
              : "cursor-pointer"
          }
          onClick={() => {
            setConfig({ ...config, ckanVersion: "2.10.10" });
          }}
        ></Card>
      </Cards>
    </>
  );
}
