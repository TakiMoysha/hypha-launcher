<script setup lang="ts">
import { TriangleAlertIcon } from "@lucide/vue";
import { RouterLink } from "vue-router";
</script>

<template>
  <div class="flex flex-col items-center">
    <TriangleAlertIcon class="mx-auto h-12 w-12 text-warning" aria-hidden="true" />

    <h2 class="mt-6 text-2xl font-bold">Invalid path - 404</h2>

    <p class="mt-4">We couldn't find the page you're looking for.</p>

    <RouterLink to="/">
      <button type="button" class="btn btn-primary mt-6">Back to Home</button>
    </RouterLink>

    <p class="mt-6 text-sm text-neutral-content">
      If You think this is a bug, you can create issue on
      <a href="#" class="underline hover:text-neutral">github page</a>.
    </p>
  </div>
</template>
