async function dbconnect() {
    return new Promise((resolve, reject) => {
        let request = indexedDB.open("dovecote", 3);
        request.onerror = function(event) {
            reject("failed to create a database. Code: " + event.target.errorCode);
        };
        request.onsuccess = function(event) {
            resolve(event.target.result)
        };
        request.onupgradeneeded = function(event) {
            let db = event.target.result;
            // projects
            if (db.objectStoreNames.contains('projects')) { db.deleteObjectStore("projects"); }
            let tbprojects = db.createObjectStore("projects", { keyPath: "id" });
            tbprojects.createIndex("name", "name", { unique: false });
            tbprojects.createIndex("dialect", "dialect", { unique: false });
            // tree
            if (db.objectStoreNames.contains('tree')) { db.deleteObjectStore("tree"); }
            let tbtree = db.createObjectStore("tree", { keyPath: "project_id" });
            tbtree.createIndex("data", "data", { unique: false });
        }
    });
}

export async function project_list() {
    return dbconnect()
        .then((db) => {
            return new Promise((resolve, reject) => {
                db.transaction("projects").objectStore("projects").getAll().onsuccess = function(event) {
                    event.target.result.sort((a, b) => {
                        return (a.name > b.name) ? +1 : (a.name < b.name ? -1 : 0);
                    })
                    resolve(event.target.result);
                };
                db.transaction("projects").objectStore("projects").getAll().onerror = function(event) {
                    reject(event);
                };
            })
        });
}

async function project_get(project_id) {
    return dbconnect()
        .then((db) => {
            return new Promise((resolve, reject) => {
                let request = db.transaction("projects")
                    .objectStore("projects")
                    .get(project_id);
                request.onsuccess = function(event) {
                    let row = event.target.result;
                    resolve(row ? row : null)
                };
                request.onerror = function(event) {
                    reject(event);
                }
            })
        })
}

export async function create_project(project_name, dialect) {
    return dbconnect()
        .then((db) => {
            return new Promise((resolve, reject) => {
                let request = db.transaction("projects", 'readwrite')
                    .objectStore("projects")
                    .add({
                        id: Math.floor(Math.random() * 100) + "_" + new Date().getTime(),
                        name: project_name,
                        dialect: dialect
                    });
                request.onsuccess = function(event) {
                    let row = event.target.result;
                    resolve()
                };
                request.onerror = function(event) {
                    reject(event);
                }
            });
        });
}

export async function remove_project(id) {
    return dbconnect()
        .then((db) => {
            return new Promise((resolve, reject) => {
                let request = db.transaction(["projects"], "readwrite").objectStore("projects").delete(id);
                request.onsuccess = function(event) {
                    let row = event.target.result;
                    resolve()
                };
                request.onerror = function(event) {
                    reject(event);
                }
            });
        });
}

export async function project_tree(project_id) {
    return dbconnect()
        .then((db) => {
            return new Promise((resolve, reject) => {
                let request = db.transaction("tree")
                    .objectStore("tree")
                    .get(project_id);
                request.onsuccess = function(event) {
                    project_get(project_id)
                        .then(info => {
                            resolve({
                                Dir: [
                                    (info ? info.name : "none"),
                                    (event.target.result && event.target.result.data) ? event.target.result.data : []
                                ]
                            });
                        });
                };
                request.onerror = function(event) {
                    reject(event);
                }
            })
        })
}

async function project_tree_save(project_id, data) {
    return dbconnect().then(db => {
        return new Promise((resolve, reject) => {
            let row = {
                project_id: project_id,
                data: data.Dir[1],
            };
            let request = db
                .transaction("tree", 'readwrite')
                .objectStore("tree")
                .put(row);
            request.onsuccess = function(e) {
                resolve(true)
            };
            request.onerror = function(e) {
                reject(e)
            };
        })
    });
}

async function project_tree_add(project_id, path, object) {
    return project_tree(project_id)
        .then(list => {
            let cursor = list,
                path_parts = path.split("/");

            for (let key in path_parts) {
                if (!cursor.Dir || !cursor.Dir[1]) { break; }
                let finded = cursor.Dir[1].find((element) => {
                    return element.Dir && element.Dir[0] === path_parts[key];
                });
                if (!finded) { break; }
                cursor = finded;
            }
            if (!cursor.Dir[1] || !Array.isArray(cursor.Dir[1])) {
                cursor.Dir[1] = [];
            }

            let duplicate = cursor.Dir[1].find(el => {
                if (object.File) {
                    return el.File && el.File[1] === object.File[1];
                } else {
                    return el.Dir && el.Dir[0] === object.Dir[0];
                }
            });
            if (duplicate) {
                return new Promise((resolve, _) => { resolve(true); });
            }

            cursor.Dir[1].push(object);
            return project_tree_save(project_id, list);
        });
}

export async function create_file(project_id, path, name) {
    return project_tree_add(project_id, path, {
        File: [
            Math.floor(Math.random() * 100) + "_" + new Date().getTime(),
            name
        ]
    });
}

export async function create_directory(project_id, path, name) {
    return project_tree_add(project_id, path, {
        Dir: [
            name, []
        ]
    });
}

export async function rename_file(project_id, file_id, new_name) {
    return project_tree(project_id)
        .then(list => {
            function find(object, file_id) {
                if (object.Dir && object.Dir[1] && Array.isArray(object.Dir[1])) {
                    for (let index in object.Dir[1]) {
                        let result = find(object.Dir[1][index], file_id);
                        if (result) {
                            return Array.isArray(result) ? result : [object, result];
                        }
                    }
                } else if (object.File) {
                    if (object.File[0] == file_id) {
                        return object;
                    }
                }
                return null;
            };
            return new Promise((resolve, reject) => {
                let finded = find(list, file_id);
                if (!finded) {
                    reject("Not found");
                }
                let [parent_dir, file] = finded;

                if (parent_dir.Dir[1].find(el => { return el.File && el.File[1] === new_name; })) {
                    reject("A file with that name already exists");
                }
                file.File[1] = new_name;
                resolve(project_tree_save(project_id, list));
            });
        });
}

export async function rename_directory(project_id, path, old_name, new_name) {
    return project_tree(project_id)
        .then(list => {
            function find(object) {
                if (object.Dir) {
                    if (object.Dir[0] === old_name) {
                        return object;
                    }
                    if (object.Dir[1] && Array.isArray(object.Dir[1])) {
                        for (let index in object.Dir[1]) {
                            let result = find(object.Dir[1][index]);
                            if (result) {
                                return Array.isArray(result) ? result : [object, result];
                            }
                        }
                    }
                }
                return null;
            };
            return new Promise((resolve, reject) => {
                let finded = find(list);
                if (!finded) {
                    reject("Not found");
                }
                let parent_dir, need_dir;
                if (parent.Dir) {
                    parent_dir = null;
                    need_dir = find;
                } else {
                    [parent_dir, need_dir] = finded;
                };

                if (parent_dir && parent_dir.Dir[1].find(el => { return el.Dir && el.Dir[0] === new_name; })) {
                    reject("A file with that name already exists");
                }
                need_dir.Dir[0] = new_name;
                resolve(project_tree_save(project_id, list));
            });
        });
}

export async function remove_file(project_id, file_id) {
    return project_tree(project_id)
        .then(list => {
            function remove(object, file_id) {
                if (object.Dir && object.Dir[1] && Array.isArray(object.Dir[1])) {
                    for (let index in object.Dir[1]) {
                        if (object.Dir[1][index].File) {
                            if (object.Dir[1][index].File[0] === file_id) {
                                object.Dir[1].splice(index, 1);
                                // @todo Удалить содержимое файла
                                return true;
                            }
                            continue;
                        }
                        let result = remove(object.Dir[1][index], file_id);
                        if (result) {
                            return true;
                        }
                    }
                }
                return false;
            };
            remove(list, file_id);
            return project_tree_save(project_id, list);
        });
}

export async function remove_directory(project_id, path) {
    return project_tree(project_id)
        .then(list => {
            function remove(object, path_array) {
                if (!path_array || !Array.isArray(path_array)) { return false; }
                let name = path_array.splice(0, 1);
                if (object.Dir && object.Dir[1] && Array.isArray(object.Dir[1])) {
                    for (let index in object.Dir[1]) {
                        if (object.Dir[1][index].Dir && object.Dir[1][index].Dir[0] == name) {
                            if (!path_array.length) {
                                // @todo Удалить файлы из базы
                                object.Dir[1].splice(index, 1);
                                return true;
                            }
                            return remove(object.Dir[1][index], path_array)
                        }
                    }
                }
                return false;
            };
            remove(list, path.split('/'));
            return project_tree_save(project_id, list);
        });
}