#!/usr/bin/env bash

set -Eeuo pipefail

cargo build
cargo doc
git rm -r docs || echo "Apparently first run .. ignoring error"
mv target/doc docs
echo '<meta http-equiv="refresh" content="0; url=stack_trait/index.html">' > docs/index.html
echo "" > docs/.nojekyll
cat >  docs/_config.yml <<EOF
title: Cayman theme
description: Cayman is a clean, responsive theme for GitHub Pages.
show_downloads: true
google_analytics:
theme: jekyll-theme-cayman
EOF
mkdir -p docs/assets/css
cat > docs/assets/css/style.scss <<EOF
---
---

@import 'jekyll-theme-cayman';
EOF
git add docs
git add _config.yml
git add docs/assets/css/style.scss

