<template>
	<Pane>
		<PaneTabList>
			<PaneTab :selected="true">Frames</PaneTab>
		</PaneTabList>
		<Frames />
	</Pane>
</template>

<script setup lang="ts">
import Pane from "@/components/basic/Pane.vue"
import PaneTab from "@/components/basic/PaneTab.vue"
import PaneTabList from "@/components/basic/PaneTabList.vue"
import Frames from "@/components/frames/Frames.vue"
</script>
