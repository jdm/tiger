<template>
	<Select :options="options" :selected="currentPreset" @selected="onSelected" />
</template>

<script setup lang="ts">
import { computed } from "vue"
import { applyDirectionPreset } from "@/backend/api"
import { DirectionPreset } from "@/backend/dto"
import { useStateStore } from "@/stores/state"
import Select, { SelectOption } from "@/components/basic/Select.vue"

const state = useStateStore();
const currentPreset = computed(() => state.currentAnimation?.directionPreset);

const options = computed((): SelectOption[] => {
	return [
		{ name: "4 Directions", value: DirectionPreset.FourDirections },
		{ name: "8 Directions", value: DirectionPreset.EightDirections },
		{ name: "Left / Right", value: DirectionPreset.LeftRight },
		{ name: "Up / Down", value: DirectionPreset.UpDown },
		{ name: "Isometric", value: DirectionPreset.Isometric },
		{ name: "Fixed Angle", value: DirectionPreset.FixedAngle },
	];
});

function onSelected(option: SelectOption) {
	applyDirectionPreset(option.value);
}

</script>