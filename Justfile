# Runs the dev server
dev: build-crates
    @overmind start

dev-js:
    @npm run dev

dev-rs:
    @cargo watch --watch='crates' --watch='xtask' -x 'xtask'

dev-npm:
    @cargo watch --watch='package.json' -s 'npm i'

[group('build')]
build: check-tsc build-crates build-web build-static build-md

[group('build')]
build-crates:
    @cargo xtask

[group('build')]
build-web:
    @npm run build

[group('build')]
build-static:
    @NODE_ENV=development VITE_SSR_TARGET=static bin/render static

[group('build')]
build-md:
    @NODE_ENV=development VITE_SSR_TARGET=md bin/render md

[group('build')]
build-pdf:
    @mkdir -p target/book/pdf
    @pandoc target/book/md/index.md --output target/book/pdf/kew.pdf

[group('build')]
build-ghp: npmi build
    @rm -rf target/book/ghp
    @cp -r target/book/static target/book/ghp
    @touch target/book/ghp/.nojekyll
    #@cp target/book/pdf/kew.pdf target/book/ghp/kew.pdf

npmi:
    npm i

[group('checks')]
check-tsc:
    @npm run check-tsc
