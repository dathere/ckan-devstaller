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
          title="2.11.4"
          className={
            config.ckanVersion === "2.11.4"
              ? selectedCardClasses
              : "cursor-pointer"
          }
          onClick={() => {
            setConfig({ ...config, ckanVersion: "2.11.4" });
          }}
        ></Card>
        <Card
          icon={<SailboatIcon />}
          title="2.10.9"
          className={
            config.ckanVersion === "2.10.9"
              ? selectedCardClasses
              : "cursor-pointer"
          }
          onClick={() => {
            setConfig({ ...config, ckanVersion: "2.10.9" });
          }}
        ></Card>
      </Cards>
    </>
  );
}
