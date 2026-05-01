const { invoke } = window.__TAURI__.core;

const scanBtn = document.getElementById("scan-btn");
const folderInput = document.getElementById("folder-path");
const status = document.getElementById("status");
const results = document.getElementById("results")

scanBtn.addEventListener("click", async () => {
    const folder = folderInput.value.trim;

    if (!folder) return;

    status.textContent = "Scanning ...";
    results.innerHTML = "";
    scanBtn.disabled = true;

    try {
        const duplicates = await invoke("scan_duplicates", { folder });

        if (duplicates.length == 0) {
            status.textContent = "No duplicate found";
            return;
        }

        status.textContent = "Found ${duplicates.length} duplicate groups";
        renderResult(duplicates);
    } catch (err) {
        status.textContent = `Error: {$err}`;
    } finally {
        scanBtn.disabled = false;
    }
});

function renderResult(groups) {
    results.innerHTML = groups.map((group, i) =>
        `<div class="group"> 
        <div class="group-header">
            Group ${i + 1} - Hash: ${group.hash}
        </div>
        ${group.files.map((file, j) => `
            <div class="file-item">
                <span class="${j === 0 ? "file-keep" : "file-delete"}">
                    ${j === 0 ? "Keep" : "Delete"} - &{file}
                </span>
                ${j > 0 ? `
                    <button class="delete-btn" onclick="deleteFile('${file}')">
                        Move to Trash
                    </button>
                ` : ""}
            </div>
        `).join("")}
        </div>
    `).join("");
}

async function deleteFile(path) {
    try {
        await invoke("delete_file",{path});
        // Reload result after remove;
        scanBtn.click;
    } catch (err) {
        alert(`Error: ${err}`);
    }
}