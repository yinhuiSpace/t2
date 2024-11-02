<script setup>
import { onMounted, ref } from "vue";
import { invoke } from "@tauri-apps/api/core";

const batteries = ref([]);

async function batteryInfo() {
    batteries.value = await invoke("battery_info", {});
    setTimeout(batteryInfo, 1000);
}

onMounted(() => {
    batteryInfo();
})
</script>

<template>
    <el-row>
        <el-col v-for="(e, i) in batteries" :key="i">
            <el-card shadow="always">
                <template #header>
                    <div class="card-header">
                        <span>电池</span>
                    </div>
                </template>
                <p class="text item">温度: {{ e.temperature }}</p>
                <p class="text item">循环次数: {{ e.cycle_count }}</p>
                <p class="text item">状态: {{ e.state }}</p>
                <p class="text item">电量: {{ e.percentage }} %</p>
                <p class="text item">健康程度: {{ e.state_of_health }}</p>
            </el-card>
        </el-col>
    </el-row>
</template>

<style scoped>
.chart {
    height: 80px;
    width: 100%;
}

.el-row {
    margin-bottom: 8px;
}

.el-row:last-child {
    margin-bottom: 0;
}

.el-card {
    --el-card-padding: 8px
}

.card-header {
    font-size: smaller;
    /* font-weight: bold; */
    display: flex;
    justify-content: space-between;
    align-items: center;
    color: var(--el-text-color-regular);
}
</style>