const mathjaxPlugin = require("eleventy-plugin-mathjax");

module.exports = function (eleventyConfig) {
  eleventyConfig.addPassthroughCopy({
    static: "static",
    "static/simple.css": "simple.css",
    "static/favicon.ico": "favicon.ico",
  });

  eleventyConfig.addPlugin(mathjaxPlugin);

  return {
    dir: {
      input: "pages",
      output: "dist",
    },
  };
};
