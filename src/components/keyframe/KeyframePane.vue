<template>
	<Pane>
		<div class="flex flex-row bg-plastic-900">
			<PaneTab :selected="true">Hitboxes</PaneTab>
			<PaneTab :selected="false">Sockets</PaneTab>
			<PaneTab :selected="false">Events</PaneTab>
		</div>
		<div class="flex-1 flex flex-col min-h-0 p-4 space-y-4">
			<div class="w-full flex flex-row space-x-2 items-center">
				<Toggle :toggled="true" icon="EyeIcon" />
				<Toggle :toggled="true" icon="LockClosedIcon" />
				<div class="flex-1 flex flex-row justify-end">
					<Button :positive="true" icon="TagIcon" label="New" />
				</div>
			</div>
			<PaneInset class="flex-1 min-h-0">
				<div class="p-4 overflow-y-auto h-full styled-scrollbars">
					<div class="flex flex-col">
						<Hitbox v-for="hitbox, name in app.currentKeyframe?.hitboxes" :hitbox="hitbox" :name="name"
							:key="hitbox.key" />
					</div>
				</div>
			</PaneInset>
		</div>
	</Pane>
</template>

<script setup lang="ts">
import { useAppStore } from "@/stores/app"
import Button from "@/components/basic/Button.vue"
import Pane from "@/components/basic/Pane.vue"
import PaneTab from "@/components/basic/PaneTab.vue"
import PaneInset from "@/components/basic/PaneInset.vue"
import Hitbox from "@/components/keyframe/Hitbox.vue";
import Toggle from "@/components/basic/Toggle.vue"

const app = useAppStore();

</script>