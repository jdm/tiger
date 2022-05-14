<template>
	<Pane>
		<template #header>
			<PaneTab @select="focusContentTab('frames')" :selected="currentTab == 'frames'">Frames</PaneTab>
			<PaneTab @select="focusContentTab('animations')" :selected="currentTab == 'animations'">Animations</PaneTab>
		</template>
		<template #content>
			<ul v-if="currentTab == 'frames'">
				<li class="overflow-x-hidden text-ellipsis" v-for="frame in app.currentDocument?.sheet.frames">{{
						frame.name
				}}
				</li>
			</ul>
		</template>
	</Pane>
</template>

<script setup lang="ts">
import { useAppStore } from '@/stores/app'
import { computed } from '@vue/reactivity';
import Pane from '@/components/pane/Pane.vue'
import PaneTab from '@/components/pane/PaneTab.vue'
import { focusContentTab } from '@/api/document'

const app = useAppStore()

const currentTab = computed(() => {
	return app.currentDocument?.view?.contentTab
})

</script>