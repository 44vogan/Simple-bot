<template>
	<div class="add-new-rule">
		<button v-if="!showNewRule.val" @click="$emit('add_rule')" title="添加规则">
			{{ store.lang === "en" ? "+  new rule" : "+  新规则" }}
		</button>
		<div v-if="showNewRule.val" class="new-rule">
			<div class="new-rule-match">
				<!-- 新规则条件 -->
				<div
					v-for="(condition, conditionIdx) in newRule.conditions"
					class="new-rule-condition"
				>
					<span title="条件"
						>{{ store.lang === "en" ? "condition" : "条件" }}
						{{ conditionIdx + 1 }}:
					</span>
					<div>
						x:<input
							type="number"
							min="0"
							v-model="condition.matchPoint[0]"
							title="匹配坐标的x值"
						/>
						y:<input
							type="number"
							min="0"
							v-model="condition.matchPoint[1]"
							title="匹配坐标的y值"
						/>
					</div>
					<div>
						<select v-model="condition.matchResult">
							<option title="匹配时" value="Match">
								{{ store.lang === "en" ? "Match color" : "颜色是" }}
							</option>
							<option title="不匹配" value="Not match">
								{{ store.lang === "en" ? "Not Match color" : "颜色不是" }}
							</option>
						</select>
					</div>
					<div>
						r:<input
							type="number"
							min="0"
							max="255"
							v-model="condition.matchColor[0]"
							title="用来匹配坐标rgb颜色的r值(0-255)"
						/>
						g:<input
							type="number"
							min="0"
							max="255"
							v-model="condition.matchColor[1]"
							title="用来匹配坐标rgb颜色的g值(0-255)"
						/>
						b:<input
							type="number"
							min="0"
							max="255"
							v-model="condition.matchColor[2]"
							title="用来匹配坐标rgb颜色的b值(0-255)"
						/>
					</div>
					<button
						v-if="newRule.conditions.length > 1"
						@click="$emit('delete_condition', conditionIdx)"
						title="删除条件"
					>
						x
					</button>
				</div>
				<button @click="$emit('add_condition')" title="添加条件">+</button>
			</div>
			<!-- 新规则动作 -->
			<div class="new-rule-actions">
				<div
					v-for="(action, actionIdx) in newRule.actions"
					class="new-rule-action"
				>
					<div>
						<span title="动作"
							>{{ store.lang === "en" ? "Do" : "动作" }} {{ actionIdx + 1 }} :
						</span>
						<select v-model="action.actionType">
							<option value="Move Mouse" title="移动鼠标">
								{{ store.lang === "en" ? "Move Mouse" : "移动鼠标" }}
							</option>
							<option value="Press key" title="按键">
								{{ store.lang === "en" ? "Press key" : "按键" }}
							</option>
							<option value="Send Text" title="发送文字">
								{{ store.lang === "en" ? "Send Text" : "发送文字" }}
							</option>
						</select>
					</div>
					<span
						v-if="
							action.actionType === 'Send Text' ||
							action.actionType === '发送文字'
						"
					>
						<input
							class="input-text"
							type="text"
							v-model="action.text"
							placeholder="text to input"
							title="输入文字"
						/>
					</span>
					<span
						class="click-point"
						v-if="
							action.actionType === 'Move Mouse' ||
							action.actionType === '移动鼠标'
						"
					>
						x:<input
							type="number"
							min="0"
							v-model="action.coor[0]"
							title="鼠标移到x坐标"
						/>
						y:<input
							type="number"
							min="0"
							v-model="action.coor[1]"
							title="鼠标移到y坐标"
						/>
					</span>
					<span
						class="click-point"
						v-if="
							action.actionType === '按键' || action.actionType === 'Press key'
						"
					>
						<select v-model="action.keyType">
							<option value="0-9,a-z" title="0-9,a-z">0-9,a-z</option>
							<option value="special key" title="special">
								{{ store.lang === "en" ? "special key" : "特殊按键" }}
							</option>
						</select>
						<span v-if="action.keyType === '0-9,a-z'">
							<input
								type="text"
								size="1"
								maxlength="1"
								v-model="action.key"
								title="输入单个按键(0-9,a-z,留空为q)"
							/>
						</span>
						<span v-else>
							<select v-model="action.specialKey">
								<option v-for="key of specialKeys">{{ key }}</option>
							</select>
						</span>
						<select v-model="action.pressAndHold">
							<option :value="false">
								{{ store.lang === "en" ? "duration" : "时长" }}
							</option>
							<option :value="true">
								{{ store.lang === "en" ? "hold" : "按住" }}
							</option>
						</select>
						<input
							v-if="!action.pressAndHold"
							type="number"
							min="0"
							v-model="action.pressDuration"
							title="按下时长"
						/>
						<span v-if="!action.pressAndHold">{{
							store.lang === "en" ? "ms" : "毫秒"
						}}</span>
					</span>

					<div>
						<span>
							{{ store.lang === "en" ? "delay(ms):" : "延迟(毫秒):" }}
							<input
								type="number"
								min="0"
								v-model="action.delay"
								title="动作延迟时间"
							/>
						</span>

						<button
							v-if="newRule.actions.length > 1"
							@click="$emit('delete_action', actionIdx)"
							title="删除动作"
							class="delete-action-btn"
						>
							x
						</button>
					</div>
				</div>
				<button @click="$emit('add_action')" title="添加动作">+</button>
			</div>
			<!-- sleepAfterFinish -->
			<div class="sleep-after-finish">
				{{
					store.lang === "en"
						? "Rule sleep after actived"
						: "执行一次后规则休眠"
				}}
				<select v-model="newRule.sleepAfterFinish">
					<option title="规则休眠" :value="true">
						{{ store.lang === "en" ? "Yes" : "是" }}
					</option>
					<option title="规则不休眠" :value="false">
						{{ store.lang === "en" ? "no" : "否" }}
					</option>
				</select>
				<span class="sleep-time" v-if="newRule.sleepAfterFinish">
					<input
						type="number"
						min="0"
						v-model="newRule.sleepTime"
						:title="store.lang === 'en' ? 'sleep duration' : '规则睡眠时长'"
					/>(ms)
				</span>
			</div>
			<div class="new-rule-comfirm">
				<button @click="$emit('comfirm_new_rule')" title="确认添加规则">
					{{ store.lang === "en" ? "Confirm" : "确认" }}
				</button>
				<span title="备注">
					{{ store.lang === "en" ? "Note:" : "备注:" }}
					<input
						id="input-note"
						type="text"
						maxlength="50"
						v-model="newRule.note"
					/>
				</span>
				<button
					@click="showNewRule.val = !showNewRule.val"
					title="取消添加新规则"
				>
					{{ store.lang === "en" ? "Cancel" : "取消" }}
				</button>
			</div>
		</div>
	</div>
</template>

<script setup lang="ts">
	import { store } from "./store.js";
	import { exit } from "@tauri-apps/api/process";
	import { Action, Condition, Rule } from "./types";
	const props = defineProps<{
		newRule: Rule;
		showNewRule: {
			val: boolean;
		};
	}>();

	const specialKeys =
		"MouseButtonLeft,MouseButtonRight,Windows,Super,Command,Esc,Tab,CapsLock,Shift,Control,Alt,Space,Delete,Backspace,Return,Enter,Home,End,PageDown,PageUp,UpArrow,DownArrow,LeftArrow,RightArrow,F1,F2,F3,F4,F5,F6,F7,F8,F9,F10,F11,F12".split(
			","
		);
</script>

<style scoped>
	/* .a-rule div { */
	/* padding: 2px; */
	/* } */
	.new-rule {
		display: flex;
		flex-direction: column;
		justify-content: space-between;
		margin-bottom: 10px;
		border-radius: 8px;
		padding: 5px;
		border: 1px solid transparent;
		font-family: inherit;
		color: #ffffff;
		background-color: #20202098;
		transition: border-color 0.25s;
		box-shadow: 0 2px 2px rgba(0, 0, 0, 0.2);
	}
	.new-rule-condition {
		display: flex;
		align-items: center;
		justify-content: space-between;
	}
	.new-rule div {
		margin: 2px 5px;
	}
	.new-rule-action {
		display: flex;
		justify-content: space-between;
		align-items: center;
	}
	.input-text {
		width: max-content;
	}
	.new-rule-comfirm {
		display: flex;
		flex-direction: row;
		justify-content: space-between;
	}
	#input-note {
		width: 150px;
	}
	.delete-action-btn {
		margin-left: 5px;
	}
</style>
