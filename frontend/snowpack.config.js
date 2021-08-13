/** @type {import("snowpack").SnowpackUserConfig } */
module.exports = {
  mount: {
    public: "/",
    src: "/dist",
  },

  optimize: {
    bundle: true,
    minify: true,
    target: 'es2018',
  },

  plugins: [
    "@snowpack/plugin-svelte",
    "@snowpack/plugin-postcss",
    "@snowpack/plugin-dotenv",
    "@snowpack/plugin-typescript",
  ],

  packageOptions: {
    installTypes: true,
    /* ... */
  },
  devOptions: {
    port: 8081,
  },
  buildOptions: {
    /* ... */
  },

  alias: {
    /* ... */
  },

};
