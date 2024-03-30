module.exports = function (eleventyConfig) {
  eleventyConfig.addPassthroughCopy({
    static: "static",
    "static/simple.css": "simple.css",
    "static/favicon.ico": "favicon.ico",
  });
  return {
    dir: {
      input: "pages",
      output: "dist",
    },
  };
};
