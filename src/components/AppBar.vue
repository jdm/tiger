<template>
	<WindowTitleBar>
		<template #left>
			<MenuBar :entries="menuEntries" class="h-full px-5" />
		</template>
		<template #right>
			<div>
				<div v-if="!app.isReleaseBuild">
					<button @click="onToggleDevTools" class="p-2 px-4 text-white"
						:class="debugMode ? 'bg-green-500' : 'bg-red-500'">🐛</button>
				</div>
			</div>
		</template>
	</WindowTitleBar>
</template>

<script setup lang="ts">
import MenuBar from '@/components/basic/MenuBar.vue';
import WindowTitleBar from '@/components/basic/WindowTitleBar.vue';
import { useAppStore } from '@/stores/app'

const props = defineProps<{ debugMode: boolean, }>();
const emit = defineEmits(["update:debugMode"]);

const app = useAppStore();

function onToggleDevTools() {
	emit("update:debugMode", !props.debugMode);
}

const menuEntries = [
	{
		name: "File", content: [
			{ name: "New Spritesheet…" },
			{ name: "Open Spritesheet…" },
			{},
			{ name: "Save" },
			{ name: "Save As…" },
			{ name: "Save All" },
			{ name: "Export" },
			{ name: "Export As…" },
			{},
			{ name: "Close" },
			{ name: "Close All" },
		]
	},
	{
		name: "Edit", content: [
			{ name: "Undo" },
			{ name: "Redo" },
		]
	},
	{
		name: "View", content: [
			{ name: "Center Workbench" },
			{ name: "Zoom In (Workbench)" },
			{ name: "Zoom Out (Workbench)" },
			{ name: "Reset Zoom (Workbench)" },
			{},
			{ name: "Zoom In (Timeline)" },
			{ name: "Zoom Out (Timeline)" },
			{ name: "Reset Zoom (Timeline)" },
		]
	}
];

</script>