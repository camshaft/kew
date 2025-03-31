# Runs the dev server
dev: build-crates
    @overmind start

dev-js:
    @deno run -A --node-modules-dir npm:vite

dev-rs:
    @cargo watch --watch='crates' -x 'xtask'

[group('build')]
build: check-tsc build-crates build-web build-static build-md

[group('build')]
build-crates:
    @cargo xtask

[group('build')]
build-web:
    @NODE_ENV=production deno run -A --node-modules-dir npm:vite build --outDir target/book/web

[group('build')]
build-static:
    @NODE_ENV=development VITE_SSR_TARGET=static deno run -A --node-modules-dir ./bin/render static

[group('build')]
build-md:
    @NODE_ENV=development VITE_SSR_TARGET=md bin/render md

[group('build')]
build-pdf:
    @mkdir -p target/book/pdf
    @pandoc target/book/md/index.md --output target/book/pdf/kew.pdf

[group('build')]
build-ghp: build
    @rm -rf target/book/kew
    @cp -r target/book/static target/book/kew
    @touch target/book/kew/.nojekyll
    #@cp target/book/pdf/kew.pdf target/book/kew/kew.pdf

[group('checks')]
check-tsc:
    @deno run -A --node-modules-dir npm:typescript/tsc --noEmit
