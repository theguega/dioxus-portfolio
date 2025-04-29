# Dioxus Portfolio

This is a portfolio website built with Dioxus.
The site is available at [https://theguega.github.io/dioxus-portfolio/](https://theguega.github.io/dioxus-portfolio/).

# Publishing

To publish the site, run the following command:

```bash
dx build --release
cp -r target/dx/dioxus-portfolio/release/web/public/* docs/
cp docs/index.html docs/404.html
```
