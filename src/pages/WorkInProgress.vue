<script setup lang="ts">
import { OctagonAlertIcon } from "@lucide/vue";
</script>

<template>
  <div class="flex flex-col items-center">
    <OctagonAlertIcon class="mx-auto h-12 w-12 text-warning" aria-hidden="true" />

    <h2 class="mt-6 text-2xl font-bold">Work in progress</h2>

    <p class="mt-4">This page or feature is not ready yet.</p>

    <p class="mt-6 text-sm text-neutral-content">
      If You think this is a bug, you can create issue on
      <a href="#" class="underline hover:text-neutral">github page</a>.
    </p>
  </div>
</template>
