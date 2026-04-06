<script setup lang="ts">
import { SettingsIcon } from "@lucide/vue";
import { useTemplateRef, ref } from "vue";
import useRustABI from "@/composables/useRustABI";
import * as Sentry from "@sentry/vue";

const modalStartupParams = useTemplateRef("modal_startup_params");
const runGameArgsInput = ref([]);

const { runGameClient } = useRustABI();

const runGameClientHandler = (event: Event) => {
  runGameClient(runGameArgsInput.value);
};

const send_log = () => {
  Sentry.logger.warn("User triggered test log", { log_source: "sentry_test" });
};
</script>

<template>
  <div>
    <div class="join">
      <button class="join-item btn bg-primary" type="button" name="start-client" @click="runGameClientHandler">
        Run Client
      </button>
      <button class="join-item btn bg-primary" type="button" name="start-params"
        @click="modalStartupParams?.showModal()">
        <SettingsIcon width="24" height="24"></SettingsIcon>
      </button>
    </div>

    <dialog ref="modal_startup_params" class="modal">
      <div class="modal-box">
        <h3 class="text-lg font-bold">Startup Params</h3>

        <div class="grid grid-cols-2">
          <div class="relative col-span-2">
            <label for="startup-params" class="leading-7 text-sm text-gray-600">Running Params</label>
            <input id="startup-params" name="startup-params" type="text" v-model="runGameArgsInput"
              placeholder="Type here" class="input input-bordered w-full duration-200 ease-in-out" />
          </div>
        </div>

        <div class="modal-action">
          <form method="dialog">
            <button class="btn">Save</button>
          </form>
        </div>
      </div>
      <form method="dialog" class="modal-backdrop">
        <button>close</button>
      </form>
    </dialog>
  </div>
</template>
