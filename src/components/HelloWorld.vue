<script setup lang="ts">
import { useAppStore } from '@/stores/app'
import { openFiles } from '@/commands/local'
import TabList from '@/components/tabs/TabList.vue'
import Tab from '@/components/tabs/Tab.vue'

const app = useAppStore()

</script>

<template>
  <button @click="openFiles">Open document</button>

  <TabList class="">
    <Tab v-for="document in app.documents" :selected="document.path == app.currentDocumentPath">
      {{ document.name }}
    </Tab>
  </TabList>

  <p>Current document: {{ app.currentDocument?.path }}</p>
  <div v-if="app.currentDocument">
    <ul v-for="frame in app.currentDocument.sheet.frames">
      <li>{{ frame }}</li>
    </ul>
  </div>
</template>
