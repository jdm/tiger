<template>
	<Pane>
		<PaneTabList>
			<PaneTab @select="focusContentTab(ContentTab.Frames)" :selected="currentTab == ContentTab.Frames">Frames
			</PaneTab>
			<PaneTab @select="focusContentTab(ContentTab.Animations)" :selected="currentTab == ContentTab.Animations">
				Animations
			</PaneTab>
		</PaneTabList>
		<Frames v-if="currentTab == ContentTab.Frames" />
		<Animations v-if="currentTab == ContentTab.Animations" />
	</Pane>
</template>

<script setup lang="ts">
import { useAppStore } from "@/stores/app"
import { computed } from "@vue/reactivity"
import { focusContentTab } from "@/api/document"
import { ContentTab } from "@/api/dto"
import Pane from "@/components/basic/Pane.vue"
import PaneTab from "@/components/basic/PaneTab.vue"
import PaneTabList from "@/components/basic/PaneTabList.vue"
import Animations from "@/components/content/Animations.vue"
import Frames from "@/components/content/Frames.vue"

// TODO Consider saving and restoring scroll positions between content tab or document changes

const app = useAppStore();

const currentTab = computed(() => {
	return app.currentDocument?.contentTab
});

</script>
