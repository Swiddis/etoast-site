module.exports = function (eleventyConfig) {
  eleventyConfig.addPassthroughCopy({
    static: "static",
    "static/favicon.ico": "favicon.ico",
  });
};
