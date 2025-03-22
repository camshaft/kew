export default function parsePages(input) {
  const lines = input[0].split("\n");

  let root = {
    path: "/",
    _id: "",
    pages: [],
  };

  let stack = [{ indent: -1, page: root }];

  for (let line of lines) {
    const trimmed = line.trimStart();

    if (!trimmed.length) continue;

    let { page: parent, indent: currentIndent } = stack[0];
    const indent = line.length - trimmed.length;

    while (indent <= currentIndent) {
      stack.shift();
      parent = stack[0].page;
      currentIndent = stack[0].indent;
    }

    const page = parseLink(trimmed, parent);
    parent.pages.push(page);

    const p = { page, indent };
    stack.unshift(p);
  }

// console.log(JSON.stringify(root.pages, null, '  '));

  return root.pages;
}

function parseLink(line, parent) {
  let [name, link] = line.trim().split("|");

  if (!link) link = toKebabCase(name);

  const _id = `${parent._id}${parent.pages.length + 1}.`;

  name = `${_id} ${name}`;

  const sep = parent.path.endsWith("/") ? "" : "/";

  return {
    name,
    _id,
    path: `/${link}`,
    pages: [],
    open: true,
  };
}

function toKebabCase(str) {
  return str.replace(/[ -]+/g, "-").toLowerCase();
}
