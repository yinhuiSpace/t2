<script setup>
import { onMounted, ref } from "vue";
import { invoke } from "@tauri-apps/api/core";

const sys = ref({});
async function sysInfo() {
    sys.value = await invoke("sys_info", {});
}
onMounted(async () => {
    sys.value = await invoke("sys_info", {});
})
</script>

<template>
    <el-row>
        <el-col>
            <el-card style="width:100%" shadow="always">
                <el-row>
                    <el-col :span="6">
                        <el-statistic title="当前平台" :value="sys.system_name" />
                    </el-col>

                    <el-col :span="6">
                        <el-statistic :value="sys.host_name">
                            <template #title>
                                <div style="display: inline-flex; align-items: center">
                                    主机名
                                </div>
                            </template>
                        </el-statistic>
                    </el-col>
                    <el-col :span="6">
                        <el-statistic title="内核版本" :value="sys.kernel_version" />
                    </el-col>
                    <el-col :span="6">
                        <el-statistic title="操作系统版本" :value="sys.os_version">

                        </el-statistic>
                    </el-col>
                </el-row>
            </el-card>
        </el-col>
    </el-row>
</template>


<style scoped>
.chart {
    height: 80px;
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