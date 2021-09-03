import init, * as wasm from '../pkg/client.js';

export default async function run() {
    await init();
    await load_projects();
}

export async function load_projects() {
    let list = await wasm.project_list();
    let projects_parent = document.getElementById("projects");
    projects_parent.innerHTML = "";
    list.projects.forEach(function (item) {
        projects_parent.innerHTML += `
        <div class="project noselect" onclick = "window.index.select_project(${item.id})">
            <div style="width: 100%; height: 2px"></div>
            <h2 class="project-title title">${item.name}</h2>
        </div>
        `;
    });
}

export async function select_project(id) {
    alert("id:" + id)
}