<script setup lang="ts">
import { useApp } from '@/stores/app'
import { useDocument } from '@/stores/document'

const app = useApp()
const document = useDocument()
</script>

<template>
  Oink oink
  Opened:
  <div v-for="path in app.documents">
    {{ path }}
  </div>
  Editing {{ document.source }}
</template>