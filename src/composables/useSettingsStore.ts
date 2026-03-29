import { reactive } from "vue";

const settings = reactive({
  args: "",
  setArgs: (args: string) => (settings.args = args),
});

/** Loading settings from tauri.rs (system files/cache) when loading app and consistently updating */
export default () => { };
