import Content from "./hospital.mdx";
import Page from "@/Page.tsx";

export default function () {
  return <Page>{({ components }) => <Content components={components} />}</Page>;
}
