<template>
	<div ref="el">
		<Selectable @click="(event) => onHitboxClicked(event)" :selected="hitbox.selected" :text="props.name"
			left-icon="TagIcon" :actions="renaming ? [] :
			[
				{ icon: 'PencilAltIcon', callback: onRenameClicked },
				{ icon: 'XIcon', callback: onDeleteClicked }
			]">
			<template #content v-if="renaming">
				<InputRename v-model="newName" @complete-rename="onRenameInputComplete"
					@cancel-rename="onRenameInputCancelled" />
			</template>
		</Selectable>
	</div>
</template>

<script setup lang="ts">
import { Hitbox as HitboxDTO } from "@/api/dto"
import { selectHitbox } from "@/api/document"
import { Ref, ref } from "@vue/reactivity"
import Selectable from "@/components/basic/Selectable.vue"
import InputRename from "@/components/basic/InputRename.vue"

const props = defineProps<{
	name: string,
	hitbox: HitboxDTO
}>();

const name = props.name;
defineExpose({
	name,
	scrollIntoView() {
		if (el.value) {
			el.value.scrollIntoView({ behavior: "smooth", block: "center" });
		}
	}
});

const renaming = ref(false);
const newName = ref("");
const el: Ref<HTMLElement | null> = ref(null);

function onHitboxClicked(event: MouseEvent) {
	selectHitbox(props.name, event.shiftKey, event.ctrlKey)
}

function onRenameClicked() {
	newName.value = props.name;
	renaming.value = true;
}

function onRenameInputComplete() {
	// renameHitbox(props.name, newName.value);
	renaming.value = false;
}

function onRenameInputCancelled() {
	renaming.value = false;
}

function onDeleteClicked() {
	// deleteHitbox(props.name);
}
</script>
