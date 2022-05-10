<script setup lang="ts">
import { useAppStore } from '@/stores/app'
import { useDocumentStore } from '@/stores/document'
import { openDocument } from '@/commands/app'

const app = useAppStore()
const document = useDocumentStore()

function o() {
  openDocument("example" + Math.random());
}
</script>

<template>
  <p>Current document: {{ document.source }}</p>
  <button @click="o">Open document</button>
  <p>Open documents:</p>
  <ul v-for=" path in app.documents">
    <li>{{ path }}</li>
  </ul>
</template>