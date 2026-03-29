<script setup lang="ts">
import { invoke } from "@tauri-apps/api/core";
</script>

<template>
  <main class="flex min-h-screen flex-col bg-base-200">
    <div class="">
      <!-- palette -->
      <section class="flex flex-col bg-background mx-auto max-w-6xl p-6">
        <h4 class="mt-4 text-2xl text-base-content">Base</h4>
        <div class="palette-block text-primary-content bg-primary hover:bg-primary-focus">
          text-primary-content + bg-primary
        </div>
        <div class="palette-block text-secondary-content bg-secondary hover:bg-secondary-focus">
          text-secondary-content + bg-secondary
        </div>
        <div class="palette-block text-accent-content bg-accent hover:bg-accent-focus">
          text-accent-content + bg-accent
        </div>
        <div class="palette-block text-neutral-content bg-neutral hover:bg-neutral-focus">
          text-neutral-content + bg-neutral
        </div>

        <h4 class="mt-4 text-2xl text-base-content">Signals</h4>
        <div class="palette-block bg-info text-info-content">
          text-info-content + bg-info
        </div>
        <div class="palette-block bg-success text-success-content">
          text-success-content + bg-success
        </div>
        <div class="palette-block bg-warning text-warning-content">
          text-warning-content + bg-warning
        </div>
        <div class="palette-block bg-error text-error-content">
          text-error-content + bg-error
        </div>

        <h4 class="mt-4 text-2xl text-base-content">Background & Surface</h4>
        <div class="palette-block text-base-content bg-base-100">
          text-base-content + bg-base-100
        </div>
        <div class="palette-block text-base-content bg-base-200">
          text-base-content + bg-base-200
        </div>
        <div class="palette-block text-base-content bg-base-300">
          text-base-content + bg-base-300
        </div>
        <div class="palette-block text-base-content bg-base-400">
          text-base-content + bg-base-400
        </div>

        <h4 class="mt-4 text-2xl text-base-content">Fonts & Sizes</h4>
        <div class="palette-block text-base-content font-sans">
          Font Sans <span class="uppercase">UppserCase</span>
        </div>
        <div class="palette-block text-base-content font-mono">
          Font Mono <span class="uppercase">UppserCase</span>
        </div>
        <div class="palette-block text-base-content font-digital">
          Font Digital <span class="uppercase">UppserCase</span>
        </div>
      </section>
    </div>
    <footer class="shadow-sm bg-base-100">
      <div class="flex items-center justify-around justify-items-center max-w-6xl mx-auto p-4"></div>
    </footer>
  </main>
</template>

<style scoped>
@reference "tailwindcss";

.palette-block {
  @apply px-4 py-2 my-2 text-center border border-emerald-600/20;
}
</style>
