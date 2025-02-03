import init, { create_terp } from "./pkg/terp.js";

export default class Terp {
  #terp;
  #terpLoader;

  async init() {
    await init();
    return new Promise((resolve) => {
      this.#terp = create_terp((loader) => {
        this.#terpLoader = loader;
        resolve();
      });
      try {
        this.#terp.run();
      } catch (error) {
        if (!error.message.startsWith("Using exceptions for control flow")) {
          throw error;
        }
      }
    });
  }

  load(project) {
    this.#terpLoader.load(project);
  }
}
