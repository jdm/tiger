<template>

	<div
		class="flex flex-col pb-4 space-y-4 bg-plastic-700 rounded-sm outline outline-plastic-600 shadow-lg shadow-black/30">
		<div class="flex flex-row bg-plastic-900">
			<PaneTab :selected="true">Hitboxes</PaneTab>
			<PaneTab :selected="false">Sockets</PaneTab>
			<PaneTab :selected="false">Events</PaneTab>
		</div>

		<PaneInset class="flex-1 m-4 p-4">
			<div class="overflow-hidden pt-0 mx-2">
				<Selectable text="hit" :selected="true" left-icon="CollectionIcon" />
				<Selectable text="weak" :selected="false" left-icon="CollectionIcon" />
				<Selectable text="strong" :selected="false" left-icon="CollectionIcon" />
			</div>
		</PaneInset>
	</div>
</template>

<script setup lang="ts">
import Selectable from "@/components/basic/Selectable.vue"
import Pane from "@/components/basic/Pane.vue"
import PaneTab from "@/components/basic/PaneTab.vue"
import PaneInset from "@/components/basic/PaneInset.vue"

</script>