import mathjax from "eleventy-plugin-mathjax";
import syntaxHighlight from "@11ty/eleventy-plugin-syntaxhighlight";
import purgeCss from "eleventy-plugin-purgecss";
import esbuild from "esbuild";

import markdownIt from "markdown-it";
import markdownItFootnote from "markdown-it-footnote";

const config = (eleventyConfig) => {
  eleventyConfig.on('eleventy.before', async () => {
    await esbuild.build({
      entryPoints: ["web/scripts/*"],
      outdir: "dist/scripts",
      platform: "browser",
      target: "esnext",
    })
  });

  eleventyConfig.addPassthroughCopy({
    "web/static": "static",
    "web/static/favicon.ico": "favicon.ico",
    "web/static/robots.txt": "robots.txt"
  });

  eleventyConfig.addCollection("writing", (collectionApi) => {
    return collectionApi.getFilteredByGlob("web/pages/writing/*.md")
      .filter(page => page.data.author_date) // Missing dates are WIP, don't show
      .sort((a, b) => new Date(b.data.author_date) - new Date(a.data.author_date));
  });

  eleventyConfig.setLibrary(
    "md",
    markdownIt({ html: true }).use(markdownItFootnote)
  );

  eleventyConfig.addPlugin(mathjax);
  eleventyConfig.addPlugin(syntaxHighlight);
  eleventyConfig.addPlugin(purgeCss, {
    config: {
      content: ["./dist/**/*.html"],
      css: ["./dist/**/*.css"],
    },
    quiet: true,
  });

  return {
    dir: {
      input: "web/pages",
      output: "dist",
    },
  };
};

export default config;
