<script setup>
import { onMounted, ref } from "vue";
import { invoke } from "@tauri-apps/api/core";

import 'echarts';
import 'echarts/theme/blue.js';
import VChart, { THEME_KEY } from 'vue-echarts';


const processes = ref([]);
const processChart = ref({});

async function processInfo() {
    processes.value = await invoke("process_info", {}).then((res) => {
        const ns = [];
        const ms = [];
        res.forEach(e => {
            ns.push(e.pid + ":" + e.name);
            ms.push((e.mem/ (1024 * 1024 * 1024)).toFixed(2))
        });
        processChart.value?.setOption({
            yAxis: {
                type: 'category',
                data: ns
            },
            series: [
                {
                    name: '内存占用',
                    data: ms
                }
            ]
        })
    });
    setTimeout(processInfo, 1000);
}
const option = {
    tooltip: {
        trigger: 'axis',
        axisPointer: {
            type: 'shadow'
        },
        formatter: function (params) {
            var relVal = params[0].name
            for (var i = 0, l = params.length; i < l; i++) {
                relVal += '<br/>' + params[i].marker + params[i].value + 'GB'
            }
            return relVal
        }
    },
    legend: {},
    grid: {
        left: '3%',
        right: '4%',
        bottom: '3%',
        containLabel: true
    },
    xAxis: {
        type: 'value',
        boundaryGap: [0, 0.01]
    },
    yAxis: {
        type: 'category',
        data: []
    },
    series: [
        {
            name: '内存占用',
            type: 'bar',
            data: []
        }
    ]
};

onMounted(() => {
    processChart.value?.setOption(option);
    processInfo();
})

</script>

<template>
    <el-card style="width:100%" shadow="always">
        <template #header>
            <div class="card-header">
                <span>进程</span>
            </div>
        </template>
        <el-row>
            <el-col>
                <v-chart ref="processChart" :manual-update="true" autoresize class="chart" />
            </el-col>
        </el-row>
    </el-card>
    <!-- <el-row>
        <el-col>
            <el-table :data="processes" height="500" stripe style="width: 100%"
                :default-sort="{ prop: 'mem', order: 'descending' }">
                <el-table-column prop="pid" sortable label="pid" />
                <el-table-column prop="name" label="进程名" />
                <el-table-column prop="mem" sortable label="占用内存" />
            </el-table>
        </el-col>
    </el-row> -->
</template>

<style scoped>
.chart {
    height: 1200px;
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