<script setup lang="ts">
import { ExclamationTriangleIcon } from "@heroicons/vue/24/outline";
import { RouterLink } from "vue-router";
</script>

<template>
  <div class="flex min-h-full items-center justify-center">
    <div class="max-w-md text-center">
      <ExclamationTriangleIcon class="mx-auto h-12 w-12 text-warning" aria-hidden="true" />

      <h2 class="mt-6 text-2xl font-bold">Invalid path - 404</h2>

      <p class="mt-4 text-pretty">
        We couldn't find the page you're looking for.
      </p>

      <RouterLink to="/"><button type="button"></button></RouterLink>

      <button type="button"
        class="mt-6 block w-full rounded-lg bg-primary px-6 py-3 text-sm font-medium text-primary-content transition-colors hover:bg-accent">
        Go to home page
      </button>

      <p class="mt-6 text-sm text-neutral-content">
        If You think this is a bug, you can create issue on
        <a href="#" class="underline hover:text-neutral">github page</a>.
      </p>
    </div>
  </div>
</template>
