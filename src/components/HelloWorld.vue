<script setup lang="ts">
import { useAppStore } from '@/stores/app'
import { openFiles } from '@/commands/local'
import TabList from '@/components/tabs/TabList.vue'
import Tab from '@/components/tabs/Tab.vue'
import Workbench from '@/components/Workbench.vue';

const app = useAppStore()

</script>

<template>

  <div class="flex flex-nowrap flex-col h-screen w-screen overflow-clip select-none">

    <button @click="openFiles" class="place-self-start">Open document</button>

    <TabList>
      <Tab v-for="document in app.documents" :selected="document.path == app.currentDocumentPath">
        {{ document.name }}
      </Tab>
    </TabList>

    <div class="flex-1 min-h-0 flex flex-row">
      <div class="flex-1 flex flex-col">
        <Workbench class="flex-1">
        </Workbench>

        <div class="basis-60 flex flex-col">
          <h2 class="px-3 py-1 bg-zinc-800 text-slate-50 font-semibold">
            Timeline</h2>
          <div class="flex-1 px-4 py-2 overflow-y-auto bg-neutral-900 border-r border-zinc-800 text-gray-400">
            Beep boop I'm a timeline
          </div>
        </div>
      </div>
      <div class="basis-96 min-w-0 flex flex-col">
        <h2 class="px-3 py-1 bg-zinc-800 text-slate-50 font-semibold">
          Content</h2>
        <div class="flex-1 px-4 py-2 overflow-y-auto bg-neutral-900 text-zinc-300">
          <ul v-if="app.currentDocument">
            <li class="overflow-x-hidden text-ellipsis" v-for="frame in app.currentDocument.sheet.frames">{{ frame }}
            </li>
          </ul>
        </div>
      </div>
    </div>

  </div>

</template>
