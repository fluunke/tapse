/** @type {import("snowpack").SnowpackUserConfig } */
module.exports = {
  mount: {
    public: "/",
    src: "/dist",
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
