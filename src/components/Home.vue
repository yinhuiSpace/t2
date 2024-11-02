<script setup>
import SysPanel from "./SysPanel.vue";
import CpuPanel from "./CpuPanel.vue";
import NetPanel from "./NetPanel.vue";
import DiskPanel from "./DiskPanel.vue";
import MemPanel from "./MemPanel.vue";
import ProcessesPanel from "./ProcessesPanel.vue";
import BatteryPanel from "./BatteryPanel.vue";
</script>

<template>
    <el-row>
        <SysPanel />
    </el-row>
    <el-row :gutter="8">
        <el-col :span="12">
            <BatteryPanel />
            <MemPanel />
        </el-col>
        <el-col :span="6">
            <el-card>
                <template #header>
                    <div class="card-header">
                        <span>网络连接</span>
                    </div>
                    <NetPanel />
                </template>
                <!-- <v-chart class="chart" ref="payloadChart" :manual-update="true" autoresize /> -->
            </el-card>
        </el-col>
        <el-col :span="6">
            <CpuPanel />
        </el-col>
    </el-row>
    <el-row>
        <ProcessesPanel />
    </el-row>
</template>


<style scoped>
.chart {
    height: 100px;
    width: 100%;
}

.el-row {
    margin-bottom: 8px;
}

.el-row:last-child {
    margin-bottom: 0;
}

:deep() .el-card {
    --el-card-padding: 8px;
    --el-card-border-radius: 10px;
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