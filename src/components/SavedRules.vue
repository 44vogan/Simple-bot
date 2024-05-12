<template>
	<div class="rules-div">
		<div v-for="(rule, index) in savedRules" :key="index">
			<div
				class="a-rule"
				v-if="
					(rule.enabled || !hideDisabled) &&
					(currentTabName === '' || rule.tab === currentTabName)
				"
			>
				<div class="rule-note-delete">
					<p
						@click="$emit('fold_this_rule', index)"
						:title="rule.note"
						class="rule-note"
					>
						<div class="actived-circle" v-if="rule.enabled"></div>
						{{ rule.note }}
					</p>
					<div>
						<button @click="$emit('copy_to_new_rule', index)" title="复制规则">
							{{ store.lang === "en" ? "Copy" : "复制" }}
						</button>
						<button
							v-if="store.lang === 'en'"
							@click="$emit('toggleEnable', index)"
						>
							{{ rule.enabled ? "Enabled" : "Disabled" }}
						</button>
						<button v-else @click="$emit('toggleEnable', index)">
							{{ rule.enabled ? "启用中" : "已禁用" }}
						</button>
						<!-- <button @click="edit_this_rule(index)" title="编辑规则">
							Edit
						</button> -->
						<button @click="$emit('fold_this_rule', index)" title="折叠/显示">
							{{ rule.fold === true ? "O" : "-" }}
						</button>
						<button @click="$emit('delete_this_rule', index)" title="删除">
							x
						</button>
					</div>
				</div>
				<!-- 条件 -->
				<div v-if="!rule.fold" class="rule-conditions">
					<!-- 所有的condition -->
					<div
						class="rule-condition"
						v-for="(condition, conditionIdx) in rule.conditions"
					>
						<span title="条件">
							<button
								@click="$emit('addConditionToNewRule', condition)"
								:title="
									store.lang === 'en' ? 'add to new rule' : '添加到新规则中'
								"
							>
								+
							</button>
							{{ store.lang === "en" ? "condition" : "条件" }}
							{{ conditionIdx + 1 }}
						</span>
						<div title="坐标">
							<span> x: {{ condition.matchPoint[0] }}, </span>
							<span>y: {{ condition.matchPoint[1] }}</span>
						</div>
						<div>
							<span v-if="store.lang === 'en'">
								{{ condition.matchResult }}
							</span>
							<span v-else>
								{{ condition.matchResult === "Match" ? "颜色是" : "颜色不是" }}
							</span>
						</div>
						<div title="rgb颜色">
							<span>r: {{ condition.matchColor[0] }},</span>
							<span>g: {{ condition.matchColor[1] }},</span>
							<span>b: {{ condition.matchColor[2] }}</span>
						</div>
					</div>
				</div>
				<!-- 动作 actions -->
				<div
					v-if="!rule.fold"
					v-for="(action, actionIdx) in rule.actions"
					class="rule-action"
				>
					<div class="">
						<!-- 动作编号 -->
						<span>
							{{ store.lang === "en" ? "action" : "动作" }}
							{{ actionIdx + 1 }}
						</span>
						<!-- 延迟部分 -->
						<span title="距离条件达成时延后" v-if="action.delay !== 0">
							{{ store.lang === "en" ? "delay" : "延迟" }}
							{{ action.delay }} ms
						</span>
						<!-- 动作类型 -->
						<span class="action-type">
							{{ action.actionType }}
						</span>
						<!-- 动作类型 移动鼠标 -->
						<span
							class="click-point action-type"
							v-if="
								action.actionType === 'Move Mouse' ||
								action.actionType === '移动鼠标'
							"
							:title="store.lang === 'en' ? 'move mouse to' : '鼠标移动到'"
						>
							{{ store.lang === "en" ? "move mouse to" : "鼠标移动到" }}
							{{ action.coor[0] }},{{ action.coor[1] }}
						</span>
						<!-- 动作类型 按键 -->
						<span
							class="click-point action-type"
							v-if="
								action.actionType === 'Press key' ||
								action.actionType === '按键'
							"
						>
							<span v-if="action.keyType === '0-9,a-z'">{{ action.key }}</span>
							<span v-else> {{ action.specialKey }}</span>
							<span> - </span>
							<!-- <span>{{ store.lang === "en" ? "duration:" : "时长:" }}</span> -->
							<span v-if="!action.pressAndHold">
								{{ action.pressDuration }}ms
							</span>
							<span v-else>
								{{ store.lang === "en" ? "press and hold" : "一直按住" }}
							</span>
						</span>
						<!-- 动作类型 发送文字 -->
						<span
							class="click-point action-type"
							v-if="
								action.actionType === 'Send Text' ||
								action.actionType === '发送文字'
							"
						>
							{{ action.text }}
						</span>
					</div>
				</div>
				<!-- sleep or not -->
				<span
					v-if="rule.sleepAfterFinish && !rule.fold"
					class="saved-rule-sleep"
				>
					{{ store.lang === "en" ? "sleep" : "休眠" }} {{ rule.sleepTime }}
					{{ store.lang === "en" ? "ms" : "毫秒" }}
				</span>
			</div>
		</div>
	</div>
</template>

<script setup lang="ts">
	import { store } from "./store.js";
	import { Action, Condition, Rule } from "./types";
	const props = defineProps<{
		savedRules: Rule[];
		hideDisabled: boolean;
		currentTabName: string;
	}>();
</script>

<style scoped>
	.rules-div {
		margin-bottom: 5px;
	}
	.a-rule {
		display: flex;
		flex-direction: column;
		justify-content: space-between;
		margin-bottom: 10px;
		border-radius: 8px;
		padding: 3px;
		border: 1px solid transparent;
		font-family: inherit;
		color: #ffffff;
		background-color: #0f0f0f98;
		transition: border-color 0.25s;
		box-shadow: 0 2px 2px rgba(0, 0, 0, 0.2);
	}
	.saved-rule-sleep {
		padding: 5px;
	}
	.rule-condition {
		display: flex;
		align-items: center;
		border-radius: 5px;
		margin: 5px 0;
		justify-content: space-between;
		width: 100%;
		padding: 2px;
	}

	.rule-conditions {
		padding: 3px 50px;
		background: #242424;
		border-radius: 5px;
		align-items: center;
		display: flex;
		flex-direction: column;
		justify-content: flex-start;
		align-items: baseline;
		font-size: 16px;
	}
	.rule-action {
		padding: 3px 50px;
		background: #242424;
		border-radius: 5px;
		align-items: center;
		margin: 5px 0;
		margin-bottom: 0;
		display: flex;
		flex-direction: column;
		justify-content: space-between;
		align-items: baseline;
	}
	.rule-action div {
		padding: 2px 0;
		display: flex;
		width: 100%;
		justify-content: space-between;
		align-items: center;
	}
	.action-type {
		font-weight: bold;
	}
	.rule-note-delete {
		margin: 0;
		padding: 0 20px;
		display: flex;
		flex-direction: row;
		justify-content: space-between;
		align-items: center;
	}
	.rule-note {
		padding: 0;
		min-width: 450px;
        display: flex;
        align-items: center;
	}
	.actived-circle {
		background: #00ff55;
		height: 15px;
		width: 15px;
        margin-right: 5px;
        border-radius: 15px;
	}
</style>
