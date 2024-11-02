<script setup>
import { onMounted, ref } from "vue";
import { invoke } from "@tauri-apps/api/core";

const disks = ref([]);

async function diskInfo() {
    disks.value = await invoke("disk_info", {});
    setTimeout(diskInfo, 10000);
}

onMounted(() => {
    diskInfo();
})
</script>

<template>
    <h1>disk</h1>
    <el-row :gutter="20">
        <el-col :span="8" v-for="(e, i) in disks" :key="i">
            <el-card shadow="always">
                <template #header>
                    <div class="card-header">
                        <span>{{ e.mnt }}</span>
                    </div>
                </template>
                <p class="text item">名称: {{ e.name }}</p>
                <p class="text item">格式: {{ e.fs }}</p>
                <p class="text item">种类: {{ e.ty }}</p>
                <p class="text item">是否可拔出: {{ e.is_rmv ? "是" : "否" }}</p>
                <p class="text item">总空间: {{ Number((e.total / (1024.0 * 1024.0 * 1024.0)).toFixed(0)) }} GB</p>
                <p class="text item">已使用: {{ Number((e.used / (1024.0 * 1024.0 * 1024.0)).toFixed(0)) }} GB</p>
                <p class="text item">使用率: {{ Number(((e.used / e.total) * 100).toFixed(0)) }}%</p>
            </el-card>
        </el-col>
    </el-row>
</template>


<style scoped>

</style>