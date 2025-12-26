<template>
    <div
        class="bg-app-bg text-app-text font-sans antialiased overflow-hidden transition-colors duration-300 h-screen w-screen flex flex-col"
    >
        <toast_manager />
        <div v-cloak class="h-full flex flex-col">
            <!-- 1. Title Bar (for Tauri window dragging) -->
            <header
                class="h-10 bg-app-sidebar flex items-center justify-between px-4 border-b border-app-border titlebar-drag-region select-none transition-colors duration-300"
            >
                <div
                    class="flex items-center gap-2 text-sm font-medium text-app-mute"
                >
                    <i class="ph-fill ph-package text-app-primary text-lg"></i>
                    <span>R Extract</span>
                </div>
                <div class="flex items-center gap-3 no-drag">
                    <button
                        @click="minimize"
                        class="w-3 h-3 rounded-full bg-yellow-500 hover:bg-yellow-400 transition"
                    ></button>
                    <button
                        @click="close_app"
                        class="w-3 h-3 rounded-full bg-red-500 hover:bg-red-400 transition"
                    ></button>
                </div>
            </header>

            <!-- Main Layout. -->
            <div class="flex-1 flex overflow-hidden">
                <!-- 2. Sidebar. -->
                <aside
                    class="w-64 bg-app-sidebar flex flex-col border-r border-app-border transition-colors duration-300"
                >
                    <div class="p-4 space-y-1">
                        <div
                            class="flex items-center gap-3 px-3 py-3 rounded-lg cursor-pointer transition-all duration-200 text-sm font-medium"
                            :class="
                                current_view === 'home'
                                    ? 'bg-app-primary text-white shadow-md'
                                    : 'text-app-mute hover:bg-app-surface-hover hover:text-app-text'
                            "
                            @click="current_view = 'home'"
                        >
                            <!-- Home Icon (Lucide SVG) -->
                            <svg
                                xmlns="http://www.w3.org/2000/svg"
                                width="24"
                                height="24"
                                viewBox="0 0 24 24"
                                fill="none"
                                stroke="currentColor"
                                stroke-width="2"
                                stroke-linecap="round"
                                stroke-linejoin="round"
                                class="w-5 h-5"
                            >
                                <path
                                    d="m3 9 9-7 9 7v11a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2z"
                                />
                                <polyline points="9 22 9 12 15 12 15 22" />
                            </svg>

                            <span>首页</span>
                        </div>
                    </div>

                    <!-- Bottom: Settings. -->
                    <div class="mt-auto p-4 border-t border-app-border">
                        <div
                            @click="current_view = 'settings'"
                            class="flex items-center gap-3 px-3 py-3 rounded-lg cursor-pointer transition-all duration-200"
                            :class="
                                current_view === 'settings'
                                    ? 'bg-app-primary text-white shadow-md'
                                    : 'text-app-mute hover:bg-app-surface-hover hover:text-app-text'
                            "
                        >
                            <svg
                                xmlns="http://www.w3.org/2000/svg"
                                width="24"
                                height="24"
                                viewBox="0 0 24 24"
                                fill="none"
                                stroke="currentColor"
                                stroke-width="2"
                                stroke-linecap="round"
                                stroke-linejoin="round"
                                class="w-5 h-5"
                            >
                                <path
                                    d="M12.22 2h-.44a2 2 0 0 0-2 2v.18a2 2 0 0 1-1 1.73l-.43.25a2 2 0 0 1-2 0l-.15-.08a2 2 0 0 0-2.73.73l-.22.38a2 2 0 0 0 .73 2.73l.15.1a2 2 0 0 1 0 2.73l-.15.1a2 2 0 0 0-.73 2.73l.22.38a2 2 0 0 0 2.73.73l.15-.08a2 2 0 0 1 2 0l.43.25a2 2 0 0 1 1 1.73V20a2 2 0 0 0 2 2h.44a2 2 0 0 0 2-2v-.18a2 2 0 0 1 1-1.73l.43-.25a2 2 0 0 1 2 0l.15.08a2 2 0 0 0 2.73-.73l.22-.39a2 2 0 0 0-.73-2.73l-.15-.09a2 2 0 0 1 0-2.73l.15-.1a2 2 0 0 0 .73-2.73l-.22-.38a2 2 0 0 0-2.73-.73l-.15.08a2 2 0 0 1-2 0l-.43-.25a2 2 0 0 1-1-1.73V4a2 2 0 0 0-2-2z"
                                />
                                <circle cx="12" cy="12" r="3" />
                            </svg>

                            <span class="text-sm font-medium">设置</span>
                        </div>
                    </div>
                </aside>

                <!-- 3. Content Area. -->
                <main
                    class="flex-1 flex flex-col bg-app-bg relative transition-colors duration-300"
                >
                    <!-- View Content Switching. -->
                    <!-- A. Settings View. -->
                    <div
                        v-if="current_view === 'settings'"
                        class="p-8 max-w-2xl mx-auto w-full animate-fade-in"
                    >
                        <h2 class="text-2xl font-bold text-app-text mb-6">
                            设置
                        </h2>

                        <div class="space-y-6">
                            <!-- Theme Settings. -->
                            <div
                                class="bg-app-sidebar border border-app-border rounded-xl p-6 transition-colors duration-300"
                            >
                                <div class="flex items-center justify-between">
                                    <div class="flex items-center gap-4">
                                        <div>
                                            <h3
                                                class="font-medium text-app-text"
                                            >
                                                界面主题
                                            </h3>
                                            <p class="text-xs text-app-mute">
                                                在深色模式和浅色模式之间切换
                                            </p>
                                        </div>
                                    </div>

                                    <!-- Toggle Switch. -->
                                    <button
                                        @click="toggle_theme"
                                        class="relative inline-flex h-8 w-14 items-center rounded-full transition-colors focus:outline-none"
                                        :class="
                                            is_dark_mode
                                                ? 'bg-app-primary'
                                                : 'bg-gray-300'
                                        "
                                    >
                                        <span class="sr-only">切换主题</span>
                                        <span
                                            class="h-6 w-6 transform rounded-full bg-white transition-transform duration-200 ease-in-out shadow-sm flex items-center justify-center"
                                            :class="
                                                is_dark_mode
                                                    ? 'translate-x-7'
                                                    : 'translate-x-1'
                                            "
                                        >
                                            <i
                                                v-if="is_dark_mode"
                                                class="ph-moon-stars text-xs text-indigo-500"
                                            ></i>
                                            <i
                                                v-else
                                                class="ph-sun text-xs text-orange-500"
                                            ></i>
                                        </span>
                                    </button>
                                </div>
                            </div>
                        </div>
                    </div>

                    <!-- B. Home / File View. -->
                    <div
                        v-else
                        class="flex-1 overflow-auto relative"
                        id="drop-zone"
                    >
                        <!-- Empty State / Drag-and-Drop Hint. -->
                        <div
                            v-if="files.length === 0"
                            class="absolute inset-0 flex flex-col items-center justify-center transition-all duration-300 m-6 rounded-2xl border-2 border-dashed"
                            :class="
                                drag_over
                                    ? 'border-app-primary bg-app-primary/5'
                                    : 'border-transparent'
                            "
                        >
                            <img
                                src="../assets/images/drag-and-drop-add-new-file-concept-illustration-flat-design-eps10-modern-graphic-element-for-landing-page-empty-state-ui-infographic-icon-vector.png"
                                alt="Cloud Upload"
                                class="w-32 h-32 object-contain mb-6 transition-transform duration-300 drop-shadow-xl"
                                :class="drag_over ? 'scale-110' : ''"
                                draggable="false"
                            />

                            <h2
                                class="text-2xl font-semibold text-app-text mb-2"
                            >
                                拖放文件至此
                            </h2>
                            <p
                                class="text-app-mute text-sm mb-8 text-center max-w-xs"
                            >
                                或点击下方按钮选择文件, 支持 .zip, .rar, .7z
                                等格式
                            </p>

                            <button
                                @click="open_file"
                                class="px-8 py-3 bg-app-primary hover:opacity-90 text-white text-sm font-medium rounded-full transition transform active:scale-95 flex items-center gap-2"
                            >
                                <span>选择文件</span>
                            </button>
                        </div>

                        <!-- File List. -->
                        <div v-else class="p-2">
                            <!-- The space at the top is now occupied by the title bar, so the list follows directly after it. -->
                            <table class="w-full text-left border-collapse">
                                <thead
                                    class="sticky top-0 bg-app-bg text-app-mute text-xs font-medium uppercase tracking-wider z-10 transition-colors duration-300"
                                >
                                    <tr>
                                        <th
                                            class="px-4 py-3 cursor-pointer hover:text-app-text"
                                        >
                                            名称
                                        </th>
                                        <th
                                            class="px-4 py-3 w-32 cursor-pointer hover:text-app-text text-right"
                                        >
                                            大小
                                        </th>
                                        <th
                                            class="px-4 py-3 w-32 cursor-pointer hover:text-app-text"
                                        >
                                            类型
                                        </th>
                                    </tr>
                                </thead>
                                <tbody
                                    class="text-sm divide-y divide-app-border"
                                >
                                    <tr
                                        v-for="file in files"
                                        class="group hover:bg-app-surface transition cursor-default select-none"
                                    >
                                        <td
                                            class="px-4 py-3 flex items-center gap-3"
                                        >
                                            <span
                                                class="text-app-text font-medium truncate max-w-[200px]"
                                                >{{ file.name }}</span
                                            >
                                        </td>
                                        <td
                                            class="px-4 py-3 text-app-mute text-right font-mono text-xs"
                                        >
                                            {{ file.size }}
                                        </td>
                                        <td class="px-4 py-3 text-app-mute">
                                            {{ file.type }}
                                        </td>
                                    </tr>
                                </tbody>
                            </table>
                        </div>
                    </div>

                    <!-- Bottom Action Bar (Visible only when files exist and not on the settings page) -->
                    <div
                        v-if="files.length > 0 && current_view !== 'settings'"
                        class="h-16 border-t border-app-border bg-app-sidebar px-6 flex items-center justify-end transition-colors duration-300"
                    >
                        <div class="flex items-center gap-3">
                            <button
                                @click="files = []"
                                class="px-4 py-2 text-sm text-app-mute hover:text-app-text transition"
                            >
                                清空
                            </button>
                            <button
                                class="px-6 py-2 bg-app-primary hover:opacity-90 text-white text-sm font-medium rounded-lg transition flex items-center gap-2"
                            >
                                <i class="ph-export"></i>
                                <span>解压</span>
                            </button>
                        </div>
                    </div>
                </main>
            </div>

            <!-- Processing Dialog. -->
            <div
                v-if="processing"
                class="fixed inset-0 bg-black/50 backdrop-blur-sm z-50 flex items-center justify-center"
            >
                <div
                    class="bg-app-sidebar border border-app-border w-96 p-6 rounded-2xl shadow-2xl transition-colors duration-300"
                >
                    <div class="flex justify-between items-center mb-4">
                        <h3 class="font-medium text-app-text">处理中...</h3>
                    </div>
                    <div
                        class="h-2 bg-app-surface rounded-full overflow-hidden mb-2"
                    >
                        <div
                            class="h-full bg-app-primary rounded-full w-[85%] animate-pulse"
                        ></div>
                    </div>
                </div>
            </div>
        </div>
    </div>
</template>

<script setup lang="ts">
import { invoke } from "@tauri-apps/api/core";
import { UnlistenFn } from "@tauri-apps/api/event";
import { getCurrentWebview } from "@tauri-apps/api/webview";
import { onMounted, onUnmounted, ref } from "vue";
import toast_manager from "./components/toast_manager.vue";
import { toast } from "../ts/utils/toast.ts";

interface FileItem {
    name: string;
    size: string;
    type: string;
}

let current_view = ref("home");
let is_dark_mode = ref(false);
let drag_over = ref(false);
let files = ref<FileItem[]>([]);
let processing = ref(false);
let rect: DOMRect;
let unlisten: UnlistenFn;
const archive_exts = ["zip", "rar", "7z"];

function close_app() {
    invoke("exit");
}

function toggle_theme() {
    is_dark_mode.value = !is_dark_mode.value;
    document.body.classList.toggle("dark-theme", is_dark_mode.value);
}

function open_file() {
    for (let i = 0; i < 30; i++) {
        files.value.push({
            name: "hello.txt",
            size: "30",
            type: "txt",
        });
    }
}

function minimize() {
    invoke("minimize");
}

onMounted(async () => {
    const drop_zone = document.getElementById("drop-zone");
    if (!drop_zone) return;

    rect = drop_zone.getBoundingClientRect();

    unlisten = await getCurrentWebview().onDragDropEvent(async (event) => {
        if (event.payload.type === "leave") {
            drag_over.value = false;
            return;
        }

        const { x, y } = event.payload.position;
        const inside =
            x >= rect.left &&
            x <= rect.right &&
            y >= rect.top &&
            y <= rect.bottom;

        if (!inside) {
            drag_over.value = false;
            return;
        }

        if (event.payload.type === "over") {
            drag_over.value = true;
            return;
        }
        if (event.payload.type === "drop") {
            drag_over.value = false;
            const path = event.payload.paths[0];
            if (isArchive(path)) {
                files.value.push({
                    name: path.replace(/\\/g, "/").split("/").pop() || "",
                    size: format_size(await get_file_size(path)),
                    type: path.split(".").pop() || "",
                });
            } else {
                toast.warning("未知格式", 1000);
            }
            return;
        }
    });

    function isArchive(path: string): boolean {
        const ext = path.split(".").pop()?.toLowerCase();
        if (!ext) return false;
        return archive_exts.includes(ext);
    }

    function format_size(bytes: number) {
        if (bytes < 1024) return bytes + " B";
        if (bytes < 1024 * 1024) return (bytes / 1024).toFixed(1) + " KB";
        if (bytes < 1024 * 1024 * 1024)
            return (bytes / 1024 / 1024).toFixed(1) + " MB";
        return (bytes / 1024 / 1024 / 1024).toFixed(1) + " GB";
    }

    async function get_file_size(path: string): Promise<number> {
        return await invoke("get_file_size", { path });
    }
});

onUnmounted(() => {
    unlisten();
});
</script>
