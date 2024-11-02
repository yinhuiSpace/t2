<script setup>
import { onMounted, ref } from "vue";
import { invoke } from "@tauri-apps/api/core";
import 'echarts';
import 'echarts/theme/blue.js';
import VChart, { THEME_KEY } from 'vue-echarts';




const nds = ref([]);
const netChart = ref({});
const netChart2 = ref({});


const option_column = {
    title:{
        text:'上传'
    },
    tooltip: {
        trigger: 'item',
        formatter: function (params) {
            return params.name + '<br/>' + params.marker + params.value + 'KB'
        }
    },
    legend: {
        top: '5%',
        left: 'center'
    },
    series: [
        {
            name: '上传速度',
            type: 'pie',
            radius: ['40%', '70%'],
            avoidLabelOverlap: false,
            itemStyle: {
                borderRadius: 10,
                borderColor: '#fff',
                borderWidth: 2
            },
            label: {
                show: false,
                position: 'center'
            },
            emphasis: {
                label: {
                    show: true,
                    fontSize: 40,
                    fontWeight: 'bold'
                }
            },
            labelLine: {
                show: false
            },
            data: []
        }
    ]
}

const option = {
    title:{
        text:'下载'
    },
    tooltip: {
        trigger: 'item',
        formatter: function (params) {
            return params.name + '<br/>' + params.marker + params.value + 'KB'
        }
    },
    legend: {
        top: '5%',
        left: 'center'
    },
    series: [
        {
            name: '下载速度',
            type: 'pie',
            radius: ['40%', '70%'],
            avoidLabelOverlap: false,
            itemStyle: {
                borderRadius: 10,
                borderColor: '#fff',
                borderWidth: 2
            },
            label: {
                show: false,
                position: 'center'
            },
            emphasis: {
                label: {
                    show: true,
                    fontSize: 40,
                    fontWeight: 'bold'
                }
            },
            labelLine: {
                show: false
            },
            data: []
        }
    ]
};


async function netsInfo() {
    nds.value = await invoke("net_data", {}).then((res) => {
        const data = [];
        const data2 = [];
        res.forEach(e => {
            data.push({
                value: (e.tran / (1024 * 1024)).toFixed(2),
                name: e.name
            })
            data2.push({
                value: (e.rec / (1024 * 1024)).toFixed(2),
                name: e.name
            })
        });
        netChart.value?.setOption({
            series: [
                {
                    data: data
                }
            ]
        })

        netChart2.value?.setOption({
            series: [
                {
                    data: data2
                }
            ]
        })


    })
    setTimeout(netsInfo, 2000);
}
onMounted(() => {
    netChart.value?.setOption(option_column);
    netChart2.value?.setOption(option);
    netsInfo();
})
</script>

<template>
    <el-row>
        <el-col>
            <v-chart ref="netChart" :manual-update="true" autoresize class="chart" />
        </el-col>
    </el-row>
    <el-row>
        <el-col>
            <v-chart ref="netChart2" :manual-update="true" autoresize class="chart" />
        </el-col>
    </el-row>
    <!-- <el-row v-for="(e, i) in nds" :key="i">
        <el-col>
            <el-card shadow="always">
                <template #header>
                    <div class="card-header">
                        <span>{{ e.name }}</span>
                    </div>
                </template>
<p class="text item">下载: {{ e.rec }} 字节</p>
<p class="text item">上传: {{ e.tran }} 字节</p>
</el-card>
</el-col>
</el-row> -->
</template>


<style scoped>
.chart {
    height: 230px;
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