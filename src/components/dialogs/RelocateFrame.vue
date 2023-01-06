<template>
	<div class="flex flex-col gap-2 border-l-0 border-plastic-700">
		<div class="flex items-center gap-4">
			<Thumbnail class="w-20 h-20" :path="newLocation" v-model:is-valid="hasReplacement" />
			<div class="grow flex flex-col gap-3">
				<div class="flex gap-2 items-center">
					<CheckCircleIcon class="w-6 text-green-500" v-if="hasReplacement" />
					<div class="text-lg font-semibold leading-none" :class="hasReplacement ? 'text-green-500' : ''">{{
					frame.name }}
					</div>
				</div>
				<InputPath v-model="newLocation" pick-existing :placeholder="frame.path" />
			</div>
		</div>
	</div>
</template>

<script setup lang="ts">
import { computed, Ref, ref } from "vue";
import { CheckCircleIcon } from "@heroicons/vue/24/solid"
import { relocateFrame } from "@/api/document";
import { Frame } from "@/api/dto";
import { useAppStore } from "@/stores/app";
import InputPath from "@/components/basic/InputPath.vue"
import Thumbnail from "@/components/frames/Thumbnail.vue"

const app = useAppStore();

const props = defineProps<{
	frame: Frame
}>();

const hasReplacement = ref(false);

const newLocation = computed({
	get: () => app.currentDocument?.framesBeingRelocated?.[props.frame.path] || "",
	set: (f) => relocateFrame(props.frame.path, f),
});

</script>