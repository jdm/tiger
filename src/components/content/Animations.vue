<template>
	<div class="flex-1 flex flex-col min-h-0">
		<div class="w-full p-4 flex flex-row items-center space-x-2">
			<input type="text" placeholder="Idle"
				class="w-full h-10 px-4 placeholder-plastic-500 font-bold bg-plastic-800 rounded-md border-y border-plastic-900 border-b-plastic-600" />
			<Button :positive="true" label="New" />
		</div>
		<Separator />
		<PaneInset class="flex-1 m-4 min-h-0">
			<div class="p-4 overflow-y-auto h-full styled-scrollbars">
				<div class="text-plastic-200 flex flex-col">
					<Animation v-for="animation in app.sortedAnimations" :animation="animation" ref="animationRefs"
						:key="animation.name" />
				</div>
			</div>
		</PaneInset>
	</div>
</template>

<script setup lang="ts">
import { useAppStore } from '@/stores/app'
import Button from '@/components/basic/Button.vue'
import PaneInset from '@/components/basic/PaneInset.vue'
import Separator from '@/components/basic/Separator.vue'
import Animation from '@/components/content/Animation.vue'

const app = useAppStore();
</script>