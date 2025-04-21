import Content from "./coffee.mdx";
import Page from "@/Page.tsx";

export default function () {
  return <Page>{({ components }) => <Content components={components} />}</Page>;
}
