<script setup lang="ts">
import { openFiles } from '@/api/local'
import Workbench from '@/components/Workbench.vue';
import ContentPane from '@/components/ContentPane.vue'

</script>

<template>

  <div class="flex flex-nowrap flex-col h-screen w-screen overflow-clip select-none">

    <button @click="openFiles" class="place-self-start">Open document</button>

    <div class="flex-1 min-h-0 flex flex-row">
      <ContentPane class="basis-96 min-w-0" />
      <Workbench class="flex-1" />
    </div>
    <div class="p-4 basis-60 bg-stone-800">
      Beep boop I'm a timeline
    </div>
  </div>
</template>
