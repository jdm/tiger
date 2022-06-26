<template>
	<Pane>
		<PaneTabList>
			<PaneTab :selected="true">Animations</PaneTab>
		</PaneTabList>
		<Animations />
	</Pane>
</template>

<script setup lang="ts">
import Animations from "@/components/animations/Animations.vue"
import Pane from "@/components/basic/Pane.vue"
import PaneTab from "@/components/basic/PaneTab.vue"
import PaneTabList from "@/components/basic/PaneTabList.vue"

</script>
