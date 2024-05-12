<template>
	<div>
		<div class="ctn">
			<button
				class="tab-btn"
				v-for="(tab, index) in tabs"
				:key="index"
				:title="tab"
				@click="$emit('changeTab', index)"
				:style="
					currentTab === index ? 'border: 1px dimgrey solid;' : 'border: 0;'
				"
			>
				<span v-if="store.lang === 'en'">{{ index == 0 ? "All" : tab }}</span>
				<span v-else>{{ index == 0 ? "所有" : tab }}</span>
			</button>
		</div>
		<br />
		<div class="ctn">
			<!-- 删除本页 -->
			<button @click="$emit('deleteTab')" class="delete-btn">
				{{ store.lang === "en" ? "Delete tab" : "删除本页" }}
			</button>
			<!-- 添加新tab -->
			<div class="add-new-tab-div">
				<input
					type="text"
					class="new-tab-input"
					:placeholder="store.lang === 'en' ? 'new tab' : '新标签页'"
					v-model="newTabName"
				/>
				<button class="add-new-tab-btn" @click="$emit('addNewTab', newTabName)">
					+
				</button>
			</div>
		</div>
	</div>
</template>

<script setup lang="ts">
	import { store } from "./store.js";
	import { computed, watch, ref, reactive } from "vue";
	import { exit } from "@tauri-apps/api/process";
	import { Action, Condition, Rule } from "./types";
	const props = defineProps<{
		currentTab: number;
		tabs: string[];
	}>();
	const newTabName = ref("");
</script>

<style scoped>
	.ctn {
		margin-bottom: 0.5rem;
		display: flex;
		flex-direction: row;
		flex-wrap: wrap;
	}
	.tab-btn {
		min-width: 4rem;
		margin-right: 2px;
		margin-bottom: 5px;
	}
	.add-new-tab-div {
		margin-left: 1rem;
		border-radius: 8px;
		border: 1px solid transparent;
		padding: 0;
		/* font-size: 1em; */
		font-family: inherit;
		color: #ffffff;
		background-color: #00000098;
	}
	.add-new-tab-btn {
		width: fit-content;
	}
	.delete-btn {
		color: #ffffff;
		/* margin-left: 1rem; */
	}
	.new-tab-input {
		min-width: 4rem;
	}
</style>
