import init, { create_player, create_editor } from "./pkg/terp.js";

export default class Terp {
  #terp;
  #terpLoader;

  async init(isEditor) {
    const create = isEditor ? create_editor : create_player;
    await init();
    return new Promise((resolve) => {
      this.#terp = create((loader) => {
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
