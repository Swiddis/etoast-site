const mathjax = require("eleventy-plugin-mathjax");
const syntaxHighlight = require("@11ty/eleventy-plugin-syntaxhighlight");

const markdownIt = require("markdown-it");
const markdownItFootnote = require("markdown-it-footnote");

module.exports = function (eleventyConfig) {
  eleventyConfig.addPassthroughCopy({
    static: "static",
    "static/favicon.ico": "favicon.ico",
  });

  eleventyConfig.setLibrary(
    "md",
    markdownIt({ html: true }).use(markdownItFootnote)
  );

  eleventyConfig.addPlugin(mathjax);
  eleventyConfig.addPlugin(syntaxHighlight);

  return {
    dir: {
      input: "pages",
      output: "dist",
    },
  };
};
