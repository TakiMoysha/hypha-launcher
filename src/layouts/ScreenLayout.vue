<script setup lang="ts">
import DiscordIcon from "@/assets/icons/discord.svg?raw";
import GithubIcon from "@/assets/icons/github.svg?raw";
import RunClientButtonGroup from "@/components/RunClientButtonGroup.vue";
</script>

<template>
  <div class="flex flex-col h-screen bg-base-200">
    <header class="p-4 shrink-0 hidden">
      <div class="flex justify-between">
        Header ?
        <div>
          <div class="hidden sm:hidden md:hidden lg:block">large size</div>
          <div class="hidden sm:hidden md:block lg:hidden">medium size</div>
          <div class="hidden sm:block md:hidden lg:hidden">small size</div>
        </div>
      </div>
    </header>

    <div class="flex flex-1 min-h-0">
      <main class="flex-1 p-4 overflow-auto mx-1 my-2">
        <slot name="content"> Container Area </slot>
      </main>


      <aside class="w-1/6 p-4 shrink-0 overflow-auto mx-1 my-2 flex flex-col gap-2">
        <nav class="navbar w-full bg-base-100">

        </nav>

        <router-link class="hover:underline border-b-2 p-2" to="/">home</router-link>
        <router-link class="hover:underline border-b-2 p-2" to="/dev">dev</router-link>
        <router-link class="hover:underline border-b-2 p-2" to="/dev/server">server</router-link>
        <router-link class="hover:underline border-b-2 p-2" to="/debug/style-preview">style</router-link>
        <router-link class="hover:underline border-b-2 p-2" to="/not-found-page">404</router-link>
      </aside>
    </div>

    <footer class="p-4 shrink-0 min-h-16">
      <div class="flex justify-between">
        <run-client-button-group />
        <div class="flex gap-2">
          <button class="btn btn-ghost" v-html="DiscordIcon"></button>
          <button class="btn btn-ghost" v-html="GithubIcon"></button>
        </div>
      </div>
    </footer>
  </div>
</template>
