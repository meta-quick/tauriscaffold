<template>
  <el-container class="contain_wrapper">
    <el-header>
      <navbar></navbar>
    </el-header>
    <el-container class="sub_contain_wrapper">
      <el-main>
        <app-main/>
      </el-main>
    </el-container>
  </el-container>
</template>
  
<script lang="ts" setup>
import AppMain from './components/AppMain.vue';
import Navbar from './components/Navbar.vue';
import { ElHeader } from 'element-plus';
</script>
  
<style lang="scss" scoped>
.contain_wrapper {
  width: 100%;
  height: 100%;
}
.sub_contain_wrapper {
  margin-top: 10px;
}
</style>
  