const { defineConfig } = require("@vue/cli-service");

module.exports = defineConfig({
  transpileDependencies: true,
  outputDir: "../vue_dist",
  indexPath: "sheet.html.tera",
  assetsDir: "vue",
  chainWebpack: config => {
    config
      .plugin('html')
      .tap(args => {
        args[0].inject = false;
        args[0].template = "public/sheet.html.tera";
        return args;
      })
  }
});
