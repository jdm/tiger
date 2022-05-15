<template>
	<div class="h-full">
		<div class="h-full graph-paper">
			<div class="flex flex-row ">
				<PaneTab :closeable="true" v-for="document in app.documents" @select="focusDocument(document.path)"
					@close="closeDocument(document.path)" :selected="document.path == app.currentDocumentPath">
					{{ document.name }}
				</PaneTab>
				<div class="flex-1 bg-zinc-900" />
			</div>
		</div>
	</div>
</template>

<script setup lang="ts">
import { closeDocument, focusDocument } from '@/api/app'
import { useAppStore } from '@/stores/app'
import PaneTab from '@/components/pane/PaneTab.vue'
const app = useAppStore()
</script>

<style scoped>
.graph-paper {
	background:
		linear-gradient(-90deg, theme('colors.neutral.700') 1px, transparent 1px),
		linear-gradient(0deg, theme('colors.neutral.700') 1px, transparent 1px),
		linear-gradient(-90deg, theme('colors.neutral.800') 1px, transparent 1px),
		linear-gradient(0deg, theme('colors.neutral.800') 1px, transparent 1px),
		theme('colors.neutral.900');
	background-size:
		128px 128px,
		128px 128px,
		16px 16px,
		16px 16px;
	background-position: -64px -64px, -64px -64px, -64px -64px;
}
</style>