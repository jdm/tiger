<script setup lang="ts">
import { openFiles } from '@/api/local'
import Workbench from '@/components/Workbench.vue';
import ContentPane from '@/components/ContentPane.vue'

</script>

<template>

  <div class="flex flex-nowrap flex-col h-screen w-screen space-y-5 p-5 bg-plastic-900 overflow-clip select-none">

    <button @click="openFiles" class="place-self-start">Open document</button>

    <div class="flex-1 min-h-0 flex flex-row space-x-5">
      <ContentPane class="basis-96 min-w-0 rounded-sm outline outline-plastic-600" />
      <Workbench class="flex-1 rounded-sm outline outline-plastic-600" />
    </div>

    <div class="p-4 basis-60 bg-plastic-700 rounded-sm outline outline-plastic-600">
      Beep boop I'm a timeline
    </div>

  </div>
</template>
