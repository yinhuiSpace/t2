<script setup>
import { onMounted, ref } from "vue";
import { invoke } from "@tauri-apps/api/core";

import 'echarts';
import 'echarts/theme/blue.js';
import VChart, { THEME_KEY } from 'vue-echarts';

const cpuChart = ref({});

const cpu = ref({});

const option = {
    title: {
        text: '各CPU使用率'
    },
    tooltip: {
        trigger: 'axis',
        axisPointer: {
            type: 'shadow'
        },
        formatter: function (params) {
            var relVal = params[0].name
            for (var i = 0, l = params.length; i < l; i++) {
                relVal += '<br/>' + params[i].marker + params[i].value + '%'
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
        boundaryGap: [0, 0.01],
    },
    yAxis: {
        type: 'category',
        data: []
    },
    series: [
        {
            name: '使用率',
            type: 'bar',
            data: []
        }
    ]
};

async function cpuInfo() {
    await invoke("cpu_info", {}).then((res) => {
        const ns = [];
        const us = [];
        res.core_list.forEach(e => {
            ns.push(e.name);
            us.push(e.usage.toFixed(2));
        });
        cpuChart.value?.setOption({
            yAxis: {
                type: 'category',
                data: ns
            },
            series: [
                {
                    name: '使用率',
                    type: 'bar',
                    data: us
                }
            ]
        })
    });
    cpu.value = await invoke("cpu_info", {});
    setTimeout(cpuInfo, 1000);
}
onMounted(() => {
    cpuChart.value?.setOption(option);
    cpuInfo();
})
</script>

<template>
    <el-row>
        <el-col>
            <el-card shadow="always">
                <el-row>
                    <el-col :span="8">
                        <el-statistic title="芯片名称" :value="cpu.chip_name" />
                    </el-col>
                    <el-col :span="8">
                        <el-statistic title="cpu数量" :value="cpu.core_cnt" />
                    </el-col>
                    <!-- <el-col :span="8">
                        <el-statistic title="频率" :value="cpu.frequency" />
                    </el-col> -->
                    <el-col :span="8">
                        <el-statistic title="使用率" :value="cpu.global_usage" />
                    </el-col>
                </el-row>
            </el-card>
        </el-col>
    </el-row>
    <el-row>
        <el-col>
            <el-card shadow="always">
                <el-row>
                    <el-col>
                        <v-chart ref="cpuChart" :manual-update="true" autoresize class="chart" />
                    </el-col>
                </el-row>
            </el-card>
        </el-col>
    </el-row>
    <!-- <el-row :gutter="20">
        <el-col :span="8" v-for="(e, i) in cpu.core_list" :key="i">
            <el-card shadow="always">
                <template #header>
                    <div class="card-header">
                        <span>{{ e.name }}</span>
                    </div>
                </template>
<p class="text item">品牌: {{ e.brand }}</p>
<p class="text item">频率: {{ e.frequency }}</p>
<p class="text item">使用: {{ e.usage }}%</p>
</el-card>
</el-col>
</el-row> -->
</template>


<style scoped>
.chart {
    height: 300px;
    width: 100%;
}

.el-card {
    --el-card-padding: 8px
}

.multi-text {
    -webkit-line-clamp: 1;
}

.card-header {
    font-size: smaller;
    /* font-weight: bold; */
    display: flex;
    justify-content: space-between;
    align-items: center;
    color: var(--el-text-color-regular);
}

.el-row {
    margin-bottom: 8px;
}

.el-row:last-child {
    margin-bottom: 0;
}
</style>