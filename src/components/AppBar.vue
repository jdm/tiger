<template>
	<WindowTitleBar>
		<template #left>
			<div class="px-5 flex flex-row space-x-6 text-plastic-300">
				<button @click="newDocument" class="place-self-start">New Document</button>
				<button @click="openDocuments" class="place-self-start">Open Document</button>
			</div>
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
import WindowTitleBar from '@/components/basic/WindowTitleBar.vue';
import { newDocument, openDocuments } from '@/api/local'
import { useAppStore } from '@/stores/app'

const props = defineProps<{ debugMode: boolean, }>();
const emit = defineEmits(["update:debugMode"]);

const app = useAppStore();

function onToggleDevTools() {
	emit("update:debugMode", !props.debugMode);
}

</script>