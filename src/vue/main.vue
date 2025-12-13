<template>

    <body
        class="bg-app-bg text-app-text font-sans antialiased overflow-hidden transition-colors duration-300 h-screen w-screen flex flex-col">

        <div id="app" v-cloak class="h-full flex flex-col">

            <!-- 1. Title Bar (for Tauri window dragging) -->
            <header
                class="h-10 bg-app-sidebar flex items-center justify-between px-4 border-b border-app-border titlebar-drag-region select-none transition-colors duration-300">
                <div class="flex items-center gap-2 text-sm font-medium text-app-mute">
                    <i class="ph-fill ph-package text-app-primary text-lg"></i>
                    <span>Archiver Pro</span>
                </div>
                <div class="flex items-center gap-3 no-drag">
                    <button class="w-3 h-3 rounded-full bg-yellow-500 hover:bg-yellow-400 transition"></button>
                    <button class="w-3 h-3 rounded-full bg-green-500 hover:bg-green-400 transition"></button>
                    <button @click="close_app" class="w-3 h-3 rounded-full bg-red-500 hover:bg-red-400 transition">
                    </button>
                </div>
            </header>

            <!-- Main Layout. -->
            <div class="flex-1 flex overflow-hidden">

                <!-- 2. Sidebar. -->
                <aside
                    class="w-64 bg-app-sidebar flex flex-col border-r border-app-border transition-colors duration-300">
                    <div class="p-4 space-y-1">
                        <nav-item icon="ph-house" label="首页" :active="current_view === 'home'"
                            @click="current_view = 'home'"></nav-item>
                        <nav-item icon="ph-clock-counter-clockwise" label="最近文件" :active="current_view === 'recent'"
                            @click="current_view = 'recent'"></nav-item>
                    </div>

                    <!-- Bottom: Settings. -->
                    <div class="mt-auto p-4 border-t border-app-border">
                        <div @click="current_view = 'settings'"
                            class="flex items-center gap-3 px-3 py-3 rounded-lg cursor-pointer transition-all duration-200"
                            :class="current_view === 'settings' ? 'bg-app-primary text-white shadow-md' : 'text-app-mute hover:bg-app-surface-hover hover:text-app-text'">
                            <i class="ph-gear text-xl"></i>
                            <span class="text-sm font-medium">设置</span>
                        </div>
                    </div>
                </aside>

                <!-- 3. Content Area. -->
                <main class="flex-1 flex flex-col bg-app-bg relative transition-colors duration-300">
                    <!-- View Content Switching. -->
                    <!-- A. Settings View. -->
                    <div v-if="current_view === 'settings'" class="p-8 max-w-2xl mx-auto w-full animate-fade-in">
                        <h2 class="text-2xl font-bold text-app-text mb-6">设置</h2>

                        <div class="space-y-6">
                            <!-- Theme Settings. -->
                            <div
                                class="bg-app-sidebar border border-app-border rounded-xl p-6 transition-colors duration-300">
                                <div class="flex items-center justify-between">
                                    <div class="flex items-center gap-4">
                                        <div>
                                            <h3 class="font-medium text-app-text">界面主题</h3>
                                            <p class="text-xs text-app-mute">在深色模式和浅色模式之间切换</p>
                                        </div>
                                    </div>

                                    <!-- Toggle Switch. -->
                                    <button @click="toggle_theme"
                                        class="relative inline-flex h-8 w-14 items-center rounded-full transition-colors focus:outline-none"
                                        :class="is_dark_mode ? 'bg-app-primary' : 'bg-gray-300'">
                                        <span class="sr-only">切换主题</span>
                                        <span
                                            class="inline-block h-6 w-6 transform rounded-full bg-white transition-transform duration-200 ease-in-out shadow-sm flex items-center justify-center"
                                            :class="is_dark_mode ? 'translate-x-7' : 'translate-x-1'">
                                            <i v-if="is_dark_mode" class="ph-moon-stars text-xs text-indigo-500"></i>
                                            <i v-else class="ph-sun text-xs text-orange-500"></i>
                                        </span>
                                    </button>
                                </div>
                            </div>
                        </div>
                    </div>

                    <!-- B. Home / File View. -->
                    <div v-else class="flex-1 overflow-auto relative" @drag_over.prevent="drag_over = true"
                        @dragleave.prevent="drag_over = false" @drop.prevent="handle_drop">

                        <!-- Empty State / Drag-and-Drop Hint. -->
                        <div v-if="files.length === 0"
                            class="absolute inset-0 flex flex-col items-center justify-center transition-all duration-300 m-6 rounded-2xl border-2 border-dashed"
                            :class="drag_over ? 'border-app-primary bg-app-primary/5' : 'border-transparent'">

                            <h2 class="text-2xl font-semibold text-app-text mb-2">拖放文件至此</h2>
                            <p class="text-app-mute text-sm mb-8 text-center max-w-xs">
                                或点击下方按钮选择文件<br>支持 .zip, .rar, .7z 等格式
                            </p>

                            <button @click="openFile"
                                class="px-8 py-3 bg-app-primary hover:opacity-90 text-white text-sm font-medium rounded-full shadow-lg shadow-app-primary/30 transition transform active:scale-95 flex items-center gap-2">
                                <i class="ph-folder-open"></i>
                                <span>选择文件</span>
                            </button>
                        </div>

                        <!-- File List. -->
                        <div v-else class="p-2">
                            <!-- The space at the top is now occupied by the title bar, so the list follows directly after it. -->
                            <table class="w-full text-left border-collapse">
                                <thead
                                    class="sticky top-0 bg-app-bg text-app-mute text-xs font-medium uppercase tracking-wider z-10 transition-colors duration-300">
                                    <tr>
                                        <th class="px-4 py-3 cursor-pointer hover:text-app-text">名称</th>
                                        <th class="px-4 py-3 w-32 cursor-pointer hover:text-app-text text-right">大小</th>
                                        <th class="px-4 py-3 w-32 cursor-pointer hover:text-app-text">类型</th>
                                    </tr>
                                </thead>
                                <tbody class="text-sm divide-y divide-app-border">
                                    <tr v-for="(file, index) in files" :key="index"
                                        class="group hover:bg-app-surface transition cursor-default select-none"
                                        :class="{ 'bg-app-primary/10': selectedFiles.includes(index) }"
                                        @click="toggleSelect(index, $event)">
                                        <td class="px-4 py-3 flex items-center gap-3">
                                            <i :class="getFileIcon(file.type)"
                                                class="text-xl opacity-80 group-hover:opacity-100"></i>
                                            <span class="text-app-text font-medium truncate max-w-[200px]">{{ file.name
                                                }}</span>
                                        </td>
                                        <td class="px-4 py-3 text-app-mute text-right font-mono text-xs">{{ file.size }}
                                        </td>
                                        <td class="px-4 py-3 text-app-mute">{{ file.type }}</td>
                                    </tr>
                                </tbody>
                            </table>
                        </div>
                    </div>

                    <!-- Bottom Action Bar (Visible only when files exist and not on the settings page) -->
                    <div v-if="files.length > 0 && current_view !== 'settings'"
                        class="h-16 border-t border-app-border bg-app-sidebar px-6 flex items-center justify-between transition-colors duration-300">
                        <div class="text-xs text-app-mute">
                            已选中 {{ selectedFiles.length }} 项
                        </div>
                        <div class="flex items-center gap-3">
                            <button @click="files = []"
                                class="px-4 py-2 text-sm text-app-mute hover:text-app-text transition">清空</button>
                            <button
                                class="px-6 py-2 bg-app-primary hover:opacity-90 text-white text-sm font-medium rounded-lg shadow-lg shadow-app-primary/20 transition flex items-center gap-2">
                                <i class="ph-export"></i>
                                <span>解压</span>
                            </button>
                        </div>
                    </div>

                </main>
            </div>

            <!-- Processing Dialog. -->
            <div v-if="processing"
                class="fixed inset-0 bg-black/50 backdrop-blur-sm z-50 flex items-center justify-center">
                <div
                    class="bg-app-sidebar border border-app-border w-96 p-6 rounded-2xl shadow-2xl transition-colors duration-300">
                    <div class="flex justify-between items-center mb-4">
                        <h3 class="font-medium text-app-text">处理中...</h3>
                    </div>
                    <div class="h-2 bg-app-surface rounded-full overflow-hidden mb-2">
                        <div class="h-full bg-app-primary rounded-full w-[85%] animate-pulse"></div>
                    </div>
                </div>
            </div>
        </div>
    </body>
</template>

<script setup lang="ts">
import { onMounted, ref } from 'vue';

let current_view = ref('home');
let is_dark_mode = ref(true);
let drag_over = ref(false);
let files = ref([]);

function close_app() {

}

function toggle_theme() {

}

function handle_drop() {

}

</script>