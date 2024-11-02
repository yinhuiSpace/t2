<script setup>
import { onMounted, ref } from "vue";
import { invoke } from "@tauri-apps/api/core";

import 'echarts';
import 'echarts/theme/blue.js';
import VChart, { THEME_KEY } from 'vue-echarts';

const mem = ref({});
const memChart = ref({});

const option = {
    tooltip: {
        trigger: 'item',
        formatter: function (params) {
            return params.name + '<br/>' + params.marker + params.value + 'GB'
        }
    },
    legend: {
        top: '5%',
        left: 'center'
    },
    series: [
        {
            name: '内存空间',
            type: 'pie',
            radius: ['40%', '70%'],
            center: ['50%', '70%'],
            // adjust the start and end angle
            startAngle: 180,
            endAngle: 360,
            data: []
        }
    ]
};

async function memInfo() {
    mem.value = await invoke("mem_info", {}).then((res) => {
        const data = []
        data.push({
            value: (res.used_memory / (1024 * 1024 * 1024)).toFixed(2),
            name: "已使用"
        })
        data.push({
            value: ((res.total_memory - res.used_memory) / (1024 * 1024 * 1024)).toFixed(2),
            name: "剩余空间"
        })
        memChart.value?.setOption({
            series: [{
                data: data
            }]
        })
    });
    setTimeout(memInfo, 1000);
}

onMounted(() => {
    memChart.value?.setOption(option);
    memInfo();
})
</script>

<template>
    <el-row>
        <el-col>
            <el-card shadow="always">
                <template #header>
                    <div class="card-header">
                        <span>内存</span>
                    </div>
                </template>
                <v-chart ref="memChart" :manual-update="true" autoresize class="chart" />
                <!-- <p class="text item">总内存: {{ Number((mem.total_memory / (1024 * 1024 * 1024)).toFixed(0)) }} GB</p>
                <p class="text item">已使用: {{ Number((mem.used_memory / (1024 * 1024 * 1024)).toFixed(0)) }} GB</p>
                <p class="text item">总虚拟内存: {{ Number((mem.total_swap / (1024 * 1024 * 1024)).toFixed(0)) }} GB</p>
                <p class="text item">已使用虚拟内存: {{ Number((mem.used_swap / (1024 * 1024 * 1024)).toFixed(0)) }} GB</p> -->
            </el-card>
        </el-col>
        <el-col :span="8"></el-col>
    </el-row>
</template>


<style scoped>
.card-header {
    font-size: smaller;
    /* font-weight: bold; */
    display: flex;
    justify-content: space-between;
    align-items: center;
    color: var(--el-text-color-regular);
}

.chart {
    height: 210px;
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
</style>