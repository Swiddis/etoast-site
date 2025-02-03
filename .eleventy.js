const mathjax = require("eleventy-plugin-mathjax");
const syntaxHighlight = require("@11ty/eleventy-plugin-syntaxhighlight");
const purgeCss = require("eleventy-plugin-purgecss");

const markdownIt = require("markdown-it");
const markdownItFootnote = require("markdown-it-footnote");

module.exports = function (eleventyConfig) {
  eleventyConfig.addPassthroughCopy({
    static: "static",
    "static/favicon.ico": "favicon.ico",
    "static/robots.txt": "robots.txt"
  });

  eleventyConfig.addCollection("writing", function (collectionApi) {
    return collectionApi.getFilteredByGlob("pages/writing/*.md")
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
    config: './purgecss.config.js',
    quiet: false,
  });

  return {
    dir: {
      input: "pages",
      output: "dist",
    },
  };
};
